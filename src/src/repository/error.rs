use failure::{Context, Fail};
use std::io::Error as IOError;
use std::option::NoneError;
use std::path::StripPrefixError;

#[derive(Fail, Debug)]
pub enum ErrorKind {
    #[fail(display = "IO error: {}", message)]
    IO { message: String },
    #[fail(display = "Internal error: {}", message)]
    Internal { message: String },
    #[fail(display = "Already exists: {}", path)]
    AlreadyExists { path: String },
}

#[derive(Debug)]
pub struct Error {
    pub inner: Context<ErrorKind>,
}

impl From<IOError> for Error {
    fn from(error: IOError) -> Error {
        Error {
            inner: Context::new(ErrorKind::IO {
                message: error.to_string(),
            }),
        }
    }
}

impl From<NoneError> for Error {
    fn from(_error: NoneError) -> Error {
        Error {
            inner: Context::new(ErrorKind::Internal {
                message: String::from("NoneError"),
            }),
        }
    }
}

impl From<ErrorKind> for Error {
    fn from(error: ErrorKind) -> Error {
        Error {
            inner: Context::new(error),
        }
    }
}

impl From<StripPrefixError> for Error {
    fn from(error: StripPrefixError) -> Error {
        Error {
            inner: Context::new(ErrorKind::IO {
                message: error.to_string(),
            }),
        }
    }
}
