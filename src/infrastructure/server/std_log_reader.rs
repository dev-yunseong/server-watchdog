mod util;

use log::error;
use crate::domain::server::Server;
use crate::infrastructure::server::std_log_reader::util::SystemCommandExecutor;

pub struct StdLogReader {
    system_command_executor: SystemCommandExecutor
}

impl StdLogReader {

    pub fn new() -> Self {
        Self {
            system_command_executor: SystemCommandExecutor::new()
        }
    }
    pub async fn read(&self, server: &Server, n: i32) -> Option<String> {
        if let Some(log_command) = server.log_command.as_ref() {
            let n_str = n.to_string();

            let mut args: Vec<&str> = log_command[1..]
                .iter()
                .map(|s| s.as_str())
                .collect();

            args.insert(0, "-n");
            args.insert(1, &n_str);
            println!("실행될 명령어: {} {:?}", log_command[0].as_str(), args);

            let result = self.system_command_executor.capture_output(
                log_command[0].as_str(),
                &args
            ).await;

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