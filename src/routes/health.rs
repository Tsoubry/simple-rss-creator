use actix_web::{get, Error, HttpResponse};

#[get("/health")]
pub async fn health() -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok().finish())
}
