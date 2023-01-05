use bevy_ecs::{system::Query, prelude::*};
use glam::f64::DVec2;

use super::DeltaTime;

#[derive(Component, Default)]
pub struct Position(DVec2);

#[derive(Component, Default)]
pub struct Velocity(pub DVec2);

#[derive(Component, Default)]
pub struct Acceleration(pub DVec2);

#[derive(Component)]
pub struct Weight(f64);

#[derive(Component)]
pub struct Force(f64);


pub fn solve_movement(mut query: Query<(&mut Position, &mut Velocity, &mut Acceleration)>, dt: Res<DeltaTime>){
    for (mut pos, mut vel, acc) in &mut query{
        vel.0 += acc.0 * dt.seconds;
        pos.0 += vel.0 * dt.seconds;
        println!("Position of Entity: ({}|{})", pos.0.x, pos.0.y);
    }
}
