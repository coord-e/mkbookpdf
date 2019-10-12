use lopdf;
use std::{error, fmt, io, path, process};

#[derive(Debug)]
pub enum Error {
    IO(io::Error),
    PDF(lopdf::Error),
    Print(process::ExitStatus),
    InvaildPath(path::PathBuf),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::IO(e) => write!(f, "IO Error: {}", e),
            Error::PDF(e) => write!(f, "PDF Error: {}", e),
            Error::Print(code) => write!(f, "Print command returned non-zero exit code: {}", code),
            Error::InvaildPath(path) => write!(f, "Unsupported path string: {}", path.display()),
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
