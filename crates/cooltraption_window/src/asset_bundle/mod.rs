use std::collections::{BTreeMap, HashMap};
use std::error::Error;
use std::fmt::Debug;

use as_any::AsAny;
use serde::{Deserialize, Serialize};

use crate::render::instance_renderer::texture_atlas::TextureAtlasBuilder;

pub mod file_asset_loader;
pub mod strings_asset;
pub mod texture_asset;

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
