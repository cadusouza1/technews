mod devto;
mod hackaday;
use lettre::{
    message::{header::ContentType, MultiPart, SinglePart},
    transport::smtp::authentication::Credentials,
    Message, SmtpTransport, Transport,
};
use reqwest::{header::HeaderMap, Client};
use tokio;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let smtp_username = std::env::var("SMTP_USERNAME")?;
    let smtp_password = std::env::var("SMTP_PASSWORD")?;
    let smtp_server = std::env::var("SMTP_SERVER")?;

    let client = Client::new();
    let mut headers = HeaderMap::new();

    headers.insert("authorization", "<authorization>".parse()?);
    headers.insert("user-agent", "CUSTOM_NAME/1.0".parse()?);

    let hackaday_news =
        hackaday::get_hackaday_news(&client, headers.clone(), "https://hackaday.com/").await?;

    let filtered_hackaday_news = hackaday::filter_news_from_today(hackaday_news);
    let hackaday_page = hackaday::hackaday_news_to_html(&filtered_hackaday_news);

    let devto_news = devto::get_devto_news(
        &client,
        "https://dev.to",
        "https://dev.to/t/programming",
        headers,
    )
    .await?;
    let filtered_devto_news = devto::filter_news_from_today(devto_news);
    let devto_page = devto::devto_news_to_html(&filtered_devto_news);

    let email = Message::builder()
        .from(smtp_username.parse()?)
        .to(smtp_username.parse()?)
        .subject("Tech news from the past 24h")
        .multipart(
            MultiPart::mixed()
                .singlepart(
                    SinglePart::builder()
                        .header(ContentType::TEXT_HTML)
                        .body(hackaday_page),
                )
                .singlepart(
                    SinglePart::builder()
                        .header(ContentType::TEXT_HTML)
                        .body(devto_page),
                ),
        )?;

    let mailer = SmtpTransport::starttls_relay(&smtp_server)?
        .credentials(Credentials::new(smtp_username, smtp_password))
        .build();

    mailer.send(&email)?;
    Ok(())
}
