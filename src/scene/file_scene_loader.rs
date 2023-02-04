use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use std::path::{Path, PathBuf};

use bevy_ecs::world::World;
use log::{info, warn};

use crate::asset_bundle::file_asset_bundle::{FileAssetBundle, LoadAssetError};
use crate::components::{Acceleration, Drawable, Position, Velocity};
use crate::render::wgpu_state::WgpuState;
use crate::scene::{LoadScene, SceneImpl};
use crate::stages::physics_stage::Float;

pub struct MockFileSceneLoader {
    path: PathBuf,
}

#[derive(Debug)]
pub enum LoadSceneError {
    IOError(std::io::Error),
    AssetError(LoadAssetError),
}

impl Display for LoadSceneError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            LoadSceneError::IOError(e) => write!(f, "IO error occurred during scene load: {}", e),
            LoadSceneError::AssetError(e) => {
                write!(f, "an asset returned an error during loading {}", e)
            }
        }
    }
}

impl Error for LoadSceneError {}

impl From<std::io::Error> for LoadSceneError {
    fn from(e: std::io::Error) -> Self {
        LoadSceneError::IOError(e)
    }
}

impl From<LoadAssetError> for LoadSceneError {
    fn from(e: LoadAssetError) -> Self {
        LoadSceneError::AssetError(e)
    }
}

impl<T: AsRef<Path>> From<T> for MockFileSceneLoader {
    fn from(path: T) -> Self {
        MockFileSceneLoader {
            path: path.as_ref().to_path_buf(),
        }
    }
}

impl LoadScene<SceneImpl, LoadSceneError> for MockFileSceneLoader {
    fn load(&self, state: &mut WgpuState) -> Result<SceneImpl, LoadSceneError> {
        warn!(
            "Mock file loader used to load {}",
            self.path.to_str().unwrap_or("None")
        );

        // if let Ok(file_content) = fs::read_to_string(&self.path) {
        //let assets_path = &self.path.join(PathBuf::from("assets/"));
        // TODO: Load from scene path; ^ will do v is just to debug
        let assets_path = PathBuf::from("./assets/");
        let assets = FileAssetBundle::load(assets_path, state)?;
        //let assets = FileAssetBundle::load("./assets", state)?;

        let mut world = World::new();
        //world.insert_resource(assets);

        //let deserialized_map: BTreeMap<String> = serde_yaml::from_str(&yaml)?;

        let ent = world
            .spawn((
                Acceleration::default(),
                Velocity::default(),
                Position::default(),
                Drawable {
                    asset: "test".to_string(),
                },
            ))
            .id();
        let mut ent_mut = world.get_entity_mut(ent).unwrap();
        let mut vel = ent_mut.get_mut::<Velocity>().unwrap();
        vel.0.x = Float::from_num(0.3);
        vel.0.y = Float::from_num(0.1);

        Ok(SceneImpl { world, assets })

        /*let registration = TypeRegistration::of();
        let registry = TypeRegistry::new();
        let deserializer = TypedReflectDeserializer::new(&registration, &registry);

        let de = serde_yaml::Deserializer::from_str(&file_content);
        let world: &mut World = deserializer.deserialize(de).unwrap().take().unwrap();*/
        // }
    }
}
