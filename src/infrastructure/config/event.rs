use crate::application::config::{EventConfigUseCase, EventSubscribeUseCase};
use crate::domain::config::{Config, EventConfig, EventSubscribe, EventSubscribeList};
use crate::domain::file_accessor::FileAccessor;
use async_trait::async_trait;
use derive_new::new;
use std::error::Error;
use std::sync::Arc;

#[derive(new)]
pub struct EventConfigAdapter {
    config_file_accessor: Arc<dyn FileAccessor<Config>>,
    subscribe_file_accessor: Arc<dyn FileAccessor<EventSubscribeList>>,
}

#[async_trait]
impl EventConfigUseCase for EventConfigAdapter {
    async fn add_event(
        &self,
        event_config: EventConfig,
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        let mut config = self.config_file_accessor.read().await?;
        config.events.push(event_config);
        self.config_file_accessor.write(&config).await?;
        Ok(())
    }

    async fn list_event(&self) -> Result<Vec<EventConfig>, Box<dyn Error + Send + Sync>> {
        let config = self.config_file_accessor.read().await?;
        Ok(config.events)
    }

    async fn remove_event(&self, name: String) -> Result<(), Box<dyn Error + Send + Sync>> {
        let mut config = self.config_file_accessor.read().await?;
        config.events.retain(|event| event.name != name);
        self.config_file_accessor.write(&config).await?;
        Ok(())
    }
}

#[async_trait]
impl EventSubscribeUseCase for EventConfigAdapter {
    async fn subscribe(
        &self,
        chat_id: String,
        event_name: String,
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        // Validate that the event exists in the config
        let config = self.config_file_accessor.read().await?;
        let event_exists = config.events.iter().any(|e| e.name == event_name);
        if !event_exists {
            return Err(format!("Event '{}' does not exist in configuration", event_name).into());
        }

        let mut subscribe_file: EventSubscribeList = self.subscribe_file_accessor.read().await?;

        if subscribe_file.contains(event_name.as_str(), chat_id.as_str()) {
            return Ok(());
        }

        match subscribe_file.find_subscribe_mut(event_name.as_str()) {
            Some(subscribe) => {
                subscribe.chat_ids.push(chat_id);
            }
            None => {
                let mut chat_ids = Vec::new();
                chat_ids.push(chat_id);
                subscribe_file.subscribes.push(EventSubscribe {
                    event_name,
                    chat_ids,
                })
            }
        }

        let _ = self.subscribe_file_accessor.write(&subscribe_file).await?;
        Ok(())
    }

    async fn list_subscribed_event(
        &self,
        chat_id: String,
    ) -> Result<Vec<EventConfig>, Box<dyn Error + Send + Sync>> {
        let subscribe_file: EventSubscribeList = self.subscribe_file_accessor.read().await?;
        let event_names = subscribe_file.find_subscribed_events(chat_id.as_str());
        let config = self.config_file_accessor.read().await?;

        let event_configs = config
            .events
            .into_iter()
            .filter(|event_config| event_names.contains(&event_config.name.as_str()))
            .collect();

        Ok(event_configs)
    }

    async fn unsubscribe(
        &self,
        chat_id: String,
        event_name: String,
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        let mut subscribe_file: EventSubscribeList = self.subscribe_file_accessor.read().await?;
        subscribe_file.unsubscribe(event_name.as_str(), chat_id.as_str());
        let _ = self.subscribe_file_accessor.write(&subscribe_file).await?;

        Ok(())
    }
}
