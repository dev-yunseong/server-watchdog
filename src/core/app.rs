use std::collections::HashMap;
use std::sync::OnceLock;
use tokio::runtime::Handle;
use tokio::task;
use crate::client::ClientKind;
use crate::core::config;
use crate::core::worker::{Worker, WorkerRegistry};
use crate::watchdog::server::Server;
use crate::watchdog::{HttpWatchdog, Watchdog, WatchdogKind};
use crate::watchdog::WatchdogKind::Http;

pub struct App {
    worker_registry: WorkerRegistry,
    client_registry: HashMap<String, ClientKind>,
    watchdog_registry: HashMap<String, WatchdogKind>
}

static INSTANCE: OnceLock<App> = OnceLock::new();

impl App {
    pub fn global() -> &'static Self {
        INSTANCE.get().expect("App must be initialized before use")
    }

    pub async fn init() {
        if INSTANCE.get().is_none() {
            let app = App::new().await;
            let _ = INSTANCE.set(app);
        }
    }

    async fn new() -> Self {
        config::init().await;
        let config = config::read().await;

        let mut worker_registry = WorkerRegistry::new();

        let mut client_registry = HashMap::new();
        let mut watchdog_registry = HashMap::new();

        for client_config in config.clients.clone().into_iter() {
            let client = match ClientKind::from(client_config) {
                Some(value) => value,
                None => continue
            };
            client_registry.insert(client.get_name().to_string(), client);
        }

        for server_config in config.servers.into_iter() {
            let server = Server::from(server_config);
            let watchdog = Http(HttpWatchdog::new(server));
            watchdog_registry.insert(watchdog.get_server_name().to_string(), watchdog);
        }

        worker_registry.register_batch(config.clients);

        App {
            worker_registry,
            client_registry,
            watchdog_registry
        }
    }
}