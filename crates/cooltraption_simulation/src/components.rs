use derive_more::{Add, AddAssign, Deref, Div, From, Into, Mul, Neg, Sub};

use bevy_ecs::prelude::*;

use crate::system_sets::physics_set::FromNum2;
use crate::system_sets::physics_set::Vec2f;

use serde::{Deserialize, Serialize};

#[rustfmt::skip]
#[derive(Component, Clone, Debug, Copy, Serialize, Deserialize, Deref, Add, Mul, Sub, Div, From, Into, AddAssign, Neg)]
pub struct Position(pub Vec2f);
impl Default for Position {
    fn default() -> Self {
        Position(Vec2f::from_num(0, 0))
    }
}

#[rustfmt::skip]
#[derive(Component, Clone, Debug, Copy, Serialize, Deserialize, Deref, Add, Mul, Sub, Div, From, Into, AddAssign, Neg)]
pub struct Velocity(pub Vec2f);

#[rustfmt::skip]
#[derive(Component, Clone, Debug, Copy, Serialize, Deserialize, Deref, Add, Mul, Sub, Div, From, Into, AddAssign, Neg)]
pub struct Acceleration(pub Vec2f);
impl Default for Acceleration {
    fn default() -> Self {
        Acceleration(Vec2f::from_num(0, 0))
    }
}

#[rustfmt::skip]
#[derive(Component, Default, Clone, Debug, Copy, Serialize, Deserialize, Deref, Add, Mul, Sub, Div, From, Into, AddAssign, Neg)]
pub struct Weight(pub f64);

#[rustfmt::skip]
#[derive(Component, Default, Clone, Debug, Copy, Serialize, Deserialize, Deref, Add, Mul, Sub, Div, From, Into, AddAssign, Neg)]
pub struct Force(pub f64);

#[rustfmt::skip]
#[derive(Component, Default, Clone, Debug, Serialize, Deserialize, Deref, From, Into)]
pub struct Drawable {
    pub asset: String,
}

#[rustfmt::skip]
#[derive(Bundle)]
pub struct PhysicsBundle {
    pub acc: Acceleration,
    pub vel: Velocity,
    pub pos: Position,
}
