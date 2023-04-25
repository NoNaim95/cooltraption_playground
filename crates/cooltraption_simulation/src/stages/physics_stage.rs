use std::time::Duration;

use bevy_ecs::{prelude::*, system::Query};
pub use fixed::types::I48F16;
pub use fixed::FixedI64;
pub use fixed::types::extra::U16;
use fixed_macro::fixed;
use nalgebra::Vector2;
use serde::Deserialize;

use crate::components::{Acceleration, Position, Velocity};

pub type Float = I48F16;
pub type Vec2f = Vector2<Float>;

pub const MILLIS_TO_SECONDS: Float = fixed!(0.001: I48F16); // ms to s factor

#[derive(Resource, Default)]
pub struct DeltaTime {
    pub seconds: Float,
}

impl From<Duration> for DeltaTime {
    fn from(duration: Duration) -> Self {
        Self {
            seconds: (Float::from_num(duration.as_millis()) * MILLIS_TO_SECONDS),
        }
    }
}

#[derive(StageLabel)]
pub struct PhysicsStage;

pub fn solve_movement(
    mut query: Query<(&mut Position, &mut Velocity, &mut Acceleration)>,
    dt: Res<DeltaTime>,
) {
    for (mut pos, mut vel, acc) in (&mut query).into_iter() {
        vel.0 += acc.0 * dt.seconds;
        pos.0 += vel.0 * dt.seconds;
    }
}
