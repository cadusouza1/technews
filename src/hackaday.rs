use crate::err;
use crate::news::News;
use chrono::{self, NaiveDate, NaiveDateTime, NaiveTime, TimeZone, Utc};
use scraper::{self, Html, Selector};

pub fn from_hackaday_document<'a, S>(
    title_selector_str: S, // the link is in the title tag
    date_selector_str: S,
    date_format: S,
    document: &Html,
) -> Result<Vec<News>, err::ParseError>
where
    S: Into<&'a str>,
{
    let mut news = vec![];
    let title_selector = Selector::parse(title_selector_str.into())
        .map_err(|e| err::ParseError::SelectorParseError(e.to_string()))?;
    let date_selector = Selector::parse(date_selector_str.into())
        .map_err(|e| err::ParseError::SelectorParseError(e.to_string()))?;
    let news_date_format = date_format.into();

    for (title, date) in document
        .select(&title_selector)
        .zip(document.select(&date_selector))
    {
        let news_title: String = title.text().collect();
        let link = title
            .attr("href")
            .ok_or_else(|| err::ParseError::MissingAttribute("News link not found".to_string()))?
            .to_string();

        let date = NaiveDate::parse_from_str(&date.text().collect::<String>(), news_date_format)
            .map_err(|e| err::ParseError::DateParseError(e.to_string()))?;

        let naive_time = NaiveTime::from_hms_opt(0, 0, 0).ok_or_else(|| {
            err::ParseError::TimeParseError("Error parsing naive time".to_string())
        })?;

        news.push(News::new(
            news_title,
            link,
            None,
            Utc.from_utc_datetime(&NaiveDateTime::new(date, naive_time)),
        ));
    }

    Ok(news)
}

pub fn parse_hackaday_document(document: &Html) -> Result<Vec<News>, err::ParseError> {
    let title_selector = ".recent_entries-list > li > div > h2 > a:nth-child(1)";
    let date_selector = ".recent_entries-list > li > div > div > p > span:nth-child(2)";

    from_hackaday_document(title_selector, date_selector, "%B %d, %Y", &document)
}
