mod api;
pub mod error;

pub struct FireAuth {
    pub api_key: String, // web api key
}

impl FireAuth {
    pub fn new(api_key: String) -> Self {
        Self{ api_key }
    }
}
