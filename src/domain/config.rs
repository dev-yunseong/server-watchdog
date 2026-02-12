use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub password: Option<String>,
    pub clients: Vec<ClientConfig>,
    pub servers: Vec<ServerConfig>
}

impl Config {
    pub fn new(password: Option<String>) -> Self {
        Self {
            password,
            clients: Vec::new(),
            servers: Vec::new()
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
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

#[derive(Serialize, Deserialize, Debug)]
pub struct ServerConfig {
    pub name: String,
    pub base_url: Option<String>,
    pub docker_container_name: Option<String>,
    pub health_check_path: Option<String>,
    pub kill_path: Option<String>,
    pub log_command: Option<String>,
}

impl ServerConfig {
    pub fn new(name: String, base_url: Option<String>, docker_container_name: Option<String>, health_check_path: Option<String>, kill_path: Option<String>, log_command: Option<String>,) -> Self {
        Self {
            name: String::from(name),
            base_url,
            docker_container_name,
            health_check_path,
            kill_path,
            log_command
        }
    }
}
