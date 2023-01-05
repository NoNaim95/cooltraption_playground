use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, HashMap};
use std::fs;
use std::path::Path;

pub trait AssetManager {
    fn load<T>(dir: T) -> Self
    where
        T: AsRef<Path>;

    fn get_asset<T>(&self, name: T) -> Option<&Asset>
    where
        T: AsRef<str>;
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum Asset {
    Texture { path: String },
    Strings(BTreeMap<String, String>),
}

pub struct FileAssetManager {
    assets: HashMap<String, Asset>,
}

impl AssetManager for FileAssetManager {
    fn load<T>(dir: T) -> Self
    where
        T: AsRef<Path>,
    {
        let mut manager = FileAssetManager {
            assets: HashMap::new(),
        };

        let dir = dir.as_ref();
        if dir.is_dir() {
            for file in fs::read_dir(dir).unwrap().flatten() {
                let file_content = fs::read_to_string(file.path()).unwrap();
                let asset: Asset = serde_yaml::from_str(file_content.as_str()).unwrap();
                let asset_name = file.file_name();
                let asset_name = asset_name.to_str().unwrap();
                manager.assets.insert(asset_name.to_owned(), asset);
            }
        }

        manager
    }

    fn get_asset<T>(&self, name: T) -> Option<&Asset>
    where
        T: AsRef<str>,
    {
        return self.assets.get(name.as_ref());
    }
}
