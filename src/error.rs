use lopdf;
use std::{error, fmt, io, path, process, result};

pub type Result<T> = result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    IO(io::Error),
    PDF(lopdf::Error),
    Print(String, process::ExitStatus),
    InvalidPath(path::PathBuf),
    LPNotFound(String),
    Cancelled,
    EmptyPDF,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::IO(e) => write!(f, "{}", e),
            Error::PDF(e) => write!(f, "error processing PDF file: {}", e),
            Error::Print(cmd, code) => write!(f, "`{}` returned non-zero {}", cmd, code),
            Error::InvalidPath(path) => write!(f, "unsupported path string: {}", path.display()),
            Error::LPNotFound(cmd) => write!(f, "`{}` could not be found", cmd),
            Error::Cancelled => write!(f, "cancelled by user"),
            Error::EmptyPDF => write!(f, "input PDF must have at least one page in it"),
        }
    }
}

impl error::Error for Error {}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        Error::IO(err)
    }
}

impl From<lopdf::Error> for Error {
    fn from(err: lopdf::Error) -> Self {
        match err {
            lopdf::Error::IO(e) => Error::IO(e),
            _ => Error::PDF(err),
        }
    }
}
