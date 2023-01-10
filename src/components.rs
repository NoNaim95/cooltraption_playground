use bevy_ecs::prelude::*;
use std::ops::Deref;

use glam::DVec2;

#[derive(Component, Default, Clone)]
pub struct Position(pub DVec2);

#[derive(Component, Default, Clone)]
pub struct Velocity(pub DVec2);

#[derive(Component, Default, Clone)]
pub struct Acceleration(pub DVec2);

#[derive(Component, Clone)]
pub struct Weight(pub f64);

#[derive(Component, Clone)]
pub struct Force(pub f64);

#[derive(Component, Clone)]
pub struct Drawable {
    pub asset: String,
}

impl Deref for Position {
    type Target = DVec2;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
