use notifications::infra::kafka::{KafkaHandler, TweetCreated, TweetLiked};
use rdkafka::config::{ClientConfig, RDKafkaLogLevel};
use rdkafka::consumer::{CommitMode, Consumer, ConsumerContext, Rebalance, StreamConsumer};
use rdkafka::error::KafkaResult;
use rdkafka::{ClientContext, Message, TopicPartitionList};

struct CustomContext;

impl ClientContext for CustomContext {}

impl ConsumerContext for CustomContext {
    fn pre_rebalance(&self, rebalance: &Rebalance) {
        println!("Pre rebalance {:?}", rebalance);
    }

    fn post_rebalance(&self, rebalance: &Rebalance) {
        println!("Post rebalance {:?}", rebalance);
    }

    fn commit_callback(&self, result: KafkaResult<()>, _offsets: &TopicPartitionList) {
        println!("Committing offsets: {:?}", result);
    }
}

type LoggingConsumer = StreamConsumer<CustomContext>;

#[macro_export]
macro_rules! hashmap {
    ($($key: expr => $value: expr),*) => {{
        let mut map = ::std::collections::HashMap::new();
        $(map.insert($key, $value);)*
        map
    }}
}

#[tokio::main]
async fn main() {
    println!("Starting consumer...");

    let handlers = hashmap!(
        "TweetCreated" => Box::new(TweetCreated) as Box<dyn KafkaHandler>,
        "TweetLiked" => Box::new(TweetLiked) as Box<dyn KafkaHandler>
    );

    let context = CustomContext;
    let consumer: LoggingConsumer = ClientConfig::new()
        .set("group.id", "rs_notification")
        .set("bootstrap.servers", "localhost:9092")
        .set("enable.partition.eof", "false")
        .set("session.timeout.ms", "6000")
        .set("enable.auto.commit", "true")
        //.set("statistics.interval.ms", "30000")
        //.set("auto.offset.reset", "smallest")
        .set_log_level(RDKafkaLogLevel::Debug)
        .create_with_context(context)
        .expect("Consumer creation failed");

    println!("Subscribing to topic(s) TweetCreated");

    consumer
        .subscribe(&["TweetCreated"])
        .expect("Can't subscribe to topic");

    println!("Hello world");

    loop {
        match consumer.recv().await {
            Ok(msg) => {
                println!("Received message: {:?}", msg);

                let topic = msg.topic();
                match handlers.get(topic) {
                    Some(handler) => match handler.handle(&msg).await {
                        Ok(_) => {
                            println!("Message handled successfully");
                            consumer
                                .commit_message(&msg, CommitMode::Async)
                                .expect("Commit failed");
                        }
                        Err(e) => {
                            println!("Error while handling message: {:?}", e);
                        }
                    },
                    None => {
                        println!("No handler found for topic: {}", topic);
                    }
                }
            }
            Err(err) => {
                println!("Error while receiving message: {:?}", err);
            }
        }
    }
}
