use log::{debug, error, trace};
use tokio_stream::Stream;
use crate::domain::server::Server;
use crate::infrastructure::server::util::{SystemCommandExecutor, ChildProcessStream};

pub struct StdLogReader {
    system_command_executor: SystemCommandExecutor
}

impl StdLogReader {

    pub fn new() -> Self {
        Self {
            system_command_executor: SystemCommandExecutor::new()
        }
    }

    pub async fn read_follow(&self, server: &Server) -> Option<Box<dyn Stream<Item = String> + Send>> {
        trace!("StdLogReader::read_follow for server: {}", server.name);
        let log_command = server.log_command.as_ref()?;

        debug!("log_command: {:?}", log_command);

        let mut args: Vec<&str> = log_command[1..]
            .iter()
            .map(|s| s.as_str())
            .collect();

        let command = log_command[0].as_str();

        match command {
            "docker" => {
                args.push("-f");
            }
            _ => {
                args.insert(0, "-f");
            }
        }

        self.system_command_executor.capture_output_follow(
            command, &args
        ).await.ok()
    }

    pub async fn read(&self, server: &Server, n: i32) -> Option<String> {
        trace!("StdLogReader::read for server: {}, n: {}", server.name, n);
        if let Some(log_command) = server.log_command.as_ref() {
            debug!("log_command: {:?}", log_command);
            let n_str = n.to_string();

            let mut args: Vec<&str> = log_command[1..]
                .iter()
                .map(|s| s.as_str())
                .collect();

            let command = log_command[0].as_str();
            debug!("command: {}", command);
            match command {
                "docker" => {
                    args.push("-n");
                    args.push(&n_str);
                }
                _ => {
                    args.insert(0, "-n");
                    args.insert(1, &n_str);
                }
            }
            debug!("args: {:?}", &args);
            let full_command = format!("{} {}", command, args.join(" "));
            debug!("Executing command: {}", full_command);

            let result = self.system_command_executor.capture_output(
                log_command[0].as_str(),
                &args
            ).await;
            debug!("result: {:?}", &result);

            match result {
                Ok(str) => Some(str),
                Err(e) => {
                    error!("[StdLogReader] Err: {e}");
                    None
                }
            }
        } else {
            None
        }
    }
}