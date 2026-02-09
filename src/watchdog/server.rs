use serde::{Deserialize, Serialize};
use crate::core::registrar::config::ServerConfig;

#[derive(Serialize, Deserialize)]
pub struct Server {
    pub name: String,
    proto: String,
    host: String,
    port: i16,
    health_check_path: Option<String>,
    kill_path: Option<String>,
}

impl Server {

    pub fn get_url(&self) -> String {
        format!("{}://{}:{}", self.proto, self.host, self.port)
    }

    pub fn get_health_check_url(&self) -> Option<String> {
        let url = self.get_url();
        let health_check_path = self.health_check_path.as_ref()?.trim_start_matches('/');
        Some(format!("{url}/{health_check_path}"))
    }

    pub fn get_kill_url(&self) -> Option<String> {
        let url = self.get_url();
        let kill_path = self.kill_path.as_ref()?.trim_start_matches('/');
        Some(format!("{url}/{kill_path}"))
    }

    pub fn from(config: ServerConfig) -> Self {
        Self {
            name: config.name,
            proto: config.proto,
            host: config.host,
            port:config.port,
            health_check_path: config.health_check_path,
            kill_path: config.kill_path
        }
    }
}

