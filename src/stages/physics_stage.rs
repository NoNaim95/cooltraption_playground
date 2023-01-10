use crate::components::{Acceleration, Position, Velocity};
use bevy_ecs::{prelude::*, system::Query};
use log::debug;

use crate::runtime::DeltaTime;

#[derive(StageLabel)]
pub struct PhysicsStage;

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
