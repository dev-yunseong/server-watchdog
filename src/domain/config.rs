use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub clients: Vec<ClientConfig>,
    pub servers: Vec<ServerConfig>
}

impl Config {
    pub fn new() -> Self {
        Self {
            clients: Vec::new(),
            servers: Vec::new()
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ClientConfig {
    pub name: String,
    pub kind: String, // ex: telegram
    pub token: Option<String>
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

#[derive(Serialize, Deserialize)]
pub struct ServerConfig {
    pub name: String,
    pub proto: String,
    pub host: String,
    pub port: i16,
    pub health_check_path: Option<String>,
    pub kill_path: Option<String>,
    pub log_command: Option<String>,
}

impl ServerConfig {
    pub fn new(name: String, proto: String, host: String, port: i16, health_check_path: Option<String>, kill_path: Option<String>, log_command: Option<String>,) -> Self {
        Self {
            name: String::from(name),
            proto: String::from(proto),
            host: String::from(host),
            port,
            health_check_path,
            kill_path,
            log_command
        }
    }
}
