use actix_web::http::header;
use base64::{engine::general_purpose, Engine as _};

// Basic auth credentials
#[derive(Debug, Clone)]
pub struct Credentials {
    username: Option<String>,
    password: Option<String>,
}

impl Credentials {
    pub fn new(username: Option<String>, password: Option<String>) -> Self {
        Credentials { username, password }
    }
}

use actix_web::guard::{Guard, GuardContext};

impl Guard for Credentials {
    fn check(&self, ctx: &GuardContext<'_>) -> bool {
        let mut result = false;

        match (&self.username, &self.password) {
            (Some(username), Some(password)) => {
                if let Some(auth_header) = ctx.head().headers.get(header::AUTHORIZATION) {
                    if let Ok(auth_str) = auth_header.to_str() {
                        if auth_str.starts_with("Basic ") {
                            let encoded = auth_str.trim_start_matches("Basic ");
                            if let Ok(decoded) = general_purpose::STANDARD.decode(encoded) {
                                if let Ok(credentials) = String::from_utf8(decoded) {
                                    let mut parts = credentials.splitn(2, ':');
                                    if let (Some(given_user), Some(given_pass)) =
                                        (parts.next(), parts.next())
                                    {
                                        result = username == given_user && password == given_pass
                                    }
                                }
                            }
                        }
                    }
                }
            }

            _ => {
                result = true;
            }
        }

        if !result {
            log::warn!("User not authorized!")
        }

        result
    }
}
