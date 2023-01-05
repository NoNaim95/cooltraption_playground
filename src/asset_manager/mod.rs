pub mod file_asset_bundle;

use bevy_ecs::system::Resource;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

pub trait AssetBundle: Resource {
    fn get_asset<T>(&self, name: T) -> Option<&Asset>
    where
        T: AsRef<str>;
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Asset {
    Texture { path: String },
    Strings(BTreeMap<String, String>),
}
