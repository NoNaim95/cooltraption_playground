use std::collections::{BTreeMap, HashMap};
use std::error::Error;
use std::fmt::Debug;
use std::hash::Hash;

use crate::render::texture_atlas::texture_atlas_builder::TextureAtlasBuilder;
use as_any::AsAny;
use serde::{Deserialize, Serialize};

pub mod file_asset_loader;
pub mod strings_asset;
pub mod texture_asset;

pub struct AssetBundle<Id: Eq + Hash> {
    assets: HashMap<Id, Box<dyn Asset>>,
}

impl<Id: Eq + Hash> AssetBundle<Id> {
    pub fn get_asset<A: Asset>(&self, id: &Id) -> Option<&A> {
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

pub trait LoadAssetBundle<Id: Eq + Hash, E: Error> {
    fn load(&self, atlas_builder: &mut TextureAtlasBuilder) -> Result<AssetBundle<Id>, E>;
}
