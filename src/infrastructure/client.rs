pub mod common;
pub mod telegram;

use std::collections::HashMap;
use std::error::Error;
use async_trait::async_trait;
use derive_new::new;
use tokio::sync::mpsc;
use tokio::sync::mpsc::Receiver;
pub use common::*;
use crate::application::client::{ClientLoader, MessageGateway};
use crate::application::worker::Worker;
use crate::domain::client::Message;
use crate::infrastructure::{client};
use crate::application::worker::WorkerRunner;
use crate::domain::config::Config;
use crate::infrastructure::common::file_accessor::{get_config_file_accessor, FileAccessor};

#[derive(new)]
pub struct MessageAdapter {
    client_loader: Box<dyn ClientLoader>
}

#[async_trait]
impl MessageGateway for MessageAdapter {
    async fn send_message(&self, client_name: &str, chat_id: &str, message: &str) {
        let client = self.client_loader.find(client_name)
            .expect(format!("client({client_name}) is not available").as_str());

        let total_len = message.len();
        
        let mut cut_length = 0;
        
        while cut_length < total_len {
            let end = std::cmp::min(cut_length + 4000, total_len);
            let chunk = &message[cut_length..end];
            
            client.send_message(chat_id, chunk).await;
            
            cut_length = end;
            tokio::time::sleep(std::time::Duration::from_millis(500)).await;
        }
    }
}

pub struct ClientManager {
    worker_runner: WorkerRunner,
    client_map: HashMap<String, Box<dyn Client>>,
    config_file_accessor: FileAccessor<Config>
}

impl ClientManager {
    pub fn new() -> Self {
        Self {
            worker_runner: WorkerRunner::new(),
            client_map: HashMap::new(),
            config_file_accessor: get_config_file_accessor()
        }
    }
}

#[async_trait]
impl ClientLoader for ClientManager {
    async fn load_clients(&mut self) -> Result<(), Box<dyn Error + Send + Sync>> {
        let clients = self.config_file_accessor.read().await?.clients;
        let clients: Vec<Box<dyn Client>> = clients.into_iter()
            .map(|client_config| {client::from(client_config)})
            .filter(|option|{option.is_some()})
            .map(|client| { client.unwrap() })
            .collect();

        self.client_map.clear();

        for client in clients.into_iter() {
            self.client_map.insert(client.get_name().to_string(), client);
        }
        Ok(())
    }

    fn find(&self, name: &str) -> Option<&Box<dyn Client>> {
        self.client_map.get(name)
    }

    async fn run(&mut self) -> Receiver<Message> {
        let (tx, rx) = mpsc::channel(16);
        let mut clients: Vec<Box<dyn Client>> = self.client_map
            .values()
            .map(|c| dyn_clone::clone_box(&**c))
            .collect();

        for client in clients.iter_mut() {
            let tx = tx.clone();
            client.subscribe(tx);
        }

        let workers: Vec<Box<dyn Worker>> = clients
            .into_iter()
            .map(|c| c as Box<dyn Worker>)
            .collect();

        self.worker_runner.run_batch(workers);

        rx
    }
}