use cgmath::{InnerSpace, Vector2, VectorSpace};

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
            rot: Rotation(self.rot.0.lerp(other.rot.0, amount).normalize()),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Position(pub Vector2<f32>);

impl Default for Position {
    fn default() -> Self {
        Self(Vector2::new(0.0, 0.0))
    }
}

#[derive(Clone, Debug)]
pub struct Rotation(pub Vector2<f32>);

impl Default for Rotation {
    fn default() -> Self {
        Self(Vector2::new(1.0, 0.0))
    }
}

#[derive(Clone, Debug)]
pub struct Scale(pub Vector2<f32>);

impl Default for Scale {
    fn default() -> Self {
        Self(Vector2::new(1.0, 1.0))
    }
}

#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq)]
pub struct Id(pub u64);
