mod alarm;

use std::error::Error;
use anyhow::anyhow;
use clap::command;
use log::{debug, trace};
use crate::application::handler::command::alarm::AlarmCommand;
use crate::application::handler::command::Command::{Alarm, HealthCheck, HealthCheckAll, Logs, Nothing};
use crate::application::handler::GeneralHandler;
use crate::domain::client::Message;

pub trait Run: Send + Sync {
    async fn run(&self, handler: &mut GeneralHandler, id: String, message: &Message) -> Result<String, Box<dyn Error + Send + Sync>>;
}

#[derive(Debug)]
pub enum Command {
    Logs(String, i32),
    HealthCheckAll,
    HealthCheck(String),
    Nothing,
    Alarm(AlarmCommand)
}

impl Run for Command {
    async fn run(&self, handler: &mut GeneralHandler, id: String, message: &Message) -> Result<String, Box<dyn Error + Send + Sync>> {
        match self {
            Command::Logs(name, n) => {
                handler.server_manager.logs(name.as_str(), *n).await
                    .ok_or_else(|| anyhow!("Logs are not available."))
                    .map_err(Into::into)
            },
            Command::HealthCheck(name) => {
                let health = handler.server_manager.healthcheck(name.as_str()).await;
                let response = format!("===\nServer: {name}\n Health: {health}");
                Ok(response)
            },
            Command::HealthCheckAll => {
                let response = handler.server_manager.healthcheck_all()
                    .await
                    .iter().map(|result|{format!("===\nServer: {}\nHealth: {}", result.0, result.1)})
                    .collect::<Vec<String>>()
                    .join("\n");
                Ok(response)
            },
            Command::Alarm(command) => {
                command.run(handler, id, message).await
            }
            Command::Nothing => Ok(String::from(crate::application::handler::general::INVALID_COMMAND_MESSAGE))
        }
    }
}

impl Command {
    pub fn parse(text: &str) -> Self {
        trace!("Command::parse(text: {})", &text);
        let command = match text.split_whitespace().collect::<Vec<_>>()[..] {
            ["/health", name] => HealthCheck(name.to_string()),
            ["/health"] => HealthCheckAll,
            ["/logs", name, n] => {
                match n.parse() {
                    Ok(n) => Logs(name.to_string(), n),
                    Err(_) => Nothing
                }
            },
            ["/alarm", "add", name] => {
                Alarm(AlarmCommand::Add(String::from(name)))
            },
            ["/alarm", "remove", name] => {
                Alarm(AlarmCommand::Remove(String::from(name)))
            },
            ["/alarm", "list"] => {
                Alarm(AlarmCommand::List)
            },
            ["/alarm"] => {
                Alarm(AlarmCommand::List)
            }
            _ => Nothing
        };
        debug!("parsed command: {:?}", &command);
        command
    }
}