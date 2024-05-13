use async_trait::async_trait;
use rdkafka::{error::KafkaError, message::BorrowedMessage};

#[async_trait]
pub trait KafkaHandler {
    async fn handle(&self, msg: &BorrowedMessage) -> Result<(), KafkaError>;
}
