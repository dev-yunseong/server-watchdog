pub mod health;

use crate::domain::config::ServerConfig;
use crate::domain::server::health::HealthCheckMethod;

pub struct Server {
    pub name: String,
    pub base_url: Option<String>,
    pub docker_container_name: Option<String>,
    pub health_check_method: HealthCheckMethod,
    pub kill_path: Option<String>,
    pub log_command: Option<Vec<String>>
}

impl Server {
    pub fn get_health_check_url(&self) -> Option<String> {
        let health_check_path = match &self.health_check_method {
            HealthCheckMethod::Http(value) => value,
            _ => return None
        };
        Some(format!("{}/{health_check_path}", self.base_url.as_ref()?))
    }

    pub fn get_kill_url(&self) -> Option<String> {
        let kill_path = self.kill_path.as_ref()?.trim_start_matches('/');
        Some(format!("{}/{kill_path}", self.base_url.as_ref()?))
    }

    pub fn from(config: ServerConfig) -> Self {
        let log_command = if let Some(raw_command) = config.log_command {
            Some(raw_command.split_whitespace().map(|ref_str|{String::from(ref_str)}).collect())
        } else { 
            None
        };

        let health_check_method = match config.health_check_path {
            Some(path) => HealthCheckMethod::Http(path),
            None => {
                if config.docker_container_name.is_some() {
                    HealthCheckMethod::Docker
                } else {
                    HealthCheckMethod::None
                }
            }
        };
        
        Self {
            name: config.name,
            base_url: config.base_url,
            docker_container_name: config.docker_container_name,
            health_check_method,
            kill_path: config.kill_path,
            log_command
        }
    }
}