use std::ops::Deref;
use std::time::Duration;

use bevy_ecs::{prelude::*, system::Query};
use fixed::types::I48F16;
use fixed_macro::fixed;
use nalgebra::Vector2;


pub type float = I48F16;
pub type Vec2f = Vector2<float>;

pub const MILLIS_TO_SECONDS: float = fixed!(0.001: I48F16); // µs to s factor

#[derive(Resource, Default)]
pub struct DeltaTime {
    pub seconds: float,
}

impl From<Duration> for DeltaTime {
    fn from(duration: Duration) -> Self {
        let ret = Self {
            seconds: (float::from_num(duration.as_millis()) * MILLIS_TO_SECONDS),
        };
        return ret;
    }
}

#[derive(StageLabel)]
pub struct PhysicsStage;

#[derive(Component, Default)]
pub struct Position(pub Vec2f);

#[derive(Component, Default)]
pub struct Velocity(pub Vec2f);

#[derive(Component, Default)]
pub struct Acceleration(pub Vec2f);

#[derive(Component)]
pub struct Weight(pub float);

#[derive(Component)]
pub struct Force(pub float);

impl Deref for Position {
    type Target = Vec2f;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub fn solve_movement(
    mut query: Query<(&mut Position, &mut Velocity, &mut Acceleration)>,
    dt: Res<DeltaTime>,
) {
    for (mut pos, mut vel, acc) in &mut query {
        vel.0 += acc.0 * dt.seconds;
        pos.0 += vel.0 * dt.seconds;
    }
}
