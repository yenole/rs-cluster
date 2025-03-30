use reqwest::header::InvalidHeaderValue;

impl From<InvalidHeaderValue> for crate::Error {
    fn from(value: InvalidHeaderValue) -> Self {
        Self::E(value.to_string())
    }
}

impl From<reqwest::Error> for crate::Error {
    fn from(value: reqwest::Error) -> Self {
        Self::E(value.to_string())
    }
}
