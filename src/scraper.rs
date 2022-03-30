use anyhow::Result;
use chrono::{DateTime, Utc};
use prometheus::{opts, IntGaugeVec, Registry};
use serde::Deserialize;
use strum::{EnumIter, EnumString, IntoEnumIterator, IntoStaticStr};

#[derive(Debug, Deserialize, EnumIter, EnumString, IntoStaticStr, PartialEq)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
enum StatusIndicator {
    Critical,
    Major,
    Minor,
    None,
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

#[derive(Debug, Deserialize, EnumIter, EnumString, IntoStaticStr, PartialEq)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
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

impl StatusPageResponse {
    fn get_overall_status(&self) -> Result<IntGaugeVec> {
        let metrics_vec = IntGaugeVec::new(
            opts!(
                "status_page_overall",
                "Overall status of this service, from the status element"
            ),
            &["indicator"],
        )?;

        for indicator_value in StatusIndicator::iter() {
            let gauge_value = if self.status.indicator == indicator_value {
                1
            } else {
                0
            };
            metrics_vec
                .get_metric_with_label_values(&[indicator_value.into()])?
                .set(gauge_value);
        }
        Ok(metrics_vec)
    }

    fn get_component_status(&self) -> Result<IntGaugeVec> {
        let metrics_vec = IntGaugeVec::new(
            opts!(
                "status_page_component",
                "Per component status of this service, from the components element"
            ),
            &["component", "status"],
        )?;

        for component in &self.components {
            for status_value in ComponentStatus::iter() {
                let gauge_value = if component.status == status_value {
                    1
                } else {
                    0
                };
                metrics_vec
                    .get_metric_with_label_values(&[&component.name, status_value.into()])?
                    .set(gauge_value);
            }
        }

        Ok(metrics_vec)
    }
}

pub struct Scraper {
    pub url: &'static str,
}

impl Scraper {
    pub async fn get_status(self) -> Result<Registry> {
        let result = reqwest::get(self.url)
            .await?
            .json::<StatusPageResponse>()
            .await?;

        let registry = Registry::new();
        registry.register(Box::new(result.get_component_status()?))?;
        registry.register(Box::new(result.get_overall_status()?))?;

        Ok(registry)
    }
}
