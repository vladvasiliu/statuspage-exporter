use anyhow::Result;

mod scrapers;

static _DEFAULT_PORT: u32 = 9919;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
    let scraper = scrapers::Scraper {
        url: "https://payline.statuspage.io/api/v2/summary.json",
    };
    scraper.get_status().await?;
    Ok(())
}
