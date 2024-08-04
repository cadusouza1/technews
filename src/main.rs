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

use crate::news::News;

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
        let (url, doc) = res??;
        println!("Finished fetching {}", url);
        pages.push((url, Html::parse_document(&doc)));
    }

    // Maybe putting it all in a functional way was not the way
    let parsed_pages = pages
        .iter()
        .map(|page| {
            let parse_func = url_map
                .get(page.0)
                .ok_or(format!("Could not get parse_func for {}", page.0))
                .unwrap();

            println!("Parsing {}", page.0);
            (page.0, parse_func(&page.1))
        })
        .map(|(url, n)| (url, n.into_iter().flatten().collect()))
        .collect::<Vec<(&str, Vec<News>)>>();

    let filtered_pages = parsed_pages
        .iter()
        .map(|(url, n)| {
            println!("Filtering {}", url);
            (*url, news::filter_news_from_delta(n.to_vec(), delta))
        })
        .collect::<Vec<(&str, Vec<News>)>>();

    let html_page = filtered_pages
        .iter()
        .map(|(url, n)| {
            println!("Converting to HTML {}", url);
            news::news_to_html(&n)
        })
        .collect::<String>();

    println!("Building email");
    let email = Message::builder()
        .from(smtp_username.parse()?)
        .to(smtp_username.parse()?)
        .subject("Tech news from the past 24h")
        .singlepart(
            SinglePart::builder()
                .header(ContentType::TEXT_HTML)
                .body(html_page),
        )?;

    println!("Building mailer");
    let mailer = SmtpTransport::starttls_relay(&smtp_server)?
        .credentials(Credentials::new(smtp_username, smtp_password))
        .build();

    println!("Sending email");
    mailer.send(&email)?;
    println!("Email sent");
    Ok(())
}
