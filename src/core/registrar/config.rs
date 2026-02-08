use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub  struct Config {
    pub clients: Vec<ClientConfig>
}

impl Config {
    pub fn new() -> Self {
        Self {
            clients: Vec::new()
        }
    }
}

#[derive(Serialize, Deserialize)]
pub  struct ClientConfig {
    pub  name: String,
    pub  kind: String, // ex: telegram
    pub  token: Option<String>
}

impl ClientConfig {
    pub fn new_telegram(name: &str, token: &str) -> Self {
        Self {
            name: String::from(name),
            kind: String::from("telegram"),
            token: Some(String::from(token))
        }
    }
}

