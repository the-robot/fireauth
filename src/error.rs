use thiserror::Error;

#[derive(Error, Debug, Clone)]
pub enum Error {
    #[error("{0}")]
    API(String),

    #[error("{0}")]
    SignUp(String),

    #[error("{0}")]
    SignIn(String),
}

impl std::convert::From<reqwest::Error> for Error {
    fn from(err: reqwest::Error) -> Self {
        match err {
            _ => Error::API(err.to_string()),
        }
    }
}
