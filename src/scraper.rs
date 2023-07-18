use anyhow::Result;
use chrono::{DateTime, Utc};
use prometheus::{opts, IntGauge, IntGaugeVec, Registry};
use reqwest::Url;
use serde::Deserialize;
use strum::{EnumIter, IntoEnumIterator, IntoStaticStr};
use tracing::error;

#[derive(Debug, Deserialize, EnumIter, IntoStaticStr, PartialEq)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
enum StatusIndicator {
    Maintenance,
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

#[derive(Debug, Deserialize, EnumIter, IntoStaticStr, PartialEq)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
enum ComponentStatus {
    Operational,
    DegradedPerformance,
    PartialOutage,
    MajorOutage,
    UnderMaintenance,
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
                "statuspage_overall",
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

    fn get_overall_timestamp(&self) -> Result<IntGauge> {
        let timestamp = self.page.updated_at.timestamp();
        let metric = IntGauge::new(
            "statuspage_timestamp",
            "Timestamp of last update of the status element",
        )?;
        metric.set(timestamp);
        Ok(metric)
    }

    fn get_component_status(&self) -> Result<IntGaugeVec> {
        let metrics_vec = IntGaugeVec::new(
            opts!(
                "statuspage_component",
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

    fn get_component_timestamp(&self) -> Result<IntGaugeVec> {
        let metrics_vec = IntGaugeVec::new(
            opts!(
                "statuspage_component_timestamp",
                "Last update timestamp of the component"
            ),
            &["component"],
        )?;

        for component in &self.components {
            metrics_vec
                .get_metric_with_label_values(&[&component.name])?
                .set(component.updated_at.timestamp());
        }
        Ok(metrics_vec)
    }
}

pub struct Scraper {
    // pub url: &'static str,
    url: Url,
    registry: Registry,
}

impl Scraper {
    pub fn new(url: Url) -> Self {
        Self {
            url,
            registry: Registry::new(),
        }
    }

    async fn get_status(&self) -> Result<()> {
        let result = reqwest::get(self.url.clone())
            .await?
            .json::<StatusPageResponse>()
            .await?;

        self.registry
            .register(Box::new(result.get_overall_status()?))?;
        self.registry
            .register(Box::new(result.get_overall_timestamp()?))?;
        self.registry
            .register(Box::new(result.get_component_status()?))?;
        self.registry
            .register(Box::new(result.get_component_timestamp()?))?;

        Ok(())
    }

    pub async fn probe(&self) -> Result<&Registry> {
        let success_metric = IntGauge::new(
            "statuspage_probe_success",
            "Whether all queries were successful",
        )?;
        match self.get_status().await {
            Ok(()) => success_metric.set(1),
            Err(err) => error!("Probe failed: {}", err),
        }
        self.registry.register(Box::new(success_metric))?;
        Ok(&self.registry)
    }
}
