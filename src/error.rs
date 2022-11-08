use std::io;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Generic {0}")]
    Generic(String),
    #[error(transparent)]
    Io(#[from] io::Error),
    #[error(transparent)]
    Utf8Error(#[from] std::string::FromUtf8Error)
}