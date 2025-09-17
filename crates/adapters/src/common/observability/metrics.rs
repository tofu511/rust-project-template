use observability::telemetry;

#[allow(dead_code)]
pub fn requests_counter() {
    let c = telemetry::counter("http.requests.total");
    c.inc_by(1);
}

