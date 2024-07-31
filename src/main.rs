mod devto;
mod hackaday;
use reqwest::{header::HeaderMap, Client};
use scraper::{Html, Selector};
use tokio;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    let mut headers = HeaderMap::new();

    headers.insert("authorization", "<authorization>".parse()?);
    headers.insert("user-agent", "CUSTOM_NAME/1.0".parse()?);

    let devto_news =
        devto::get_devto_news(&client, "https://dev.to/t/programming", headers).await?;
    let filtered = devto::filter_news_from_today(devto_news);

    println!("{filtered:#?}");
    Ok(())
}
