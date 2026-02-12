use std::error::Error;
use async_trait::async_trait;

#[async_trait]
pub trait AuthUseCase : Send + Sync {
    async fn set_password(&self, password: Option<String>) -> Result<(), Box<dyn Error + Send + Sync>>;
    async fn validate_password(&mut self, password: String) -> bool;
    async fn register(&mut self, client_name: String, identity: String) -> Result<(), Box<dyn Error + Send + Sync>>;
    async fn authenticate(&mut self, client_name: String, identity: String) -> Option<String>;
    fn password_required(&self) -> bool;
}