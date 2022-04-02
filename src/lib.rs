pub mod api;
mod error;

pub use error::Error;

#[derive(Debug)]
pub struct FireAuth {
    pub api_key: String, // web api key
}

impl FireAuth {
    pub fn new(api_key: String) -> Self {
        Self{ api_key }
    }
}
