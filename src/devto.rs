use crate::news::News;
use reqwest::{header::HeaderMap, Client};
use scraper::{Html, Selector};

pub fn from_devto_document<'a, S>(
    title_selector_str: S, // the link is in the title tag
    date_selector_str: S,
    base_url_str: S,
    document: &Html,
) -> Result<Vec<News>, Box<dyn std::error::Error + 'a>>
where
    S: Into<&'a str>,
{
    let mut news = vec![];
    let title_selector = Selector::parse(title_selector_str.into())?;
    let date_selector = Selector::parse(date_selector_str.into())?;
    let base_url = base_url_str.into();

    for (title, date) in document
        .select(&title_selector)
        .zip(document.select(&date_selector))
    {
        let news_title = title.text().collect::<String>();
        let link = format!(
            "{}{}",
            base_url,
            title
                .attr("href")
                .ok_or(format!("No link found for DevTo news {news_title}"))?
        );
        let datetime = date.attr("datetime").ok_or("No date")?.parse()?;

        news.push(News::new(news_title, link, None, datetime));
    }

    Ok(news)
}

pub async fn get_devto_news<'a, S>(
    client: &Client,
    headers: HeaderMap,
    url: S,
) -> Result<Vec<News>, Box<dyn std::error::Error + 'a>>
where
    S: Into<&'a str>,
{
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

    from_devto_document(title_selector, date_selector, "https://dev.to", &document)
}
