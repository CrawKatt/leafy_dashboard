use actix_web::ResponseError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum BackEndError {
    #[error("Failed to make request: {0}")]
    RequestError(#[from] reqwest::Error)
}

impl ResponseError for BackEndError {}
