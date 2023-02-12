use log::debug;
use std::collections::HashMap;
use std::error::Error;
use std::ffi::OsStr;
use std::fmt::{Display, Formatter};
use std::fs;
use std::fs::DirEntry;
use std::path::PathBuf;

use crate::asset_bundle::strings_asset::StringsAsset;
use crate::asset_bundle::texture_asset::{LoadTextureError, TextureAsset};
use crate::asset_bundle::*;
use crate::render::texture_atlas::texture_atlas_builder::TextureAtlasBuilder;

#[derive(Debug)]
pub enum LoadAssetError {
    IOError(std::io::Error),
    ParseError(serde_yaml::Error),
    PathError(PathBuf),
    TextureError(LoadTextureError),
}

impl Display for LoadAssetError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            LoadAssetError::IOError(e) => write!(f, "io error occured during asset loading: {}", e),
            LoadAssetError::ParseError(e) => {
                write!(f, "could not parse content of asset file: {}", e)
            }
            LoadAssetError::PathError(p) => write!(f, "the asset path '{:?}' does not exist", p),
            LoadAssetError::TextureError(e) => write!(f, "texture could not be loaded: {}", e),
        }
    }
}

impl Error for LoadAssetError {}

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

pub struct FileAssetLoader {
    path: PathBuf,
}

impl FileAssetLoader {
    pub fn new(path: PathBuf) -> Self {
        Self { path }
    }
}

impl LoadAssetBundle<String, LoadAssetError> for FileAssetLoader {
    fn load(
        &self,
        atlas_builder: &mut TextureAtlasBuilder,
    ) -> Result<AssetBundle<String>, LoadAssetError> {
        debug!("Loading assets from {:?}", self.path);

        if self.path.is_dir() {
            let mut bundle = AssetBundle {
                assets: HashMap::new(),
            };

            // Insert missing.png texture which can be used by the renderer when get_asset returns None
            bundle
                .assets
                .insert("missing".to_string(), create_missing_asset(atlas_builder));

            // Load all yml files
            for file in fs::read_dir(&self.path)?.flat_map(|r| r.ok()).filter(|f| {
                return if let Some(ext) = f.path().extension() {
                    [OsStr::new("yml"), OsStr::new("yaml")].contains(&ext)
                } else {
                    false
                };
            }) {
                let file_content = fs::read_to_string(file.path())?;
                let asset_config: AssetConfig = serde_yaml::from_str(file_content.as_str())?;
                let asset_name =
                    file_stem(&file).ok_or_else(|| LoadAssetError::PathError(file.path()))?;

                let asset: Box<dyn Asset> = match asset_config {
                    AssetConfig::Texture(path) => {
                        let texture_path = file
                            .path()
                            .parent()
                            .ok_or_else(|| LoadAssetError::PathError(path.clone().into()))?
                            .join(path);
                        let bytes = fs::read(texture_path)?;
                        let texture = TextureAsset::decode(bytes.as_slice(), atlas_builder)?;
                        Box::new(texture)
                    }
                    AssetConfig::Strings(map) => Box::new(StringsAsset::from(map)),
                };

                debug!("Loaded asset {} {:?}", asset_name, asset);

                bundle.assets.insert(asset_name.to_owned(), asset);
            }

            return Ok(bundle);
        }

        Err(LoadAssetError::PathError(self.path.clone()))
    }
}

fn create_missing_asset(atlas_builder: &mut TextureAtlasBuilder) -> Box<dyn Asset> {
    let bytes = include_bytes!("missing.png");
    Box::new(
        TextureAsset::decode(bytes, atlas_builder)
            .expect("decode missing.png file which is used for debugging"),
    )
}

fn file_stem(file: &DirEntry) -> Option<String> {
    let path = file.path();
    Some(String::from(path.file_stem()?.to_str()?))
}
