use anyhow::Result;
use prometheus::{Encoder, TextEncoder};

mod scraper;

static _DEFAULT_PORT: u32 = 9919;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
    let scraper = scraper::Scraper {
        url: "https://payline.statuspage.io/api/v2/summary.json",
    };
    let registry = scraper.get_status().await?;
    let mut buffer = vec![];
    let encoder = TextEncoder::new();
    let metric_families = registry.gather();
    encoder.encode(&metric_families, &mut buffer).unwrap();

    // Output to the standard output.
    println!("{}", String::from_utf8(buffer).unwrap());
    Ok(())
}
