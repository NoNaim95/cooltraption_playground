use cgmath::{Deg, Quaternion, Rotation3, Vector2, Vector3};
use cooltraption_assets::asset_bundle::{Asset, AssetBundle};
use cooltraption_assets::texture_atlas::TextureAtlas;

use super::RenderEntity;

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

#[derive(Copy, Clone, Debug)]
pub struct Id(pub u64);

#[derive(Debug)]
pub struct Drawable {
    pub id: Id,
    pub position: Position,
    pub scale: Scale,
    pub rot: Rotation,
    pub asset_name: String,
}

impl Default for Drawable {
    fn default() -> Self {
        Self {
            id: Id(0),
            position: Default::default(),
            scale: Default::default(),
            rot: Default::default(),
            asset_name: "".to_string(),
        }
    }
}

#[derive(Default, Debug)]
pub struct WorldState {
    pub drawables: Vec<Drawable>,
}

impl WorldState {
    pub fn interpolate(
        &self,
        new: &WorldState,
        assets: &AssetBundle,
        texture_atlas: &TextureAtlas,
    ) -> Vec<RenderEntity> {
        new.drawables
            .iter()
            .filter_map(|d| {
                if let Some(Asset::Sprite(asset)) = assets.get_asset(&d.asset_name) {
                    let atlas_region = *texture_atlas.get_texture_region(asset.texture_hash)?;

                    Some(RenderEntity {
                        position: Vector3::new(d.position.0.x, d.position.0.y, 0.0),
                        scale: Vector3::new(d.scale.0.x, d.scale.0.y, 1.0),
                        rotation: Quaternion::from_angle_z(Deg(d.rot.0)),
                        atlas_region,
                    })
                } else {
                    None
                }
            })
            .collect()
    }
}
