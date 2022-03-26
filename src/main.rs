use anyhow::Result;

mod scrapers;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
    let akamai_scraper = scrapers::akamai::AkamaiScraper {};
    println!("{:?}", akamai_scraper.get_status().await?);
    Ok(())
}
