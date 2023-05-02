mod controller;
mod controls;
mod debug_window;

use crate::controller::Controller;
use cgmath::num_traits::Float;
use cgmath::*;
use cooltraption_render::gui;
use cooltraption_render::renderer::WgpuInitializer;
use cooltraption_render::world_renderer::asset_bundle::{FileAssetLoader, LoadAssetBundle};
use cooltraption_render::world_renderer::texture_atlas::TextureAtlasBuilder;
use cooltraption_render::world_renderer::world_state::{Drawable, Id, Position, Rotation, Scale};
use cooltraption_render::world_renderer::{WorldRendererInitializer, WorldState};
use cooltraption_window::window::{WindowEventHandler, WinitEventLoopHandler};
use log::info;
use std::env;
use std::ops::{Neg, Range};
use std::sync::mpsc;
use std::sync::mpsc::SyncSender;
use std::thread::sleep;
use std::time::{Duration, Instant};

#[tokio::main]
async fn main() {
    env::set_var("RUST_LOG", "info");
    env_logger::init();

    let (state_send, state_recv) = mpsc::sync_channel(0);

    tokio::spawn(async move { run_mock_simulation(state_send) });

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
            state_recv,
        })
    };

    let mut wgpu_initializer = WgpuInitializer::default();
    wgpu_initializer.add_initializer(world_renderer);
    wgpu_initializer.add_initializer(Box::new(gui_renderer));

    let mut event_loop_handler = WinitEventLoopHandler::default();

    event_loop_handler.register_event_handler(Box::new(WindowEventHandler {}));
    event_loop_handler.register_event_handler(Box::new(gui_event_handler));
    event_loop_handler.register_event_handler(Box::new(wgpu_initializer));
    event_loop_handler.register_event_handler(Box::new(controller_event_handler));

    event_loop_handler.run_event_loop();
}

fn run_mock_simulation(state_send: SyncSender<WorldState>) {
    let start = Instant::now();

    loop {
        let (pos1, pos2, pos3) = {
            let time = start.elapsed().as_secs_f32() / 10.0;

            (
                Vector2::new(time.sin(), time.cos()),
                Vector2::new(wrap(time * 4.0, -4.0..4.0), wrap(time * 4.0, -4.0..4.0)),
                Vector2::new(wrap(time * 1.5, -4.0..4.0), 0.0),
            )
        };

        let world_state = WorldState {
            drawables: vec![
                Drawable {
                    id: Id(0),
                    position: Position(pos3.neg()),
                    scale: Scale(Vector2::new(0.8, 0.8)),
                    rot: Rotation(0.0),
                    asset_name: "cloud".to_string(),
                },
                Drawable {
                    id: Id(1),
                    position: Position(pos2),
                    asset_name: "plane".to_string(),
                    ..Default::default()
                },
                Drawable {
                    id: Id(2),
                    position: Position(pos1),
                    scale: Scale(Vector2::new(0.4, 0.4)),
                    rot: Rotation(0.0),
                    asset_name: "house".to_string(),
                },
                Drawable {
                    id: Id(3),
                    position: Position(pos1.neg()),
                    scale: Scale(Vector2::new(0.2, 0.2)),
                    rot: Rotation(0.0),
                    asset_name: "dude".to_string(),
                },
                Drawable {
                    id: Id(4),
                    position: Position(pos3),
                    asset_name: "cloud".to_string(),
                    ..Default::default()
                },
            ],
        };

        if let Err(e) = state_send.send(world_state) {
            info!("Exiting simulation loop: {}", e);
            return;
        }

        sleep(Duration::from_millis(10));
    }
}

fn wrap<T: Float>(val: T, range: Range<T>) -> T {
    let width = range.end - range.start;
    ((val - range.start) % width) + range.start
}
