use observability::propagation;

#[allow(dead_code)]
pub fn inject() {
    propagation::inject()
}

#[allow(dead_code)]
pub fn extract() {
    propagation::extract()
}

