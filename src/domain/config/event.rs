use derive_new::new;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct EventConfig {
    pub r#type: String, // logs, health
    pub name: String,
    pub target: String, // target server
    pub keyword: String,
}

#[derive(Serialize, Deserialize, Debug, new)]
pub struct EventSubscribeList {
    #[new(default)]
    pub subscribes: Vec<EventSubscribe>
}

impl EventSubscribeList {
    
    pub fn unsubscribe(&mut self, event_name: &str, chat_id: &str) {
        if let Some(subscribes) = self.find_subscribe_mut(event_name) {
            subscribes.chat_ids.retain(|id|{ id != chat_id})
        }
    }
    
    pub fn find_subscribed_events(&self, chat_id: &str) -> Vec<&str> {
        self.subscribes.iter()
            .filter(|subscribe| {subscribe.contains(chat_id)})
            .map(|subscribe| {subscribe.event_name.as_str()})
            .collect()
    }
    
    pub fn find_subscribe(&self, event_name: &str) -> Option<&EventSubscribe> {
        self.subscribes.iter()
            .filter(|subscribe| {subscribe.event_name.eq(event_name)})
            .next()
    }

    pub fn find_subscribe_mut(&mut self, event_name: &str) -> Option<&mut EventSubscribe> {
        self.subscribes.iter_mut()
            .filter(|subscribe| {subscribe.event_name.eq(event_name)})
            .next()
    }


    pub fn contains(&self, event_name: &str, chat_id: &str) -> bool {
        match self.find_subscribe(event_name) {
            Some(subscribe) => {
                subscribe.contains(chat_id)
            },
            None => false
        }
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct EventSubscribe {
    pub event_name: String,
    pub chat_ids: Vec<String>
}

impl EventSubscribe {
    
    pub fn contains(&self, chat_id: &str) -> bool {
        self.chat_ids.iter().any(|id| id == chat_id)
    }
}