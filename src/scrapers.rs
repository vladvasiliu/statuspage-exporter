use anyhow::Result;
use chrono::{DateTime, Utc};
use prometheus::Registry;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
enum StatusIndicator {
    Critical,
    Major,
    Minor,
    None,
}

#[derive(Debug, Deserialize)]
pub struct Status {
    updated_at: DateTime<Utc>,
    indicator: StatusIndicator,
}

#[derive(Debug, Deserialize)]
struct StatusPageResponsePage {
    updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
struct StatusPageResponseStatus {
    indicator: StatusIndicator,
    // description: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
enum ComponentStatus {
    Operational,
    DegradedPerformance,
    PartialOutage,
    MajorOutage,
}

#[derive(Debug, Deserialize)]
struct StatusPageComponent {
    name: String,
    status: ComponentStatus,
    updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
struct StatusPageResponse {
    page: StatusPageResponsePage,
    status: StatusPageResponseStatus,
    components: Vec<StatusPageComponent>,
}

pub struct Scraper {
    pub url: &'static str,
}

impl Scraper {
    pub async fn get_status(self) -> Result<()> {
        let result = reqwest::get(self.url)
            .await?
            .json::<StatusPageResponse>()
            .await?;
        println!("{:?}", result);
        Ok(())
    }
}
