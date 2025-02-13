use std::io;

use derive_more::{Display, Error, From};
use tokio::task::JoinError;

#[derive(Error, Display, Debug)]
pub struct ErrorStatusCode;

#[derive(From, Error, Display, Debug)]
pub enum RequestError {
    Hyper(hyper::Error),
    Http(hyper::http::Error),
    ErrorStatusCode(ErrorStatusCode),
}

#[derive(Error, Display, Debug)]
pub struct TimeoutWaitingForSocket;

#[derive(Error, Display, Debug)]
/// The path is not valid unicode and cannot be used
pub struct InvalidPath;

#[derive(From, Error, Display, Debug)]
pub enum SpawnError {
    IO(io::Error),
    Request(RequestError),
    Timeout(TimeoutWaitingForSocket),
    InvalidPath(InvalidPath),
    Join(JoinError),
}

#[derive(From, Error, Display, Debug)]
pub enum ShutdownError {
    IO(io::Error),
    Join(JoinError),
}
