use std::collections::HashMap;
use std::fs;
use std::fs::DirEntry;
use std::path::Path;

use bevy_ecs::prelude::Resource;

use crate::asset_bundle::{Asset, AssetBundle};

pub enum LoadAssetError {
    IOError(std::io::Error),
    ParseError(serde_yaml::Error),
    PathError,
}

impl From<std::io::Error> for LoadAssetError {
    fn from(e: std::io::Error) -> Self {
        LoadAssetError::IOError(e)
    }
}

impl From<serde_yaml::Error> for LoadAssetError {
    fn from(e: serde_yaml::Error) -> Self {
        LoadAssetError::ParseError(e)
    }
}

#[derive(Resource)]
pub struct FileAssetBundle {
    assets: HashMap<String, Asset>,
}

impl FileAssetBundle {
    pub fn load<T>(dir: T) -> Result<Self, LoadAssetError>
    where
        T: AsRef<Path>,
    {
        let mut bundle = FileAssetBundle {
            assets: HashMap::new(),
        };

        let dir = dir.as_ref();
        if dir.is_dir() {
            for file in fs::read_dir(dir)?.flatten() {
                let file_content = fs::read_to_string(file.path())?;
                let asset: Asset = serde_yaml::from_str(file_content.as_str())?;
                let asset_name = file_stem(&file).ok_or(LoadAssetError::PathError)?;
                bundle.assets.insert(asset_name.to_owned(), asset);
            }
        }

        Ok(bundle)
    }
}

fn file_stem(file: &DirEntry) -> Option<String> {
    let path = file.path();
    Some(String::from(path.file_stem()?.to_str()?))
}

impl AssetBundle for FileAssetBundle {
    fn get_asset(&self, name: &str) -> Option<&Asset> {
        self.assets.get(name)
    }
}
