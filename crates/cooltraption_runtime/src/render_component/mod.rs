mod controller;
mod controls;

use controller::Controller;
use cooltraption_render::gui::GuiRendererInitializer;
use cooltraption_render::renderer::WgpuInitializer;
use cooltraption_render::window::{WindowEventHandler, WinitEventLoopHandler};
use cooltraption_render::world_renderer::asset_bundle::{FileAssetLoader, LoadAssetBundle};
use cooltraption_render::world_renderer::texture_atlas::TextureAtlasBuilder;
use cooltraption_render::world_renderer::{WorldRendererInitializer, WorldState};
use std::env;
use std::env::current_dir;

#[tokio::main]
pub async fn run_renderer(state_iterator: impl Iterator<Item = WorldState> + 'static) -> ! {
    env::set_var("RUST_LOG", "info");
    env_logger::init();

    let (controller, controller_event_handler) = Controller::new();

    let world_renderer = {
        let mut texture_atlas_builder = TextureAtlasBuilder::default();
        let assets = FileAssetLoader::new(
            current_dir()
                .unwrap()
                .join("cooltraption_render_example/assets"),
        )
        .load(&mut texture_atlas_builder)
        .expect("load assets");

        Box::new(WorldRendererInitializer {
            controller,
            texture_atlas_builder,
            assets,
            state_recv: state_iterator,
        })
    };
    let (gui, gui_event_handler) = GuiInitializer::new();

    let mut wgpu_initializer = WgpuInitializer::default();
    wgpu_initializer.add_initializer(world_renderer);
    wgpu_initializer.add_initializer(Box::new(gui));

    let mut event_loop_handler = WinitEventLoopHandler::default();

    event_loop_handler.register_event_handler(Box::new(WindowEventHandler {}));
    event_loop_handler.register_event_handler(Box::new(gui_event_handler));
    event_loop_handler.register_event_handler(Box::new(wgpu_initializer));
    event_loop_handler.register_event_handler(Box::new(controller_event_handler));

    event_loop_handler.run_event_loop();
}
