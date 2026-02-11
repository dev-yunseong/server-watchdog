use std::fmt::{Display, Formatter};

pub enum HealthCheckMethod {
    Http(String),
    Docker,
    None
}

pub enum Health {
    Healthy,
    Unhealthy,
    Deregistered, // Draining
    Degraded,
    Down,         // Dead
    Unknown(String)
}

impl Display for Health {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let val = match self {
            Health::Healthy => "Healthy",
            Health::Unhealthy => "Unhealthy",
            Health::Deregistered => "Deregistered (Draining)",
            Health::Degraded => "Degraded",
            Health::Down => "Down",
            Health::Unknown(msg) => msg,
        };
        write!(f, "{}", val)
    }
}