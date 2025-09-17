#[cfg(feature = "inbound-kafka")]
pub mod consumer;
#[cfg(feature = "inbound-kafka")]
pub mod mapping;

#[cfg(feature = "inbound-kafka")]
#[allow(dead_code)]
pub fn start_consumer() {
    // TODO: start rdkafka consumer
}

