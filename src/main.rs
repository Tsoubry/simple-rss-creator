use actix_files::Files;
use actix_web::{middleware, web::Data, App, HttpServer};
use log::info;

mod auth;
mod error;
mod routes;
mod rss;
mod settings;
mod state;

use crate::{
    routes::{
        health::health,
        rss::{add_rss_post, add_rss_share_post, get_rss_scoped_config},
    },
    settings::{get_settings, Settings},
    state::AppState,
};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let settings = get_settings();
    let Settings {
        service_host,
        service_port,
        workers,
        auth_username,
        auth_password,
        ..
    } = settings.clone();

    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let app_state = AppState::create_state_mutex();

    info!("Server starting on {}:{}", &service_host, &service_port);

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(settings.clone()))
            .app_data(app_state.clone())
            .wrap(middleware::Logger::default())
            .service(health)
            .configure(get_rss_scoped_config(
                auth_username.clone(),
                auth_password.clone(),
            ))
            .service(add_rss_post)
            .service(add_rss_share_post)
            .service(Files::new("/", "./static-files").index_file("index.html"))
    })
    .workers(workers)
    .bind((service_host, service_port))?
    .run()
    .await
}
