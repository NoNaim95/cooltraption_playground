use crate::asset_bundle::file_asset_bundle::FileAssetBundle;
use crate::render::wgpu_state::WgpuState;
use bevy_ecs::prelude::World;

pub mod file_scene_loader;

pub trait Scene {
    fn world(&self) -> &World;
    fn world_mut(&mut self) -> &mut World;
}

pub struct SceneImpl {
    world: World,
    assets: FileAssetBundle,
}

impl Scene for SceneImpl {
    fn world(&self) -> &World {
        &self.world
    }

    fn world_mut(&mut self) -> &mut World {
        &mut self.world
    }
}

pub trait LoadScene<T: Scene, E> {
    fn load(&self, state: &WgpuState) -> Result<T, E>;
}

pub trait SaveScene<T: Scene> {
    fn save(&self, scene: T);
}
