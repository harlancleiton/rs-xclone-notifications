use async_trait::async_trait;
use rdkafka::{error::KafkaError, message::BorrowedMessage, Message};

use super::kafka_handler::KafkaHandler;

pub struct TweetLiked;

#[async_trait]
impl KafkaHandler for TweetLiked {
    fn type_name(&self) -> &'static str {
        "TweetLiked"
    }

    async fn handle(&self, msg: &BorrowedMessage) -> Result<(), KafkaError> {
        println!("TweetLiked: {:?}", msg.payload_view::<str>());
        Ok(())
    }
}
