use std::collections::HashMap;
use std::fs;
use std::fs::DirEntry;
use std::path::Path;

use crate::asset_bundle::strings_asset::StringsAsset;
use crate::asset_bundle::texture_asset::{LoadTextureError, TextureAsset};
use crate::asset_bundle::*;
use crate::render::wgpu_state::WgpuState;

#[derive(Debug)]
pub enum LoadAssetError {
    IOError(std::io::Error),
    ParseError(serde_yaml::Error),
    PathError,
    TextureError(LoadTextureError),
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

impl From<LoadTextureError> for LoadAssetError {
    fn from(e: LoadTextureError) -> Self {
        LoadAssetError::TextureError(e)
    }
}

pub struct FileAssetBundle {
    assets: HashMap<String, Box<dyn Asset>>,
}

impl FileAssetBundle {
    pub fn load<T>(dir: T, state: &WgpuState) -> Result<Self, LoadAssetError>
    where
        T: AsRef<Path>,
    {
        let mut bundle = FileAssetBundle {
            assets: HashMap::new(),
        };

        let dir = dir.as_ref();
        if dir.is_dir() {
            for file in fs::read_dir(dir)?.flat_map(|r| r.ok()) {
                let file_content = fs::read_to_string(file.path())?;
                let asset_config: AssetConfig = serde_yaml::from_str(file_content.as_str())?;
                let asset_name = file_stem(&file).ok_or(LoadAssetError::PathError)?;

                let asset: Box<dyn Asset> = match asset_config {
                    AssetConfig::Texture(path) => Box::new(TextureAsset::load(path, state)?),
                    AssetConfig::Strings(map) => Box::new(StringsAsset::from(map)),
                };

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
    type AssetId = String;

    fn get_asset<T: Into<Self::AssetId>, A: Asset>(&self, id: T) -> Option<&A> {
        let asset = self.assets.get(&id.into())?.as_ref();
        asset.as_any().downcast_ref()
    }
}
