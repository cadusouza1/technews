extern crate lettre;

mod hackaday;
use lettre::{
    message::{header::ContentType, SinglePart},
    transport::smtp::authentication::Credentials,
    Message, SmtpTransport, Transport,
};
use tokio;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let smtp_username = std::env::var("SMTP_USERNAME")?;
    let smtp_password = std::env::var("SMTP_PASSWORD")?;
    let smtp_server = std::env::var("SMTP_SERVER")?;

    let hackaday_news = hackaday::get_hackaday_news("https://hackaday.com/").await?;
    let filtered_news = hackaday::filter_news_from_today(hackaday_news);
    let page = hackaday::hackaday_news_to_html(&filtered_news);

    let email = Message::builder()
        .from(smtp_username.parse()?)
        .to(smtp_username.parse()?)
        .subject("Tech news from the past 24h")
        .singlepart(
            SinglePart::builder()
                .header(ContentType::TEXT_HTML)
                .body(page.to_string()),
        )?;

    let mailer = SmtpTransport::starttls_relay(&smtp_server)?
        .credentials(Credentials::new(smtp_username, smtp_password))
        .build();

    mailer.send(&email)?;
    println!("{page:#?}");
    Ok(())
}
