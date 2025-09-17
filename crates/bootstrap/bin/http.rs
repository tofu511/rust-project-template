#[cfg(feature = "http")]
fn main() {
    observability::init::setup_from_env();
    if let Err(e) = bootstrap::run_http(()) {
        eprintln!("http failed: {e}");
        std::process::exit(1);
    }
}

#[cfg(not(feature = "http"))]
fn main() {
    // This binary requires the `http` feature.
    eprintln!("Enable bootstrap feature: http");
    std::process::exit(1);
}

