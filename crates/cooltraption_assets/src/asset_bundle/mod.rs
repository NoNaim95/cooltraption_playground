use std::collections::{BTreeMap, HashMap};
use std::error::Error;
use std::fmt::Debug;

use as_any::AsAny;
use serde::{Deserialize, Serialize};

use crate::texture_atlas::*;
pub use file_asset_loader::*;
pub use sprite_asset::*;
pub use strings_asset::*;

mod file_asset_loader;
mod sprite_asset;
mod strings_asset;

pub struct AssetBundle {
    assets: HashMap<String, Box<dyn Asset>>,
}

impl AssetBundle {
    pub fn get_asset<A: Asset>(&self, id: &str) -> Option<&A> {
        let asset = self.assets.get(id)?.as_ref();
        asset.as_any().downcast_ref()
    }
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
enum AssetConfig {
    Sprite(String),
    Strings(BTreeMap<String, String>),
}

pub trait Asset: AsAny + Debug {}

pub trait LoadAssetBundle<E: Error> {
    fn load(&self, atlas_builder: &mut TextureAtlasBuilder) -> Result<AssetBundle, E>;
}
