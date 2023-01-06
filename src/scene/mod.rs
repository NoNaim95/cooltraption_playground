pub mod file_loader;

use bevy_ecs::prelude::World;

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

pub trait Load<T: Scene> {
    fn load(&self) -> T;
}

pub trait Save {
    fn save(&self, scene: &dyn Scene);
}
