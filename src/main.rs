mod devto;
mod hackaday;
mod news;
use std::collections::HashMap;

use chrono::TimeDelta;
use lettre::{
    message::{header::ContentType, SinglePart},
    transport::smtp::authentication::Credentials,
    Message, SmtpTransport, Transport,
};
use reqwest::{header::HeaderMap, Client};
use scraper::Html;
use tokio::{self, task::JoinSet};

type ParseFnType =
    Box<dyn Fn(&Html) -> std::result::Result<Vec<news::News>, Box<dyn std::error::Error>>>;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let smtp_username = std::env::var("SMTP_USERNAME")?;
    let smtp_password = std::env::var("SMTP_PASSWORD")?;
    let smtp_server = std::env::var("SMTP_SERVER")?;
    let delta = TimeDelta::days(1);

    let client = Client::new();
    let mut headers = HeaderMap::new();
    let mut set = JoinSet::new();

    let mut url_map: HashMap<&str, ParseFnType> = HashMap::new();
    url_map.insert(
        "https://hackaday.com/",
        Box::new(|doc| hackaday::parse_hackaday_document(doc)),
    );

    url_map.insert(
        "https://dev.to/t/programming",
        Box::new(|doc| devto::parse_devto_document(doc)),
    );

    headers.insert("authorization", "<authorization>".parse()?);
    headers.insert("user-agent", "CUSTOM_NAME/1.0".parse()?);

    let _ = url_map
        .keys()
        .map(|url| {
            println!("Fetching {url}");
            news::fetch_raw_news(client.clone(), headers.clone(), *url)
        })
        .map(|task| set.spawn(task))
        .collect::<Vec<_>>();

    let mut pages = vec![];
    while let Some(res) = set.join_next().await {
        let out = res??;
        println!("Finished fetching {}", out.0);
        pages.push((out.0, Html::parse_document(&out.1)));
    }

    let mut parsed_pages = vec![];
    for page in pages.iter() {
        let parse_func = url_map
            .get(page.0)
            .ok_or(format!("Could not get parse_func for {}", page.0))?;

        parsed_pages.push(parse_func(&page.1)?);
    }

    let filtered_pages = parsed_pages
        .iter()
        .map(|n| news::filter_news_from_delta(n.to_vec(), delta))
        .collect::<Vec<_>>();

    let html_page = filtered_pages
        .iter()
        .map(news::news_to_html)
        .collect::<String>();

    let email = Message::builder()
        .from(smtp_username.parse()?)
        .to(smtp_username.parse()?)
        .subject("Tech news from the past 24h")
        .singlepart(
            SinglePart::builder()
                .header(ContentType::TEXT_HTML)
                .body(html_page),
        )?;

    let mailer = SmtpTransport::starttls_relay(&smtp_server)?
        .credentials(Credentials::new(smtp_username, smtp_password))
        .build();

    mailer.send(&email)?;
    Ok(())
}
