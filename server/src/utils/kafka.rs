use rdkafka::ClientConfig;
use rdkafka::consumer::{Consumer, StreamConsumer};
use rdkafka::error::KafkaResult;

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
