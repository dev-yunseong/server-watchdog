mod runner;
pub  mod registrar;

use crate::client::ClientKind;
use std::collections::HashMap;
use async_trait::async_trait;

#[async_trait]
pub trait Worker {
    async fn on_tick(&mut self);
    fn get_name(&self) -> &str;
    fn interval(&self) -> i32;
}

pub struct WorkerRegistry {
    workers: HashMap<String, Box<dyn Worker>>
}

impl WorkerRegistry {

    fn new() -> Self {
        Self {
            workers: HashMap::new()
        }
    }

    fn register(&mut self, worker: Box<dyn Worker>) {
        self.workers.insert(
            worker.get_name().to_string(),
            worker
        );
    }

    async fn load(&mut self) {
        let config = registrar::read().await;

        for client_config in config.clients.into_iter() {
            let client = ClientKind::from(client_config);
            let client = match client {
                Some(client) => client,
                None => continue,
            };
            self.register(Box::new(client));
        }
    }
}