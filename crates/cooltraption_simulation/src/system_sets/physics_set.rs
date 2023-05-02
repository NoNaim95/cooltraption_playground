use std::time::Duration;

use bevy_ecs::{prelude::*, system::Query};
use fixed::traits::ToFixed;
pub use fixed::types::extra::U16;
pub use fixed::types::I48F16 as FixedI48F16;
use nalgebra::{Matrix2, Vector2};

use simba::scalar::FixedI48F16 as I48F16;

use crate::components::{Acceleration, Position, Velocity};
use derive_more::{Deref, From};

pub type Float = I48F16;
pub type Vec2f = Vector2<Float>;
pub type Mat2f = Matrix2<Float>;

pub trait FromNum2 {
    fn from_num<T: ToFixed>(a: T, b: T) -> Self;
}
impl FromNum2 for Vec2f {
    fn from_num<T: ToFixed>(a: T, b: T) -> Vec2f {
        Self::new(Float::from_num(a), Float::from_num(b))
    }
}

pub trait FromNum4 {
    fn from_num<T: ToFixed>(a: T, b: T, c: T, d: T) -> Self;
}

impl FromNum4 for Mat2f {
    fn from_num<T: ToFixed>(a: T, b: T, c: T, d: T) -> Mat2f {
        Self::new(
            Float::from_num(a),
            Float::from_num(b),
            Float::from_num(c),
            Float::from_num(d),
        )
    }
}

#[derive(Resource, Default, Deref, From)]
pub struct DeltaTime(Duration);
impl DeltaTime {
    pub fn seconds(&self) -> Float {
        Float::from_num(self.0.as_secs_f64())
    }

    pub fn milliseconds(&self) -> Float {
        Float::from_num(self.0.as_micros() / 1000)
    }

    pub fn nanoseconds(&self) -> u128 {
        self.0.as_nanos()
    }
}

pub fn solve_movement(
    mut query: Query<(&mut Position, &mut Velocity, &mut Acceleration)>,
    dt: Res<DeltaTime>,
) {
    for (mut pos, mut vel, acc) in &mut query {
        vel.0 += acc.0 * dt.seconds();
        pos.0 += vel.0 * dt.seconds();
    }
}

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub enum PhysicsSet {
    Movement,
    CollisionDetection,
}
