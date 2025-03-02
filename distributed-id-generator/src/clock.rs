use std::time::Duration;

pub trait Clock: Send {
    fn now(&self) -> Duration;
}

// Real clock implementation
pub struct SystemClock;

impl SystemClock {
    pub fn new() -> Self {
        Self {}
    }
}

impl Clock for SystemClock {
    fn now(&self) -> Duration {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("Failed to get current time")
    }
}

#[cfg(test)]
pub struct MockClock {
    current_time: Duration,
}

#[cfg(test)]
impl MockClock {
    pub fn new(initial_time: Duration) -> Self {
        Self {
            current_time: initial_time,
        }
    }

    pub fn advance(&mut self, duration: Duration) {
        self.current_time += duration;
    }
}

#[cfg(test)]
impl Clock for MockClock {
    fn now(&self) -> Duration {
        self.current_time
    }
}
