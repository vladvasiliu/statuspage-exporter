//! Akamai StatusPage scraper based on https://www.akamaistatus.com/api

use super::Scraper;
use anyhow::Result;
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use prometheus::Registry;
use serde::Deserialize;

static STATUS_URL: &str = "https://www.akamaistatus.com/api/v2/status.json";

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
enum AkamaiStatusIndicator {
    Critical,
    Major,
    Minor,
    None,
}

#[derive(Debug, Deserialize)]
pub struct AkamaiStatus {
    updated_at: DateTime<Utc>,
    indicator: AkamaiStatusIndicator,
}

#[derive(Debug, Deserialize)]
struct StatusPageResponsePage {
    updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
struct StatusPageResponseStatus {
    indicator: AkamaiStatusIndicator,
    description: String,
}

#[derive(Debug, Deserialize)]
struct StatusPageResponse {
    page: StatusPageResponsePage,
    status: StatusPageResponseStatus,
}

pub struct AkamaiScraper {}

impl AkamaiScraper {
    pub async fn get_status(self) -> Result<AkamaiStatus> {
        let response = reqwest::get(STATUS_URL)
            .await?
            .json::<StatusPageResponse>()
            .await?;

        Ok(AkamaiStatus {
            updated_at: response.page.updated_at,
            indicator: response.status.indicator,
        })
    }
}

#[async_trait]
impl Scraper for AkamaiScraper {
    async fn scrape(&self) -> anyhow::Result<Registry> {
        todo!()
    }
}
