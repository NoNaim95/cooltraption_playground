use cgmath::{Quaternion, Vector2, Vector3};
use num_traits::Zero;

use crate::asset_bundle::AssetBundle;
use crate::asset_bundle::texture_asset::TextureAsset;
use crate::render::instance::Instance;
use crate::render::texture_atlas::TextureAtlas;

#[derive(Clone, Debug)]
pub struct Position(pub Vector2<f32>);

impl Default for Position {
    fn default() -> Self {
        Self(Vector2::new(0.0, 0.0))
    }
}

#[derive(Debug)]
pub struct Drawable {
    pub id: u64,
    pub position: Position,
    pub asset_name: String,
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
    ) -> Vec<Instance> {
        new.drawables
            .iter()
            .filter_map(|d| {
                let asset = assets
                    .get_asset::<TextureAsset>(&d.asset_name)
                    .or_else(|| {
                        // if asset does not exist display missing texture
                        assets.get_asset::<TextureAsset>("missing")
                    })?;
                let atlas_region = *texture_atlas.get_texture_region(asset.texture_hash)?;

                Some(Instance {
                    position: Vector3::new(d.position.0.x, d.position.0.y, 0.0),
                    rotation: Quaternion::zero(),
                    atlas_region,
                })
            })
            .collect()
    }
}
