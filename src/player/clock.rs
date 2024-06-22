use std::time::{Duration, Instant};

#[derive(Debug, Eq, PartialEq)]
pub enum ClockState {
    Stopped,
    Running,
}

#[derive(Debug, Clone)]
pub struct SimulationTicker {
    pub clock_speed: u64,
    pub clock_period: Duration,
    pub timer: Instant,
}
