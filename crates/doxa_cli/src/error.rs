use derive_more::{Display, Error, From};
use reqwest::StatusCode;

#[derive(Error, Display, From, Debug)]
pub enum CommandError {
    #[display(fmt = "{}", _0)]
    Request(RequestError),
    #[display(fmt = "io error: {}", _0)]
    IO(std::io::Error),
}

#[derive(Error, Display, From, Debug)]
pub enum RequestError {
    #[display(fmt = "server returned error: {}", _0)]
    Doxa(DoxaError),
    #[display(fmt = "server returned improperly formatted error: {}", _0)]
    Plain(PlainError),
    #[display(fmt = "failed to make request: {}", _0)]
    Request(reqwest::Error),
    #[display(fmt = "failed to parse response: {}", _0)]
    Json(serde_json::Error),
}

#[derive(Display, Error, Debug, Clone)]
#[display(
    fmt = "{} ({}) {}",
    error_code,
    status_code,
    "message.as_ref().map(|s| s.as_str()).unwrap_or(\"\")"
)]
pub struct DoxaError {
    pub error_code: String,
    pub status_code: StatusCode,
    pub message: Option<String>,
}

#[derive(Display, Error, Debug, Clone)]
#[display(fmt = "`{}` ({})", error_message, status_code)]
pub struct PlainError {
    pub status_code: StatusCode,
    pub error_message: String,
}
