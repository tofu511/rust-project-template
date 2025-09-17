#[cfg(feature = "avro")]
#[derive(Debug, Clone)]
pub struct UserActivatedV1 {
    pub id: u64,
    pub at_epoch_ms: i64,
}

#[cfg(not(feature = "avro"))]
#[allow(dead_code)]
pub mod placeholder {
    // Enable the `avro` feature to compile concrete contract types.
}

