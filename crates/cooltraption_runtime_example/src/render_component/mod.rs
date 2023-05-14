mod controller;
mod controls;
mod debug_widget;

use controller::Controller;
use cooltraption_render::gui;
use cooltraption_render::renderer::WgpuInitializer;
use cooltraption_render::world_renderer::asset_bundle::{FileAssetLoader, LoadAssetBundle};
use cooltraption_render::world_renderer::texture_atlas::TextureAtlasBuilder;
use cooltraption_render::world_renderer::{WorldRendererInitializer, WorldState};
use cooltraption_window::window::{WindowEventHandler, WinitEventLoopHandler};
use std::env;

//use cooltraption_input::input::InputEventHandler;

#[tokio::main]
pub async fn run_renderer<I>(state_iterator: I/*, input_event_handler: InputEventHandler<'static>*/)
where
    I: Iterator<Item = WorldState> + 'static,
{
    env::set_var("RUST_LOG", "info");
    //env_logger::init();

    let (gui_renderer, gui_event_handler, dispatcher) = gui::new();
    let (controller, controller_event_handler) = Controller::new(dispatcher);

    let world_renderer = {
        let mut texture_atlas_builder = TextureAtlasBuilder::default();

        let assets_dir = env::current_exe().unwrap().parent().unwrap().join("assets");

        let assets = FileAssetLoader::new(assets_dir)
            .load(&mut texture_atlas_builder)
            .expect("load assets");

        Box::new(WorldRendererInitializer {
            controller,
            texture_atlas_builder,
            assets,
            state_recv: state_iterator,
        })
    };

    let mut wgpu_initializer = WgpuInitializer::default();
    wgpu_initializer.add_initializer(world_renderer);
    wgpu_initializer.add_initializer(Box::new(gui_renderer));

    let mut event_loop_handler = WinitEventLoopHandler::default();

    //event_loop_handler.register_event_handler(Box::new(input_event_handler));
    event_loop_handler.register_event_handler(Box::new(WindowEventHandler {}));
    event_loop_handler.register_event_handler(Box::new(gui_event_handler));
    event_loop_handler.register_event_handler(Box::new(wgpu_initializer));
    event_loop_handler.register_event_handler(Box::new(controller_event_handler));

    event_loop_handler.run_event_loop();
}
