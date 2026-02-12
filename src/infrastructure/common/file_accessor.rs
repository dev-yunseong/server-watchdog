use std::error::Error;
use std::path::PathBuf;
use anyhow::anyhow;
use derive_new::new;
use serde::{Serialize};
use serde::de::DeserializeOwned;
use tokio::fs;
use crate::domain::chat::ChatList;
use crate::domain::config::{Config, EventSubscribeList};

#[derive(new)]
pub struct FileAccessor<T>
    where T: DeserializeOwned + Serialize
{
    file_name: String,
    factory: Box<dyn Fn() -> T + Send + Sync>,
}

impl<T> FileAccessor<T>
where
    T: Serialize + DeserializeOwned
{
    pub async fn read(&self)
        -> Result<T, Box<dyn Error + Send + Sync>>
    {
        let file_path = self.get_file_path()?;

        if file_path.exists() {
            let raw_string = fs::read_to_string(file_path).await?;
            Ok(serde_json::from_str(raw_string.as_str())?)
        } else {
            Ok((self.factory)())
        }
    }

    pub async fn write(&self, data: T)
        -> Result<(), Box<dyn Error + Send + Sync>> {
        let raw_json = serde_json::to_string_pretty(&data)?;

        let directory_path = self.get_directory_path()?;

        fs::create_dir_all(directory_path).await?;

        let file_path = self.get_file_path()?;

        let mut temp_path = file_path.clone();
        temp_path.set_extension("tmp");

        fs::write(&temp_path, &raw_json).await?;
        fs::rename(&temp_path, &file_path).await?;

        Ok(())
    }

    fn get_file_path(&self)
        -> Result<PathBuf, Box<dyn Error + Send + Sync>> {
        let mut path = self.get_directory_path()?;
        path.push(self.file_name.as_str());
        Ok(path)
    }

    fn get_directory_path(&self) -> Result<PathBuf, Box<dyn Error + Send + Sync>> {
        let mut directory_path = home::home_dir()
            .ok_or(anyhow!("Fail to find home directory"))?;
        directory_path.push(".watchdog");
        Ok(directory_path)
    }
}

pub fn get_chat_list_file_accessor() -> FileAccessor<ChatList> {
    FileAccessor::new(
        String::from("chat_list.json"),
        Box::new(|| { ChatList::new() })
    )
}

pub fn get_config_file_accessor() -> FileAccessor<Config> {
    FileAccessor::new(
        String::from("config.json"),
        Box::new(||{Config::new(None)})
    )
}

pub fn get_event_subscribe_file_accessor() -> FileAccessor<EventSubscribeList> {
    FileAccessor::new(
        String::from("subscribe.json"),
        Box::new(||{EventSubscribeList::new()})
    )
}