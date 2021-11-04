use reqwest::{self, header};
use std::result;

#[derive(Debug, Clone)]
pub struct Client {
    pub client: reqwest::Client,
    pub address: String,
    pub hook_token: Option<String>,
}

impl Client {
    pub fn new(address: String, auth_token: String) -> result::Result<Self, reqwest::Error> {
        let mut headers = header::HeaderMap::new();
        let mut auth_value =
            reqwest::header::HeaderValue::from_str(&format!("Bearer {}", auth_token)).unwrap();
        auth_value.set_sensitive(true);
        headers.insert(header::AUTHORIZATION, auth_value);
        Ok(Self {
            client: reqwest::Client::builder()
                .default_headers(headers)
                .build()?,
            address,
            hook_token: None,
        })
    }

    pub fn register(device_name: String) -> result::Result<(), reqwest::Error> {
        Ok(())
    }
}
