pub mod akamai;

use anyhow::Result;
use async_trait::async_trait;
use prometheus::Registry;

#[async_trait]
pub trait Scraper: Send + Sync {
    async fn scrape(&self) -> Result<Registry>;
}
