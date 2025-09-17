pub struct UsecaseSpan;

impl Drop for UsecaseSpan {
    fn drop(&mut self) {
        // No-op end span
    }
}

pub fn usecase_span(_name: &str) -> UsecaseSpan {
    // No-op start span
    UsecaseSpan
}

pub struct Counter(&'static str);

impl Counter {
    pub fn inc_by(&self, _v: u64) {
        // No-op
    }
}

pub fn counter(name: &'static str) -> Counter {
    Counter(name)
}

