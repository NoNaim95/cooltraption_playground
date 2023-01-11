use bevy_ecs::prelude::World;

pub mod file_loader;

pub trait Scene {
    fn world(&self) -> &World;
    fn world_mut(&mut self) -> &mut World;
}

pub struct SceneImpl {
    world: World,
}

impl Scene for SceneImpl {
    fn world(&self) -> &World {
        &self.world
    }

    fn world_mut(&mut self) -> &mut World {
        &mut self.world
    }
}

pub trait Load<T: Scene, E> {
    fn load(&self) -> Result<T, E>;
}

pub trait Save<T: Scene> {
    fn save(&self, scene: T);
}
