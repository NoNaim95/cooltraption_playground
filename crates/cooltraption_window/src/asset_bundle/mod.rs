use std::collections::{BTreeMap, HashMap};
use std::error::Error;
use std::fmt::Debug;

use as_any::AsAny;
use serde::{Deserialize, Serialize};

pub use file_asset_loader::*;
pub use strings_asset::*;
pub use texture_asset::*;
pub use texture_atlas::*;

mod file_asset_loader;
mod strings_asset;
mod texture_asset;
mod texture_atlas;

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
    Texture(String),
    Strings(BTreeMap<String, String>),
}

pub trait Asset: AsAny + Debug {}

pub trait LoadAssetBundle<E: Error> {
    fn load(&self, atlas_builder: &mut TextureAtlasBuilder) -> Result<AssetBundle, E>;
}
