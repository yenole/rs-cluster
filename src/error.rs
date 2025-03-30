use std::fmt::{self, Display, Formatter};

#[derive(Debug)]
pub enum Error {
    E(String),
}

impl Display for Error {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> Result<(), fmt::Error> {
        match self {
            Error::E(v) => write!(fmt, "{}", v),
        }
    }
}

impl From<&str> for crate::Error {
    fn from(value: &str) -> Self {
        Self::E(value.to_string())
    }
}

impl From<std::io::Error> for crate::Error {
    fn from(value: std::io::Error) -> Self {
        Self::E(value.to_string())
    }
}

impl From<String> for crate::Error {
    fn from(value: String) -> Self {
        Self::E(value)
    }
}
