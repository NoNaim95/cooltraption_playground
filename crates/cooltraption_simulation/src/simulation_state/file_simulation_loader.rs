use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use std::path::{Path, PathBuf};

use bevy_ecs::world::World;
use log::warn;

use crate::components::{Acceleration, Drawable, Position, Velocity};
use crate::simulation_state::{LoadSimulation, SimulationStateImpl};
use crate::stages::physics_stage::Float;

pub struct MockFileSimulationLoader {
    path: PathBuf,
}

#[derive(Debug)]
pub enum LoadSimulationError {
    IOError(std::io::Error),
}

impl Display for LoadSimulationError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            LoadSimulationError::IOError(e) => {
                write!(f, "IO error occurred during simulation load: {}", e)
            }
        }
    }
}

impl Error for LoadSimulationError {}

impl From<std::io::Error> for LoadSimulationError {
    fn from(e: std::io::Error) -> Self {
        LoadSimulationError::IOError(e)
    }
}

impl<T: AsRef<Path>> From<T> for MockFileSimulationLoader {
    fn from(path: T) -> Self {
        MockFileSimulationLoader {
            path: path.as_ref().to_path_buf(),
        }
    }
}

impl LoadSimulation<SimulationStateImpl, LoadSimulationError> for MockFileSimulationLoader {
    fn load(&self) -> Result<SimulationStateImpl, LoadSimulationError> {
        warn!(
            "Mock file loader used to load {}",
            self.path.to_str().unwrap_or("None")
        );

        // if let Ok(file_content) = fs::read_to_string(&self.path) {
        //let assets_path = &self.path.join(PathBuf::from("assets/"));
        // TODO: Load from simulation path; ^ will do v is just to debug
        /*let assets_path = PathBuf::from("./assets/");
        let assets = FileAssetBundle::load(assets_path, state)?;*/
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
                    asset: "texture".to_string(),
                },
            ))
            .id();
        let mut ent_mut = world.get_entity_mut(ent).unwrap();
        let mut vel = ent_mut.get_mut::<Velocity>().unwrap();
        vel.0.x = Float::from_num(0.02);
        vel.0.y = Float::from_num(0.05);

        let ent = world
            .spawn((
                Acceleration::default(),
                Velocity::default(),
                Position::default(),
                Drawable {
                    asset: "texture".to_string(),
                },
            ))
            .id();
        let mut ent_mut = world.get_entity_mut(ent).unwrap();
        let mut vel = ent_mut.get_mut::<Velocity>().unwrap();
        vel.0.x = Float::from_num(-0.05);
        vel.0.y = Float::from_num(0.1);

        Ok(SimulationStateImpl { world, actions: Default::default() })

        /*let registration = TypeRegistration::of();
        let registry = TypeRegistry::new();
        let deserializer = TypedReflectDeserializer::new(&registration, &registry);

        let de = serde_yaml::Deserializer::from_str(&file_content);
        let world: &mut World = deserializer.deserialize(de).unwrap().take().unwrap();*/
        // }
    }
}
