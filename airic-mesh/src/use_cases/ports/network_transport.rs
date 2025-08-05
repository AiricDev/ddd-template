use crate::core::{errors::Result, wire_message::WireMessage};
use async_trait::async_trait;
use std::future::Future;
use std::pin::Pin;
use uuid::Uuid;

pub type MessageHandler =
    Box<dyn Fn(WireMessage) -> Pin<Box<dyn Future<Output = Result<()>> + Send>> + Send + Sync>;

#[async_trait]
pub trait NetworkTransport: Send + Sync {
    async fn send(&self, destination_device_id: Uuid, message: WireMessage) -> Result<()>;
    fn set_message_handler(&mut self, handler: MessageHandler);
}
