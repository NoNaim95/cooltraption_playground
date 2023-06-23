use std::time::{Duration, Instant};

pub struct Time {
    fixed_delta_time: f32,
    last_tick: Instant,
}

impl Time {
    pub fn new(fixed_delta_time: Duration) -> Self {
        Self {
            fixed_delta_time: fixed_delta_time.as_secs_f32(),
            last_tick: Instant::now(),
        }
    }

    pub fn tick(&mut self) {
        self.last_tick = Instant::now();
    }

    pub fn alpha(&self) -> f32 {
        (Instant::now() - self.last_tick).as_secs_f32() / self.fixed_delta_time
    }
}
