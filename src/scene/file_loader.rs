use crate::assets::file_asset_bundle::FileAssetBundle;
use crate::scene::{Load, SceneImpl};
use crate::stages::physics_stage::{Acceleration, Position, Velocity};
use bevy_ecs::world::World;
use std::path::{Path, PathBuf};

pub struct FileLoader {
    path: PathBuf,
}

impl<T: AsRef<Path>> From<T> for FileLoader {
    fn from(path: T) -> Self {
        FileLoader {
            path: path.as_ref().to_path_buf(),
        }
    }
}

impl Load<SceneImpl> for FileLoader {
    fn load(&self) -> SceneImpl {
        // if let Ok(file_content) = fs::read_to_string(&self.path) {
        let assets = FileAssetBundle::load(&self.path.join(PathBuf::from("assets/")));

        let mut world = World::new();
        world.insert_resource(assets);

        let ent = world
            .spawn((
                Acceleration::default(),
                Velocity::default(),
                Position::default(),
            ))
            .id();
        let mut ent_mut = world.get_entity_mut(ent).unwrap();
        let mut vel = ent_mut.get_mut::<Velocity>().unwrap();
        vel.0.x = 3.0;
        vel.0.y = 1.0;

        SceneImpl { world }

        /*let registration = TypeRegistration::of();
        let registry = TypeRegistry::new();
        let deserializer = TypedReflectDeserializer::new(&registration, &registry);

        let de = serde_yaml::Deserializer::from_str(&file_content);
        let world: &mut World = deserializer.deserialize(de).unwrap().take().unwrap();*/
        // }
    }
}
