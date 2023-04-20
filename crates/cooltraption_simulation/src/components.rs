use std::ops::Deref;

use bevy_ecs::prelude::*;

use crate::stages::physics_stage::Vec2f;

#[derive(Component, Default, Clone, Debug, Copy)]
pub struct Position(pub Vec2f);

#[derive(Component, Default, Clone, Debug)]
pub struct Velocity(pub Vec2f);

#[derive(Component, Default, Clone, Debug)]
pub struct Acceleration(pub Vec2f);

#[derive(Component, Clone)]
pub struct Weight(pub f64);

#[derive(Component, Clone)]
pub struct Force(pub f64);

#[derive(Component, Clone, Debug)]
pub struct Drawable {
    pub asset: String,
}

#[derive(Bundle)]
pub struct PhysicsBundle {
    pub acc: Acceleration,
    pub vel: Velocity,
    pub pos: Position,
}

impl Deref for Position {
    type Target = Vec2f;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
