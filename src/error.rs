use reqwest::Error;
use std::fmt::{self, Display, Formatter};

#[derive(Debug)]
pub enum ErrorCause {
    RequestTimeOut,
    UnknownError,
    ConnectionError,
    DecodeError,
    BodyError,
}

#[derive(Debug)]
pub struct CustomErrors {
    message: String,
    cause: ErrorCause,
}

impl Display for CustomErrors {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        writeln!(f, "{}", self.message)
    }
}

impl std::error::Error for CustomErrors {}

impl From<Error> for CustomErrors {
    fn from(error: Error) -> CustomErrors {
        if error.is_timeout() {
            CustomErrors::new(ErrorCause::RequestTimeOut)
        } else if error.is_body() {
            CustomErrors::new(ErrorCause::BodyError)
        } else if error.is_decode() {
            CustomErrors::new(ErrorCause::DecodeError)
        } else if error.is_connect() {
            CustomErrors::new(ErrorCause::ConnectionError)
        } else {
            CustomErrors::new(ErrorCause::UnknownError)
        }
    }
}

impl CustomErrors {
    pub fn new(cause: ErrorCause) -> CustomErrors {
        use ErrorCause::*;

        CustomErrors {
            message: match cause {
                RequestTimeOut => "Request Timeout".to_string(),
                UnknownError => "Unknown Error".to_string(),
                BodyError => "Body Error".to_string(),
                ConnectionError => "Internet Connection Error".to_string(),
                DecodeError => "Decode Body Error".to_string(),
            },
            cause,
        }
    }
}
