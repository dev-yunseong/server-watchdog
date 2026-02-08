
pub trait Worker {
    async fn on_tick(&mut self);
    fn interval_ms() -> i32;
}