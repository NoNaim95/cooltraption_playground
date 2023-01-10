use bevy_ecs::prelude::*;
use std::ops::Deref;

use glam::DVec2;

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

#[derive(Component)]
pub struct Render {
    pub asset: String,
}

impl Deref for Position {
    type Target = DVec2;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
