use chrono::{DateTime, Local, Utc};
use maud::{html, DOCTYPE};
use reqwest::{header::HeaderMap, Client};
use scraper::{Html, Selector};

#[derive(Debug)]
pub struct DevtoNews {
    title: String,
    link: String,
    date: DateTime<Utc>,
}

impl DevtoNews {
    fn new(title: impl Into<String>, link: impl Into<String>, date: DateTime<Utc>) -> Self {
        Self {
            title: title.into(),
            link: link.into(),
            date,
        }
    }
}

pub fn from_devto_document<'a>(
    title_selector_str: impl Into<&'a str>, // the link is in the title tag
    date_selector_str: impl Into<&'a str>,
    base_url_str: impl Into<&'a str>,
    document: &Html,
) -> Result<Vec<DevtoNews>, Box<dyn std::error::Error + 'a>> {
    let mut news = vec![];
    let title_selector = Selector::parse(title_selector_str.into())?;
    let date_selector = Selector::parse(date_selector_str.into())?;
    let base_url = base_url_str.into();

    for (title, date) in document
        .select(&title_selector)
        .zip(document.select(&date_selector))
    {
        let news_title = title.text().collect::<String>();
        let link = format!("{}{}", base_url, title.attr("href").ok_or("No link")?);
        let datetime = date.attr("datetime").ok_or("No date")?.parse()?;

        news.push(DevtoNews::new(news_title, link, datetime));
    }

    Ok(news)
}

pub fn filter_news_from_today(news: Vec<DevtoNews>) -> Vec<DevtoNews> {
    let mut filtered = vec![];

    for news_data in news {
        let delta = Local::now().to_utc() - news_data.date;

        if delta.num_days() == 0 {
            filtered.push(news_data);
        }
    }

    filtered
}

pub async fn get_devto_news<'a>(
    client: &Client,
    url: impl Into<&'a str> + Copy,
    headers: HeaderMap,
) -> Result<Vec<DevtoNews>, Box<dyn std::error::Error + 'a>> {
    let resp = client
        .get(url.into())
        .headers(headers)
        .send()
        .await?
        .text()
        .await?;

    let document = Html::parse_document(&resp);
    let title_selector = "div.crayons-story > a:nth-child(1)";
    let date_selector = "div.crayons-story > div:nth-child(2) > div:nth-child(1) > div:nth-child(1) > div:nth-child(2) > a:nth-child(2) > time:nth-child(1)";

    from_devto_document(title_selector, date_selector, url, &document)
}

pub fn devto_news_to_html(news: &Vec<DevtoNews>) -> String {
    html! {
        (DOCTYPE)
        html {
            head {
                meta charset="UTF-8";
            }
        }

        body {
            @for item in news {
                h1 { (item.title) }
                p { "See more at: " a href=(item.link) { (item.link) } }
                p {}
            }
        }
    }
    .into()
}
