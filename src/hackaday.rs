use chrono::{self, Local, NaiveDate};
use maud::{html, DOCTYPE};
use scraper::{self, Html, Selector};

#[derive(Debug)]
pub struct HackadayNews {
    title: String,
    link: String,
    date: NaiveDate,
}

impl HackadayNews {
    fn new(title: String, link: String, date: NaiveDate) -> Self {
        Self { title, link, date }
    }
}

pub fn from_hackaday_document<'a>(
    title_selector_str: impl Into<&'a str>, // the link is in the title tag
    date_selector_str: impl Into<&'a str>,
    date_format: impl Into<&'a str>,
    document: &Html,
) -> Result<Vec<HackadayNews>, Box<dyn std::error::Error + 'a>> {
    let mut news = vec![];
    let title_selector = Selector::parse(title_selector_str.into())?;
    let date_selector = Selector::parse(date_selector_str.into())?;
    let news_date_format = date_format.into();

    for (title, date) in document
        .select(&title_selector)
        .zip(document.select(&date_selector))
    {
        let news_title = title.text().collect::<String>();
        let link = title.attr("href").ok_or("News link not found")?.to_string();
        let date = NaiveDate::parse_from_str(&date.text().collect::<String>(), news_date_format)?;

        news.push(HackadayNews::new(news_title, link, date));
    }

    Ok(news)
}

pub fn filter_news_from_today(news: Vec<HackadayNews>) -> Vec<HackadayNews> {
    let mut filtered = vec![];

    for news_data in news {
        let delta = Local::now().date_naive() - news_data.date;

        if delta.num_days() <= 1 {
            filtered.push(news_data);
        }
    }

    filtered
}

pub async fn get_hackaday_news<'a>(
    url: &'a str,
) -> Result<Vec<HackadayNews>, Box<dyn std::error::Error + 'a>> {
    let resp = reqwest::get(url).await?.text().await?;
    let document = scraper::Html::parse_document(&resp);
    let title_selector = ".recent_entries-list > li > div > h2 > a:nth-child(1)";
    let date_selector = ".recent_entries-list > li > div > div > p > span:nth-child(2)";

    from_hackaday_document(title_selector, date_selector, "%B %d, %Y", &document)
}

pub fn hackaday_news_to_html(news: &Vec<HackadayNews>) -> String {
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
