use async_trait::async_trait;

#[async_trait]
pub trait Actor: Send + Sync {
    fn created(&self) {}
    async fn started(&self) {}
    async fn stopped(&self) {}
}
