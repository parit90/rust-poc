extern crate rdkafka;
use rdkafka::config::ClientConfig;
use rdkafka::error::KafkaError;
use rdkafka::producer::{FutureProducer, FutureRecord};
use rdkafka::util::Timeout;
use rdkafka::producer::Producer;

pub async fn create_kafka_producer() -> Result<FutureProducer, KafkaError> {
    // Create a Kafka producer with the specified configuration
    let producer: FutureProducer = ClientConfig::new()
        .set("bootstrap.servers", std::env::var("KAFKA_URL").unwrap_or_default()) // Replace with your Kafka broker(s)
        .create()?;

    Ok(producer)
}

pub async fn send_to_kafka(
    producer: &FutureProducer,
    topic: &str,
    payload: &[u8],
) -> Result<(), rdkafka::error::KafkaError> {
    // Create a FutureRecord using the builder pattern
    let record = FutureRecord::to(topic).key(payload).payload(payload);

    //println!("Record : ");
    // Create a Timeout value for the desired timeout duration
    // let timeout = Timeout::Never; // This sets the timeout to never expire
    let timeout = Timeout::After(std::time::Duration::from_secs(30)); // Set a 30-second timeout

    // Send the record using the send method with the Timeout
    match producer.send(record, timeout).await {
        Ok(owned_message) => {
            // Handle the sent message if needed
            //println!("Message sent successfully: ");
            Ok(()) // Return Ok as a result
        }
        Err((kafka_error, owned_message)) => {
            // Handle the Kafka error and the owned message
            //println!("Error sending message: {:?}", kafka_error);
            //println!("Failed message: {:?}", owned_message);
            Err(kafka_error) // Return the KafkaError as an error result
        }
    }
}
