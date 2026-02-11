
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
    Unknown
}