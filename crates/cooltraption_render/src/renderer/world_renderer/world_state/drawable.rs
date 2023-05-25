use cgmath::{Vector2, VectorSpace};
use num_traits::FloatConst;

#[derive(Debug)]
pub struct Drawable {
    pub id: Id,
    pub transform: Transform,
    pub asset_name: String,
}

impl Default for Drawable {
    fn default() -> Self {
        Self {
            id: Id(0),
            transform: Transform::default(),
            asset_name: "".to_string(),
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct Transform {
    pub position: Position,
    pub scale: Scale,
    pub rot: Rotation,
}

impl Transform {
    pub fn new(position: Position, scale: Scale, rot: Rotation) -> Self {
        Self {
            position,
            scale,
            rot,
        }
    }

    pub fn lerp(&self, other: &Self, amount: f32) -> Self {
        Self {
            position: Position(self.position.0.lerp(other.position.0, amount)),
            scale: Scale(self.scale.0.lerp(other.scale.0, amount)),
            rot: Rotation(lerp_angle(self.rot.0, other.rot.0, amount)),
        }
    }
}

fn lerp_angle(a: f32, b: f32, t: f32) -> f32 {
    a + ((b - a + f32::PI()) % (2.0 * f32::PI()) - f32::PI()) * t
}

#[derive(Clone, Debug)]
pub struct Position(pub Vector2<f32>);

impl Default for Position {
    fn default() -> Self {
        Self(Vector2::new(0.0, 0.0))
    }
}

#[derive(Clone, Debug, Default)]
pub struct Rotation(pub f32);

#[derive(Clone, Debug)]
pub struct Scale(pub Vector2<f32>);

impl Default for Scale {
    fn default() -> Self {
        Self(Vector2::new(1.0, 1.0))
    }
}

#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq)]
pub struct Id(pub u64);
