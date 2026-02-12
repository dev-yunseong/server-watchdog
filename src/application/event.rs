use crate::application::client::MessageGateway;
use crate::application::worker::WorkerRunner;
use crate::domain::event::Event;
use crate::infrastructure::client::MessageAdapter;

pub struct EventTask {
    event: Event,
}

impl EventTask {

    pub fn check() {
        loop {

        }
    }
}

pub struct EventMessage {
    event_name: String,
    text: String
}

pub struct EventManager {
    message_gateway: Box<dyn MessageGateway>,
    
}

impl EventManager {
    
    pub fn handle(&self, event_message: EventMessage) {
        
    }
}