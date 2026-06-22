use std::time::Duration;

pub struct CircuitBreaker {
    failure_threshold: u32,
    recovery_timeout: Duration,
}

impl CircuitBreaker {
    pub fn new(failure_threshold: u32, recovery_timeout: Duration) -> Self {
        Self { failure_threshold, recovery_timeout }
    }
    
    pub fn call<F, T>(&self, _f: F) -> Result<T, Box<dyn std::error::Error>>
    where F: FnOnce() -> Result<T, Box<dyn std::error::Error>> {
        _f()
    }
}
