use anyhow::Result;
use prometheus::{Encoder, TextEncoder};
// use reqwest::Url;
use axum::extract::Query;
use axum::{extract::Path, routing::get, Router};
use serde::Deserialize;
use tracing_error::ErrorLayer;
use tracing_subscriber::prelude::*;
use url::Url;

mod scraper;

static _DEFAULT_PORT: u32 = 9919;

#[derive(Deserialize)]
struct Target {
    target: Url,
}

async fn work(target: Query<Target>) -> String {
    let scraper = scraper::Scraper {
        // url: "https://payline.statuspage.io/api/v2/summary.json",
        url: target.target.clone(),
    };
    let registry = scraper.get_status().await.unwrap();
    let mut buffer = vec![];
    let encoder = TextEncoder::new();
    let metric_families = registry.gather();
    encoder.encode(&metric_families, &mut buffer).unwrap();
    String::from_utf8(buffer).unwrap()
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
