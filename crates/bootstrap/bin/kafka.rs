#[cfg(feature = "kafka")]
fn main() {
    observability::init::setup_from_env();
    if let Err(e) = bootstrap::run_kafka(()) {
        eprintln!("kafka failed: {e}");
        std::process::exit(1);
    }
}

#[cfg(not(feature = "kafka"))]
fn main() {
    // This binary requires the `kafka` feature.
    eprintln!("Enable bootstrap feature: kafka");
    std::process::exit(1);
}

