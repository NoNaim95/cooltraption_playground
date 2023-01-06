use std::collections::BTreeMap;

use bevy_ecs::system::Resource;
use serde::{Deserialize, Serialize};

pub mod file_asset_bundle;

pub trait AssetBundle: Resource {
    fn get_asset(&self, name: &str) -> Option<&Asset>;
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Asset {
    Texture { path: String },
    Strings(BTreeMap<String, String>),
}
