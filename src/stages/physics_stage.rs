use crate::runtime::DeltaTime;
use bevy_ecs::{prelude::*, system::Query};
use glam::f64::DVec2;
use log::debug;
use std::ops::Deref;

#[derive(StageLabel)]
pub struct PhysicsStage;

#[derive(Component, Default)]
pub struct Position(pub DVec2);

#[derive(Component, Default)]
pub struct Velocity(pub DVec2);

#[derive(Component, Default)]
pub struct Acceleration(pub DVec2);

#[derive(Component)]
pub struct Weight(pub f64);

#[derive(Component)]
pub struct Force(pub f64);

impl Deref for Position {
    type Target = DVec2;

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
        debug!("Position of Entity: ({}|{})", pos.0.x, pos.0.y);
    }
}
