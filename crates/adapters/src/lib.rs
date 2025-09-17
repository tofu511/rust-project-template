mod common {
    pub mod observability;
}

mod inbound {
    pub mod http;
    pub mod kafka;
    pub mod lambda;
}

mod outbound {
    pub mod repository {
        pub mod pg;
    }
    pub mod service;
    pub mod publisher;
}

#[cfg(feature = "inbound-http")]
pub fn build_http() {
    // Construct router/handlers, wire middleware, etc.
    // Intentionally minimal for scaffold.
}

#[cfg(feature = "inbound-kafka")]
pub fn build_kafka() {
    // Construct consumer/streams; minimal scaffold.
}

#[cfg(feature = "inbound-lambda")]
pub fn build_lambda() {
    // Construct lambda handler entrypoint(s); minimal scaffold.
}

