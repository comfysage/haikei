use std::fmt;
use std::io;

#[macro_export]
macro_rules! make_err {
    () => {
        Error::default()
    };
    ($t:ident, $v:literal) => {
        Error::$t(format!($v))
    };
}

#[derive(Debug, Default)]
pub enum Error {
  NotFound(String),
  Missing(String),
  // io object already exists, multiple packages with similar names
  Conflict(String),
  IO(String),
  Data(String),
  Regex(String),
  Parse(String),
  #[default]
  Unexpected,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let msg = match self {
            Error::NotFound(s) => format!("Not Found: {s}"),
            Error::Missing(s) => format!("Missing: {s}"),
            Error::Conflict(s) => format!("Conflict: {s}"),
            Error::IO(s) => format!("IO: {s}"),
            Error::Data(s) => format!("Data: {s}"),
            Error::Regex(s) => format!("Regex: {s}"),
            Error::Parse(s) => format!("Parse: {s}"),
            Error::Unexpected => format!("unexpected error"),
        };
        write!(f, "{}", msg)
    }
}

impl From<io::Error> for Error {
    fn from(value: io::Error) -> Self {
        Self::IO(format!("io error {}", value.raw_os_error().unwrap()))
    }
}

impl From<regex::Error> for Error {
    fn from(value: regex::Error) -> Self {
        Self::Regex(format!("regex error {}", value.to_string()))
    }
}

impl From<nom::Err<nom::error::Error<&str>>> for Error {
    fn from(value: nom::Err<nom::error::Error<&str>>) -> Self {
        Self::Parse(format!("parse error {}", value.to_string()))
    }
}

impl From<std::str::Utf8Error> for Error {
    fn from(value: std::str::Utf8Error) -> Self {
        Self::Data(format!("utf8 error {}", value.to_string()))
    }
}
