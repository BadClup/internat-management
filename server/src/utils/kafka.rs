use std::time::Duration;
use rdkafka::ClientConfig;
use rdkafka::consumer::{Consumer, StreamConsumer};
use rdkafka::error::{KafkaError, KafkaResult};
use rdkafka::util::Timeout;
use crate::routes::chat::ChatMessage;

pub fn get_chat_consumer(resident_id: u32) -> KafkaResult<StreamConsumer> {
    let topic_name = "resident-chat";
    let group_name = format!("resident-{}", resident_id);

    let consumer: StreamConsumer = ClientConfig::new()
        .set("group.id", group_name)
        .set("bootstrap.servers", &crate::ENV.KAFKA_SERVER_URL)
        .set("enable.partition.eof", "false")
        .set("allow.auto.create.topics", "true")
        .set("auto.create.topics.enable", "true")
        .set("session.timeout.ms", "6000")
        .set("enable.auto.commit", "true") // delete messages every 5 second
        .create()?;

    consumer.subscribe(&[topic_name])?;

    Ok(consumer)
}

pub fn get_producer() -> KafkaResult<rdkafka::producer::FutureProducer> {
    ClientConfig::new()
        .set("bootstrap.servers", &crate::ENV.KAFKA_SERVER_URL)
        .set("message.timeout.ms", "5000")
        .create()
}

pub async fn send_chat_message(chat_message: ChatMessage, message_id: u32) -> KafkaResult<()> {
    let producer = get_producer()?;
    
    let parsed_message = match serde_json::to_string(&chat_message) {
        Ok(message) => message,
        Err(_) => return Err(KafkaError::Canceled),
    };
    
    let result = producer.send(
        rdkafka::producer::FutureRecord::to("resident-chat")
            .payload(&parsed_message)
            .key(&message_id.to_string()),
        Timeout::from(Duration::ZERO),
    ).await;

    match result {
        Ok(_) => Ok(()),
        Err((e, _)) => Err(e),
    }
}
