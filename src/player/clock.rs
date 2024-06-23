use std::time::{Duration, Instant};

use crate::core::simulation::circuit::Circuit;
use crate::player::CirquilPlayerApp;

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
    pub tick_needed: bool,
}

impl SimulationTicker {
    pub fn request_tick(&mut self) {
        self.tick_needed = true;
    }

    pub fn check_tick_needed(&mut self) -> bool {
        let tick_needed = self.tick_needed;

        self.tick_needed = false;

        tick_needed
    }
}

impl CirquilPlayerApp {
    pub fn tick(&self, circuit: &Circuit) {
        circuit.tick();
        circuit.propagate_ticked();
    }
}
