use std::error::Error;
use crate::application::handler::command::Run;
use crate::application::handler::GeneralHandler;
use crate::domain::client::Message;
use crate::domain::config::EventConfig;

#[derive(Debug)]
pub enum AlarmCommand {
    Add(String),
    Remove(String),
    List
}

impl Run for AlarmCommand {
    async fn run(&self, handler: &mut GeneralHandler, id: String, message: &Message) -> Result<String, Box<dyn Error + Send + Sync>> {
        match self {
            AlarmCommand::Add(event_name) => {
                let _ = handler.event_subscribe_use_case
                    .subscribe(id, event_name.clone()).await?;
                Ok(String::from("Successfully subscribed"))
            },
            AlarmCommand::Remove(event_name) => {
                let _ = handler.event_subscribe_use_case
                    .unsubscribe(id, event_name.clone()).await?;
                Ok(String::from("Successfully removed"))
            },
            AlarmCommand::List => {
                let response = handler.event_subscribe_use_case
                    .list_subscribed_event(id).await?
                    .into_iter()
                    .map(|config: EventConfig| {format!("---\nname: {}\ntype: {}\ntarget: {}\nkeyword: {}",
                        config.name,
                        config.r#type,
                        config.target,
                        config.keyword)})
                    .collect::<Vec<String>>()
                    .join("\n\n");
                Ok(format!("--- list ---\n{response}"))
            }
        }
    }
}
