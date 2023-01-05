pub mod file_asset_manager;

use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::path::Path;

pub trait AssetManager {
    fn load<T>(dir: T) -> Self
    where
        T: AsRef<Path>;

    fn get_asset<T>(&self, name: T) -> Option<&Asset>
    where
        T: AsRef<str>;
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Asset {
    Texture { path: String },
    Strings(BTreeMap<String, String>),
}
