use std::pin::Pin;
use std::sync::Arc;
use std::time::Duration;
use derive_new::new;
use tokio::sync::mpsc::Sender;
use tokio_stream::StreamExt;
use crate::application::event::dto::EventMessage;
use crate::application::server::ServerManager;
use crate::domain::config::Config;
use crate::domain::event::{Event, EventKind};
use crate::domain::file_accessor::FileAccessor;

#[derive(new)]
pub struct GeneralEventChecker {
    config_file_accessor: Arc<dyn FileAccessor<Config>>,
    server_manager: Arc<dyn ServerManager>,
    tx: Sender<EventMessage>,
    health_event_checker: Box<dyn EventChecker>,
    log_event_checker: Box<dyn EventChecker>
}

impl GeneralEventChecker {

    pub async fn init(&self) {
        let config = self.config_file_accessor.read()
            .await.unwrap();
        let events: Vec<Event> = config.events.into_iter()
            .map(|event_config|{Event::from(event_config)})
            .collect();
        for event in events {
            self.check(event.event_kind);
        }
    }

    fn check(&self, event_kind: EventKind) {
        match &event_kind {
            EventKind::Health {server_name: _, keyword: _} => {
                self.health_event_checker
                    .check(event_kind, self.server_manager.clone(), self.tx.clone())
            },
            EventKind::Log {server_name: _, keyword: _} => {
                self.log_event_checker
                    .check(event_kind, self.server_manager.clone(), self.tx.clone())
            },
            EventKind::None => return
        }
    }
}

pub trait EventChecker {
    fn check(&self, event_kind: EventKind, server_manager: Arc<dyn ServerManager>, tx: Sender<EventMessage>);
}

#[derive(new)]
pub struct HealthEventChecker;

impl EventChecker for HealthEventChecker {
    fn check(&self, event_kind: EventKind, server_manager: Arc<dyn ServerManager>, tx: Sender<EventMessage>) {
        if let EventKind::Health { server_name, keyword } = event_kind {
            tokio::spawn(async move {
                loop {
                    let health = server_manager.healthcheck(server_name.as_str()).await;
                    if health.to_string().contains(keyword.as_str()) {
                        let _ = tx.send(EventMessage {
                            event_name: server_name.clone(),
                            text: format!("Keyword '{}' found in health check of server '{}'", keyword, server_name),
                        }).await;
                    }
                    tokio::time::sleep(Duration::from_secs(30)).await;
                }
            });
        }
    }
}

#[derive(new)]
pub struct LogEventChecker;

impl EventChecker for LogEventChecker {
    fn check(&self, event_kind: EventKind, server_manager: Arc<dyn ServerManager>, tx: Sender<EventMessage>) {

        if let EventKind::Log {server_name, keyword} = event_kind {
            tokio::spawn(async move {
                let stream = server_manager.logs_stream(server_name.as_str()).await;

                if let Some(stream) = stream {
                    let mut stream = Pin::from(stream);
                    while let Some(line) = stream.next().await {
                        if line.contains(keyword.as_str()) {
                            let _ = tx.send(EventMessage {
                                event_name: server_name.clone(),
                                text: format!("Keyword '{}' found in logs of server '{}'\nLog: {}", keyword, server_name, line),
                            }).await;
                        }
                    }
                }
            });
        }
    }
}
