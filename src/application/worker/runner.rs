use std::collections::HashMap;
use std::time::Duration;
use tokio::task::JoinHandle;
use crate::application::worker::Worker;

pub struct WorkerRunner {
    handles: HashMap<String, JoinHandle<()>>
}

impl WorkerRunner {
    pub fn new() -> Self {
        Self {
            handles: HashMap::new()
        }
    }

    pub fn stop(&mut self, key: &str) {
        let handle = match self.handles.get_mut(key) {
            Some(handle) => handle,
            None => return
        };
        handle.abort();
        self.handles.remove(key);
    }

    pub fn run_batch(&mut self, workers: Vec<Box<dyn Worker>>) {
        for worker in workers.into_iter() {
            self.run(worker);
        }
    }

    pub fn run(&mut self, mut worker: Box<dyn Worker>) {
        let key = worker.get_name().to_string();
        let handle = tokio::spawn(async move {
            let mut interval = tokio::time::interval(Duration::from_secs(worker.interval() as u64));

            loop {
                interval.tick().await;
                if !worker.on_tick().await {
                    break;
                }
            }
        });
        self.handles.insert(key, handle);
    }
}

#[cfg(test)]
mod tests {
    use std::env;
    use dotenv::dotenv;
    use dyn_clone::clone_trait_object;
    use tokio::sync::mpsc;
    use crate::domain::config::{ClientConfig};
    use crate::infrastructure::client;
    use crate::infrastructure::client::common::{Client};
    use crate::application::worker::runner::{ WorkerRunner};

    #[tokio::test]
    async fn run_work() {
        dotenv().ok();
        let token = env::var("TELEGRAM_TOKEN").unwrap();
        let mut registry = WorkerRunner::new();
        let mut client = client::from(
            ClientConfig::new_telegram("name", token.as_str())).unwrap();

        let (tx, mut rx) = mpsc::channel(16);
        client.subscribe(tx);
        let client_for_callback = client.clone();
        tokio::spawn(async move {
            loop {
                match rx.recv().await {
                    Some(message) => {
                        let chat_id_owned = message.chat_id;
                        let text_owned = message.data;
                        client_for_callback.send_message(chat_id_owned.as_str(), format!("echo {chat_id_owned}: {text_owned}").as_str()).await;
                    },
                    None => {
                        break;
                    }
                }
            }
        });
        registry.run(client);
        tokio::signal::ctrl_c().await.unwrap();
    }
}