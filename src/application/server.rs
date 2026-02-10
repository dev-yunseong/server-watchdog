use async_trait::async_trait;
use crate::domain::server::{Health, Server};

pub trait ServerRepository : Send + Sync {
    fn find(&self, name: &str) -> Option<&Server>;
    fn find_all(&self) -> Vec<&Server>;
}

#[async_trait]
pub trait ServerManager : Send + Sync {
    async fn kill(&self, name: &str) -> bool;
    async fn healthcheck(&self, name: &str) -> Health;
    async fn logs(&self, name: &str, n: i32) -> Option<String>;
}