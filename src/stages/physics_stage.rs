use crate::components::{Acceleration, Position, Velocity};
use bevy_ecs::{prelude::*, system::Query};
use std::time::Duration;

use fixed::types::I48F16;
use fixed_macro::fixed;
use log::debug;
use nalgebra::Vector2;

pub type Float = I48F16;
pub type Vec2f = Vector2<Float>;

pub const MILLIS_TO_SECONDS: Float = fixed!(0.001: I48F16); // µs to s factor

#[derive(Resource, Default)]
pub struct DeltaTime {
    pub seconds: Float,
}

impl From<Duration> for DeltaTime {
    fn from(duration: Duration) -> Self {
        let ret = Self {
            seconds: (Float::from_num(duration.as_millis()) * MILLIS_TO_SECONDS),
        };
        return ret;
    }
}

#[derive(StageLabel)]
pub struct PhysicsStage;

pub fn solve_movement(
    mut query: Query<(&mut Position, &mut Velocity, &mut Acceleration)>,
    dt: Res<DeltaTime>,
) {
    for (mut pos, mut vel, acc) in &mut query {
        vel.0 += acc.0 * dt.seconds;
        pos.0 += vel.0 * dt.seconds;
    }
}
