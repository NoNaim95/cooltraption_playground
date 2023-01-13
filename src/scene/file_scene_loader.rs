use std::path::{Path, PathBuf};

use bevy_ecs::world::World;
use log::info;

use crate::asset_bundle::file_asset_bundle::{FileAssetBundle, LoadAssetError};
use crate::components::{Acceleration, Position, Velocity};
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
    fn load(&self, state: &WgpuState) -> Result<SceneImpl, LoadSceneError> {
        info!(
            "Mock file loader used to load {}",
            self.path.to_str().unwrap_or("None")
        );

        // if let Ok(file_content) = fs::read_to_string(&self.path) {
        let assets_path = &self.path.join(PathBuf::from("assets/"));
        let assets = FileAssetBundle::load(assets_path, state)?;

        let mut world = World::new();
        //world.insert_resource(assets);

        //let deserialized_map: BTreeMap<String> = serde_yaml::from_str(&yaml)?;

        let ent = world
            .spawn((
                Acceleration::default(),
                Velocity::default(),
                Position::default(),
            ))
            .id();
        let mut ent_mut = world.get_entity_mut(ent).unwrap();
        let mut vel = ent_mut.get_mut::<Velocity>().unwrap();
        vel.0.x = Float::from_num(3.0);
        vel.0.y = Float::from_num(1.0);

        Ok(SceneImpl { world, assets })

        /*let registration = TypeRegistration::of();
        let registry = TypeRegistry::new();
        let deserializer = TypedReflectDeserializer::new(&registration, &registry);

        let de = serde_yaml::Deserializer::from_str(&file_content);
        let world: &mut World = deserializer.deserialize(de).unwrap().take().unwrap();*/
        // }
    }
}
