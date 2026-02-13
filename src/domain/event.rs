use crate::domain::config::EventConfig;

#[derive(Clone)]
pub struct Event {
    pub name: String,
    pub event_kind: EventKind
}

#[derive(Clone)]
pub enum EventKind {
    Log {
        server_name: String,
        keyword: String
    },
    Health {
        server_name: String,
        keyword: String
    },
    None
}

impl Event {
    
    pub fn from(event_config: EventConfig) -> Self {
        let event_kind = match event_config.r#type.as_str() {
            "logs" => {
                EventKind::Log {
                    server_name: event_config.target,
                    keyword: event_config.keyword
                }
            },
            "health" => {
                EventKind::Health {
                    server_name: event_config.target,
                    keyword: event_config.keyword
                }
            },
            _ => EventKind::None
        };
        Self {
            name: event_config.name,
            event_kind
        }
    }
}