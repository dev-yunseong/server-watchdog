mod data;

use std::path::PathBuf;
use tokio::fs;
pub use data::*;

pub async fn init() {
    let directory_path = get_directory_path()
        .expect("Fail to find directory");

    if !directory_path.exists() {
        fs::create_dir_all(&directory_path).await
            .expect("Fail to save");
    }

    let config_path = directory_path.join("config.json");

    if !config_path.exists() {
        let placeholder = Config::new();
        let placeholder = serde_json::to_string_pretty(&placeholder)
            .expect("Fail to serialize");
        fs::write(config_path, placeholder).await
            .expect("Fail to save");
    }
}

pub async fn add_client(client_config: ClientConfig) {
    init().await;
    let mut config = read().await;
    config.clients.push(client_config);
    write(config).await;
}

pub async fn add_server(server_config: ServerConfig) {
    init().await;
    let mut config = read().await;
    config.servers.push(server_config);
    write(config).await;
}

pub async fn read() -> Config {
    let config_path = get_config_path()
        .expect("Fail to find config path");
    let config = fs::read_to_string(config_path).await
        .expect("Fail to read config");

    serde_json::from_str(config.as_str())
        .expect("Fail to deserialize")
}

pub async fn write(config: Config) {
    let directory_path = get_directory_path()
        .expect("Fail to find directory");

    let config_path = directory_path.join("config.json");
    let config = serde_json::to_string_pretty(&config)
        .expect("Fail to serialize");

    fs::write(config_path, config.as_str()).await
        .expect("Fail to write config");
}

pub async fn remove() {
    let directory_path = get_directory_path()
        .expect("Fail to find directory");
    fs::remove_dir_all(directory_path)
        .await
        .expect("Fail to Remove files");
}

fn get_config_path() -> Option<PathBuf> {
    let directory_path = get_directory_path()?;
    let config_path = directory_path.join("config.json");
    Some(config_path)
}

fn get_directory_path() -> Option<PathBuf> {
    let mut directory_path = home::home_dir().expect("Fail to find home directory");
    directory_path.push(".server-manager");
    Some(directory_path)
}


#[cfg(test)]
mod tests {
    use crate::core::config::*;
    use crate::core::config::data::ClientConfig;

    #[tokio::test]
    async fn init_and_remove() {
        init().await;
        let _config = read().await;
        remove().await;
    }

    #[tokio::test]
    async fn write_and_read() {
        init().await;

        let mut config = read().await;

        config.clients.push(
            ClientConfig::new_telegram(
                "telegram_client",
                "1234:token"
            )
        );

        let last_config_num = config.clients.len();

        write(config).await;

        let config = read().await;

        remove().await;
        assert_eq!(last_config_num, config.clients.len());
    }
}