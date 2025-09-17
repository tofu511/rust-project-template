#[cfg(feature = "http")]
pub fn run_http(_cfg: ()) -> Result<(), Box<dyn std::error::Error>> {
    adapters::build_http();
    Ok(())
}

#[cfg(feature = "kafka")]
pub fn run_kafka(_cfg: ()) -> Result<(), Box<dyn std::error::Error>> {
    adapters::build_kafka();
    Ok(())
}

