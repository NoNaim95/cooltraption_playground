use std::collections::BTreeMap;
use std::fmt::Debug;
use std::path::PathBuf;

use as_any::AsAny;
use serde::{Deserialize, Serialize};

pub mod file_asset_bundle;
pub mod strings_asset;
pub mod texture_asset;

pub trait AssetBundle {
    type AssetId;

    fn get_asset<T: Into<Self::AssetId>, A: Asset>(&self, id: T) -> Option<&A>;
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
enum AssetConfig {
    Texture(String),
    Strings(BTreeMap<String, String>),
}

pub trait Asset: AsAny + Debug {}
