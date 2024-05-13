use async_trait::async_trait;
use rdkafka::{error::KafkaError, message::BorrowedMessage, Message};

use super::kafka_handler::KafkaHandler;

pub struct TweetCreated;

#[async_trait]
impl KafkaHandler for TweetCreated {
    async fn handle(&self, msg: &BorrowedMessage) -> Result<(), KafkaError> {
        println!("TweetCreated: {:?}", msg.payload_view::<str>());
        Ok(())
    }
}
