pub mod runner;

use async_trait::async_trait;
pub use runner::WorkerRunner;

#[async_trait]
pub trait Worker: Send {
    async fn on_tick(&mut self) -> bool;
    fn get_name(&self) -> &str;
    fn interval(&self) -> i32;
}
