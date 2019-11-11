use crate::repository::Error as RepositoryError;
use failure::{Backtrace, Context, Fail};
use std::fmt;
use std::fmt::Display;
use std::io::Error as IOError;
use std::option::NoneError;

#[derive(Clone, Eq, PartialEq, Debug, Fail)]
pub enum ErrorKind {
    #[fail(display = "IO error: {}", message)]
    IO { message: String },
    #[fail(display = "Internal error: {}", message)]
    Internal { message: String },
    #[fail(display = "Unknown command: {}", command_name)]
    Unknown { command_name: String },
}

#[derive(Debug)]
pub struct Error {
    inner: Context<ErrorKind>,
}

impl Fail for Error {
    fn cause(&self) -> Option<&dyn Fail> {
        self.inner.cause()
    }

    fn backtrace(&self) -> Option<&Backtrace> {
        self.inner.backtrace()
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        Display::fmt(&self.inner, f)
    }
}

impl From<ErrorKind> for Error {
    fn from(kind: ErrorKind) -> Error {
        Error {
            inner: Context::new(kind),
        }
    }
}

impl From<Context<ErrorKind>> for Error {
    fn from(inner: Context<ErrorKind>) -> Error {
        Error { inner }
    }
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

impl From<RepositoryError> for Error {
    fn from(error: RepositoryError) -> Error {
        Error {
            inner: Context::new(ErrorKind::Internal {
                message: error.inner.get_context().to_string(),
            }),
        }
    }
}
