use async_trait::async_trait;
use rdkafka::{error::KafkaError, message::BorrowedMessage};

#[async_trait]
pub trait KafkaHandler {
    fn type_name(&self) -> &'static str;
    async fn handle(&self, msg: &BorrowedMessage) -> Result<(), KafkaError>;
}
