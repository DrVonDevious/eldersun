use std::time::Duration;

use bevy::prelude::*;

#[derive(Component)]
pub struct ActionQueue {
    pub timer: Timer,
    pub interval: f32,
    pub ready: bool,
}

impl Default for ActionQueue {
    fn default() -> Self {
        Self::with_interval(0.5)
    }
}

#[allow(dead_code)]
impl ActionQueue {
    pub fn next(&mut self) {
        self.timer.set_duration(Duration::from_secs_f32(self.interval));
        self.ready = false;
    }

    pub fn with_interval(interval: f32) -> Self {
        Self {
            timer: Timer::from_seconds(interval, TimerMode::Once),
            interval,
            ready: true,
        }
    }
}