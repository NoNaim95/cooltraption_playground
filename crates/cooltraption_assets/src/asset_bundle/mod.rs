use std::collections::{BTreeMap, HashMap};
use std::error::Error;
use std::fmt::Debug;

use serde::{Deserialize, Serialize};

use crate::texture_atlas::*;
pub use file_asset_loader::*;
pub use sprite_asset::*;
pub use strings_asset::*;

mod file_asset_loader;
mod sprite_asset;
mod strings_asset;

pub struct AssetBundle {
    assets: HashMap<String, Asset>,
}

impl AssetBundle {
    pub fn get_asset(&self, id: &str) -> Option<&Asset> {
        if cfg!(feature = "missing") {
            self.assets.get(id).or_else(|| {
                // if asset does not exist display missing texture
                self.assets.get("missing")
            })
        } else {
            self.assets.get(id)
        }
    }
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
enum AssetConfig {
    Sprite(String),
    Strings(BTreeMap<String, String>),
}

#[derive(Debug)]
pub enum Asset {
    Sprite(SpriteAsset),
    Strings(StringsAsset),
}

pub trait LoadAssetBundle<E: Error> {
    fn load(&self, atlas_builder: &mut TextureAtlasBuilder) -> Result<AssetBundle, E>;
}
