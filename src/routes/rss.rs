use actix_web::{
    get, guard, post,
    web::{self, Data, Form, Path},
    Error, HttpResponse, Responder,
};
use serde::Deserialize;
use tokio::sync::Mutex;

use crate::{auth::Credentials, error::AppError, rss::get_item, state::AppState};

fn default_channel() -> String {
    "saved".to_string()
}

#[derive(Deserialize)]
pub struct RssItem {
    #[serde(default = "default_channel")]
    channel: String,
    description: String,
}

#[derive(Deserialize)]
pub struct ShareItem {
    #[serde(default = "default_channel")]
    channel: String,
    title: Option<String>,
    text: Option<String>,
    url: Option<String>,
}

#[get("/{title}")]
pub async fn get_rss(
    app_state: Data<Mutex<AppState>>,
    title: Path<String>,
) -> Result<impl Responder, Error> {
    let mut app_state_acquired = app_state.lock().await;

    let channel = app_state_acquired.get_channel(&title);

    let http_response = HttpResponse::Ok()
        .append_header(("Content-Type", "application/rss+xml; charset=utf-8"))
        .body(
            channel
                .write_to(Vec::new())
                .map_err(|_| AppError::ParseRssError)?,
        );

    Ok(http_response)
}

#[post("/add")]
pub async fn add_rss_post(
    app_state: Data<Mutex<AppState>>,
    Form(mut form): Form<RssItem>,
) -> Result<impl Responder, Error> {
    let item = get_item(None, form.description);

    if form.channel == "" {
        form.channel = default_channel();
    }

    let mut app_state_acquired = app_state.lock().await;

    app_state_acquired.add_item(&form.channel, item);

    Ok(HttpResponse::Ok().finish())
}

#[post("/share_add")]
pub async fn add_rss_share_post(
    app_state: Data<Mutex<AppState>>,
    Form(mut form): Form<ShareItem>,
) -> Result<impl Responder, Error> {
    let description = concatenate_strings(form.text, form.url);

    let item = get_item(form.title, description);

    if form.channel == "" {
        form.channel = default_channel();
    }

    let mut app_state_acquired = app_state.lock().await;

    app_state_acquired.add_item(&form.channel, item);

    Ok(HttpResponse::Ok().finish())
}

pub fn get_rss_scoped_config(
    username: Option<String>,
    password: Option<String>,
) -> impl Fn(&mut web::ServiceConfig) {
    let creds = Credentials::new(username, password);

    move |cfg| {
        cfg.service(
            web::scope("/rss")
                .guard(guard::All(guard::Get()).and(creds.clone()))
                .service(get_rss),
        );
    }
}

fn concatenate_strings(text: Option<String>, url: Option<String>) -> String {
    let mut result = String::new();

    if let Some(desc) = &text {
        result.push_str(desc)
    };

    if text.is_some() && url.is_some() {
        result.push_str("\n\n");
    }

    if let Some(link) = url {
        result.push_str(&link)
    };

    result
}
