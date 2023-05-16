use crate::action::Action;
use crate::physics_set::Float;
use crate::physics_set::{DeltaTime, FromNum2, FromNum4, Mat2f, Vec2f};
use crate::{
    action::CircularForceAction, Acceleration, Actions, PhysicsBundle, Position, Velocity,
};
use bevy_ecs::system::{Commands, Query, Res};


pub fn apply_spawn_ball_action(actions: Res<Actions>, mut commands: Commands) {
    for action in &actions.0 {
        if let Action::SpawnBall(spawn_ball_action) = action {
            commands.spawn(PhysicsBundle {
                acc: Acceleration(Vec2f::from_num(0, 0)),
                vel: Velocity(Vec2f::from_num(0, 0)),
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
        if let Action::OutwardForce(outward_force) = action {
            for (pos, mut vel, mut acc) in (&mut query).into_iter() {
                vel.0 = (pos.0 - outward_force.position.0) * outward_force.strength * dt.seconds();
            }
        }
    }
}

pub fn apply_circular_force_action(
    mut query: Query<(&mut Position, &mut Velocity, &mut Acceleration)>,
    actions: Res<Actions>,
    _dt: Res<DeltaTime>,
) {
    for action in &actions.0 {
        if let Action::CircularForce(circular_force) = action {
            let CircularForceAction { position, strength: _ } = *circular_force;

            for (pos, mut vel, _acc) in (&mut query).into_iter() {
                vel.0 = Mat2f::from_num(0, 1, -1, 0) * (position.0 - pos.0);
            }
        }
    }
}
