use super::physics_stage::{DeltaTime, Vec2f};
use crate::{Acceleration, Actions, PhysicsBundle, Position, Velocity};
use bevy_ecs::{
    schedule::StageLabel,
    system::{Commands, Query, Res},
};
use fixed::traits::ToFixed;

#[derive(StageLabel)]
pub struct ActionStage;

pub fn apply_spawn_ball_action(actions: Res<Actions>, mut commands: Commands) {
    for action in &actions.0 {
        if let crate::action::Action::SpawnBall(spawn_ball_action) = action {
            commands.spawn(PhysicsBundle {
                acc: Acceleration(Vec2f::new(0.to_fixed(), 0.to_fixed())),
                vel: Velocity(Vec2f::new(0.to_fixed(), 0.to_fixed())),
                pos: spawn_ball_action.position,
            });
        }
    }
}

pub fn apply_outward_force_action(
    mut query: Query<(&mut Position, &mut Velocity, &mut Acceleration)>,
    actions: Res<Actions>,
    dt: Res<DeltaTime>,
) {
    for action in &actions.0 {
        if let crate::action::Action::OutwardForce(outward_force) = action {
            for (pos, mut vel, mut acc) in (&mut query).into_iter() {
                vel.0 = (pos.0 - outward_force.position.0) * outward_force.strength * dt.seconds;
                acc.0 = -(pos.0 - outward_force.position.0) * outward_force.strength * 2.to_fixed() * dt.seconds;
            }
        }
    }
}
