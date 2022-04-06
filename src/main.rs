use anyhow::Result;
use axum::extract::Query;
use axum::http::StatusCode;
use axum::{routing::get, Router};
use lazy_static::lazy_static;
use prometheus::{opts, register_int_counter_vec, IntCounterVec, TextEncoder, gather, register_int_gauge_vec};
use serde::Deserialize;
use std::env;
use std::net::SocketAddr;
use tracing::{error, info, instrument};
use tracing_error::ErrorLayer;
use tracing_subscriber::prelude::*;
use url::Url;

mod scraper;

static DEFAULT_BIND: &str = "127.0.0.1:9925";

lazy_static! {
    static ref PROBES_TOTAL: IntCounterVec = register_int_counter_vec!(
        opts!(
            "probes_total",
            "Total number of handled probes, by returned request code"
        ),
        &["code"]
    )
    .expect("Failed to register probes_total metric");
}

#[derive(Deserialize, Debug)]
struct Target {
    target: Url,
}

#[instrument]
async fn probe(target: Query<Target>) -> Result<String, StatusCode> {
    let scraper = scraper::Scraper::new(target.target.clone());

    let result = match scraper.probe().await {
        Ok(registry) => {
            let encoder = TextEncoder::new();
            let metric_families = registry.gather();
            match encoder.encode_to_string(&metric_families) {
                Ok(s) => Ok(s),
                Err(err) => {
                    error!("Failed to encode metrics: {}", err);
                    Err(StatusCode::INTERNAL_SERVER_ERROR)
                }
            }
        }
        Err(err) => {
            error!("Handling probe failed: {}", err);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    };

    let code = match result {
        Ok(_) => 200,
        Err(code) => code.as_u16(),
    };

    PROBES_TOTAL
        .get_metric_with_label_values(&[&code.to_string()])
        .expect("Failed to retrieve probes_total metric")
        .inc();

    result
}

#[instrument]
async fn metrics() -> Result<String, StatusCode> {
    let encoder = TextEncoder::new();
    let metric_families = gather();
    match encoder.encode_to_string(&metric_families) {
        Ok(s) => Ok(s),
        Err(err) => {
            error!("Failed to encode metrics: {}", err);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .json()
        .finish()
        .with(ErrorLayer::default())
        .init();

    let bind_addr = env::var("STATUSPAGE_EXPORTER_LISTEN")
        .unwrap_or_else(|_| DEFAULT_BIND.to_string())
        .parse::<SocketAddr>()?;

    info!("Listening on {}", bind_addr);

    let version = env!("CARGO_PKG_VERSION");
    register_int_gauge_vec!(opts!("statuspage_info", "statuspage exporter version information"), &["version"])?.get_metric_with_label_values(&[version]).expect("Failed to get info metric").set(1);


    let app = Router::new()
        .route("/probe", get(probe))
        .route("/metrics", get(metrics));
    axum::Server::bind(&bind_addr)
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
}
