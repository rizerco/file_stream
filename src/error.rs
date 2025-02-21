use std::{io, result, string::FromUtf8Error};
use thiserror::Error;

#[derive(Error, Debug)]
/// An error that can occur in the file stream.
pub enum Error {
    #[error("An IO error occurred.")]
    Io(#[from] io::Error),
    #[error("A UTF8 decode error occurred.")]
    Utf8(#[from] FromUtf8Error),
}

/// A result type for the file stream.
pub type Result<T> = result::Result<T, Error>;
