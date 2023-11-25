use std::error::Error;

use actix_web::error;

#[derive(Debug)]
pub enum AppError {
    ParseRssError,
}

impl Error for AppError {}

impl std::fmt::Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AppError::ParseRssError => write!(f, "Parse RSS Error"),
        }
    }
}

impl error::ResponseError for AppError {}
