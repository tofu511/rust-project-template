#[cfg(feature = "outbound-service-foo")]
pub struct FooClient;

#[cfg(feature = "outbound-service-foo")]
impl FooClient {
    pub fn new() -> Self {
        Self
    }
    #[allow(dead_code)]
    pub async fn call(&self) -> Result<(), ()> {
        Ok(())
    }
}

