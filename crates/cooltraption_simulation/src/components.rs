use derive_more::{Add, Mul, Sub, Div, From, Into, AddAssign, Neg, Deref};

use bevy_ecs::prelude::*;

use crate::system_sets::physics_set::Vec2f;
use crate::system_sets::physics_set::FromNum2;

use serde::{Serialize, Deserialize};

#[derive(Component, Clone, Debug, Copy, Serialize, Deserialize, Deref, Add, Mul, Sub, Div, From, Into, AddAssign, Neg)]
pub struct Position(pub Vec2f);
impl Default for Position{
    fn default() -> Self {
        Position(Vec2f::from_num(0,0))
    }
}

#[derive(Component, Clone, Debug, Copy, Serialize, Deserialize, Deref, Add, Mul, Sub, Div, From, Into, AddAssign, Neg)]
pub struct Velocity(pub Vec2f);

#[derive(Component, Clone, Debug, Copy, Serialize, Deserialize, Deref, Add, Mul, Sub, Div, From, Into, AddAssign, Neg)]
pub struct Acceleration(pub Vec2f);
impl Default for Acceleration{
    fn default() -> Self {
        Acceleration(Vec2f::from_num(0,0))
    }
}

#[derive(Component, Default, Clone, Debug, Copy, Serialize, Deserialize, Deref, Add, Mul, Sub, Div, From, Into, AddAssign, Neg)]
pub struct Weight(pub f64);

#[derive(Component, Default, Clone, Debug, Copy, Serialize, Deserialize, Deref, Add, Mul, Sub, Div, From, Into, AddAssign, Neg)]
pub struct Force(pub f64);

#[derive(Component, Default, Clone, Debug, Serialize, Deserialize, Deref, From, Into)]
pub struct Drawable {
    pub asset: String,
}

#[derive(Bundle)]
pub struct PhysicsBundle {
    pub acc: Acceleration,
    pub vel: Velocity,
    pub pos: Position,
}
