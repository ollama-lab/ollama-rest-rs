#[derive(Debug)]
pub enum Error {
    ClientCreation(reqwest::Error),
    Event,
    UrlParsing(url::ParseError),
}

impl From<reqwest::Error> for Error {
    fn from(value: reqwest::Error) -> Self {
        Self::ClientCreation(value)
    }
}

impl From<url::ParseError> for Error {
    fn from(value: url::ParseError) -> Self {
        Self::UrlParsing(value)
    }
}
