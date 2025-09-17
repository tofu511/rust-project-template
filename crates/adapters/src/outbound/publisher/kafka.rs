#[cfg(feature = "outbound-publisher-kafka")]
#[allow(dead_code)]
pub struct KafkaPublisher;

#[cfg(feature = "outbound-publisher-kafka")]
impl KafkaPublisher {
    pub fn new() -> Self {
        Self
    }
    #[allow(dead_code)]
    pub async fn publish(&self) -> Result<(), ()> {
        Ok(())
    }
}

