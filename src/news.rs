use chrono::{DateTime, Local, TimeDelta, Utc};
use maud::{html, DOCTYPE};
use reqwest::{header::HeaderMap, Client};

#[derive(Debug, Clone)]
pub struct News {
    title: String,
    link: String,
    description: Option<String>,
    date: DateTime<Utc>,
}

impl News {
    pub fn new(
        title: impl Into<String>,
        link: impl Into<String>,
        description: impl Into<Option<String>>,
        date: DateTime<Utc>,
    ) -> Self {
        Self {
            title: title.into(),
            link: link.into(),
            description: description.into(),
            date,
        }
    }
}

pub fn filter_news_from_delta(news: Vec<News>, delta: TimeDelta) -> Vec<News> {
    news.into_iter()
        .filter(|n| {
            let news_delta = Local::now().to_utc() - n.date;
            news_delta <= delta
        })
        .collect()
}

pub async fn fetch_raw_news<'a, S>(
    client: Client,
    headers: HeaderMap,
    url: S,
) -> Result<(S, String), reqwest::Error>
where
    S: Into<&'a str> + Clone,
{
    Ok((
        url.clone(),
        client
            .get(url.into())
            .headers(headers)
            .send()
            .await?
            .text()
            .await?,
    ))
}

pub fn news_to_html(news: &Vec<News>) -> String {
    html! {
        (DOCTYPE)
        html {
            head {
                meta charset="UTF-8";
            }
        }

        body {
            h1 { center {"Tech News" } }
            @for item in news {
                h2 { center { (item.title) } }
                @if let Some(desc) = item.description.clone() {
                    p { (desc) }
                }
                p { "See more at: " a href=(item.link) { (item.link) } }
                br;
            }
        }
    }
    .into()
}
