use log::{debug, trace};
use crate::application::handler::command::Command::{HealthCheck, HealthCheckAll, Logs, Nothing};

#[derive(Debug)]
pub enum Command {
    Logs(String, i32),
    HealthCheckAll,
    HealthCheck(String),
    Nothing
}

impl Command {
    pub fn parse(text: String) -> Self {
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
            _ => Nothing
        };
        debug!("parsed command: {:?}", &command);
        command
    }
}