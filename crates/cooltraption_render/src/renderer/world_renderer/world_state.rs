use cgmath::{InnerSpace, Vector2, VectorSpace};
use cooltraption_assets::asset_bundle::AssetBundle;
use cooltraption_assets::texture_atlas::TextureAtlas;
use std::collections::BTreeMap;

use super::RenderEntity;

const NEW: usize = 0;
const OLD: usize = 1;

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

#[derive(Debug, Default)]
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

#[derive(Default)]
pub struct WorldState {
    pub(crate) drawables: [BTreeMap<Id, Drawable>; 2],
}

impl WorldState {
    pub fn get_render_entities(
        &self,
        amount: f32,
        texture_atlas: &TextureAtlas,
        assets: &AssetBundle,
    ) -> Vec<RenderEntity> {
        let render_entities = self.drawables[NEW]
            .values()
            .filter_map(|drawable| {
                if let Some(old_drawable) = self.drawables[OLD].get(&drawable.id) {
                    RenderEntity::try_from(
                        &old_drawable.transform.lerp(&drawable.transform, amount),
                        &drawable.asset_name,
                        texture_atlas,
                        assets,
                    )
                } else {
                    RenderEntity::try_from(
                        &drawable.transform,
                        &drawable.asset_name,
                        texture_atlas,
                        assets,
                    )
                }
            })
            .collect();

        render_entities
    }

    pub fn update(&mut self, drawables: Vec<Drawable>) {
        self.drawables.swap(NEW, OLD);
        self.drawables[NEW] = BTreeMap::from_iter(drawables.into_iter().map(|d| (d.id, d)));
    }
}
