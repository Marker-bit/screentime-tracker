use std::cmp::Reverse;

use chrono::NaiveDate;
use itertools::Itertools;
use lettre::{
    address::AddressError,
    message::{header::ContentType, Mailbox},
    transport::smtp::authentication::Credentials,
    Message, SmtpTransport, Transport,
};

use crate::tracker::{apps_time_map::AppsTimeMap, env::AppConfig};

/// Struct that represents the content of an email to be sent.
#[derive(Debug)]
pub struct EmailContent {
    subject: String,
    body: String,
}

/// Function that generates email body with total time and apps time
fn generate_email_body(date: NaiveDate, time_mins: u32, apps_time_map: AppsTimeMap) -> String {
    let date_text = date.format("%d.%m.%Y").to_string();
    let apps_info = apps_time_map
        .iter()
        .sorted_by_key(|(_, time)| Reverse(*time))
        .map(|(app_name, time)| {
            let app_time_mins = time / 1000 / 60;
            format!(
                "<b>{}</b>: {} часов {} минут",
                app_name,
                app_time_mins / 60,
                app_time_mins % 60
            )
        });
    let apps_body = apps_info.collect::<Vec<String>>().join("<br />");
    format!(
        "За сегодня, {date_text}, общее время активной работы за компьютером составило: {} часов {} минут.<br /><br />{apps_body}",
        time_mins / 60,
        time_mins % 60
    )
}

/// Function that generates email content to send
pub fn generate_email_content(
    date: NaiveDate,
    time_mins: u32,
    apps_time_map: AppsTimeMap,
) -> EmailContent {
    let date_text = date.format("%d.%m.%Y").to_string();
    EmailContent {
        subject: format!("Отчет об экранном времени за {}", date_text),
        body: generate_email_body(date, time_mins, apps_time_map),
    }
}

/// A simple function to create a mailbox
fn create_mailbox(email: &str) -> Result<Mailbox, String> {
    Ok(Mailbox::new(
        None,
        email.parse().map_err(|o: AddressError| {
            format!(
                "Failed to create a mailbox from {}, err: {}",
                email,
                o.to_string()
            )
        })?,
    ))
}

pub fn send_email(
    app_config: &AppConfig,
    email_content: EmailContent,
    email_to: String,
) -> Result<(), String> {
    let email = Message::builder()
        .from(create_mailbox(&app_config.smtp_user)?)
        .to(create_mailbox(email_to.as_str())?)
        .subject(email_content.subject)
        .header(ContentType::TEXT_HTML)
        .body(email_content.body)
        .map_err(|o| o.to_string())?;

    // Create credentials for authorizing on the SMTP server
    let creds = Credentials::new(app_config.smtp_user.clone(), app_config.smtp_pass.clone());

    // Open a remote connection to SMTP server
    let builder = SmtpTransport::relay(&app_config.smtp_host).map_err(|o| {
        format!(
            "Failed to create an smtp transport with {}, err: {}",
            &app_config.smtp_host,
            o.to_string()
        )
    })?;

    let mailer = builder.credentials(creds).build();

    // Send the email
    match mailer.send(&email) {
        Ok(_) => Ok(()),
        Err(e) => Err(format!("Could not send email: {e:?}")),
    }
}
