use anyhow::Result;
use axum::extract::Query;
use axum::http::StatusCode;
use axum::{routing::get, Router};
use prometheus::TextEncoder;
use serde::Deserialize;
use tracing::{error, instrument};
use tracing_error::ErrorLayer;
use tracing_subscriber::prelude::*;
use url::Url;

mod scraper;

static _DEFAULT_PORT: u32 = 9919;

#[derive(Deserialize, Debug)]
struct Target {
    target: Url,
}

#[instrument]
async fn work(target: Query<Target>) -> Result<String, StatusCode> {
    let scraper = scraper::Scraper::new(
        // url: "https://payline.statuspage.io/api/v2/summary.json",
        target.target.clone(),
    );

    match scraper.probe().await {
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
    }
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .json()
        .finish()
        .with(ErrorLayer::default())
        .init();

    let app = Router::new().route("/probe", get(work));
    axum::Server::bind(&"0.0.0.0:8000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
}
