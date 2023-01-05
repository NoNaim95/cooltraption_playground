use crate::asset_manager::{Asset, AssetBundle};
use bevy_ecs::prelude::Resource;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

#[derive(Resource)]
pub struct FileAssetBundle {
    assets: HashMap<String, Asset>,
}

impl FileAssetBundle {
    pub fn load<T>(dir: T) -> Self
    where
        T: AsRef<Path>,
    {
        let mut bundle = FileAssetBundle {
            assets: HashMap::new(),
        };

        let dir = dir.as_ref();
        if dir.is_dir() {
            for file in fs::read_dir(dir).unwrap().flatten() {
                let file_content = fs::read_to_string(file.path()).unwrap();
                let asset: Asset = serde_yaml::from_str(file_content.as_str()).unwrap();
                let asset_name = file.path();
                let asset_name = asset_name.file_stem().unwrap().to_str().unwrap();
                bundle.assets.insert(asset_name.to_owned(), asset);
            }
        }

        bundle
    }
}

impl AssetBundle for FileAssetBundle {
    fn get_asset<T>(&self, name: T) -> Option<&Asset>
    where
        T: AsRef<str>,
    {
        return self.assets.get(name.as_ref());
    }
}
