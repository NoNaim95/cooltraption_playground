mod controller;
mod controls;
mod debug_widget;

use crate::controller::Controller;
use cgmath::num_traits::Float;
use cgmath::Vector2;
use cooltraption_render::gui;
use cooltraption_render::renderer::WgpuInitializer;
use cooltraption_render::world_renderer::asset_bundle::{FileAssetLoader, LoadAssetBundle};
use cooltraption_render::world_renderer::texture_atlas::TextureAtlasBuilder;
use cooltraption_render::world_renderer::world_state::{
    Drawable, Id, Position, Rotation, Scale, Transform,
};
use cooltraption_render::world_renderer::WorldRendererInitializer;
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

    let fixed_delta_time = Duration::from_millis(20);

    let (state_send, state_recv) = mpsc::sync_channel(0);
    let state_iterator = std::iter::from_fn(move || state_recv.try_recv().ok());

    std::thread::spawn(move || run_mock_simulation(state_send, fixed_delta_time));

    let (gui_renderer, gui_event_handler, dispatcher) = gui::new();
    let (controller, controller_event_handler) = Controller::new(dispatcher);

    let world_renderer = {
        let mut texture_atlas_builder = TextureAtlasBuilder::default();

        let assets_dir = env::current_exe()
            .unwrap()
            .parent()
            .unwrap()
            .join("assets/dark");

        let assets = FileAssetLoader::new(assets_dir)
            .load(&mut texture_atlas_builder)
            .expect("load assets");

        Box::new(WorldRendererInitializer {
            controller,
            texture_atlas_builder,
            assets,
            fixed_delta_time,
            state_recv: state_iterator,
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

fn run_mock_simulation(state_send: SyncSender<Vec<Drawable>>, fixed_delta_time: Duration) {
    let start = Instant::now();

    loop {
        let time = start.elapsed().as_secs_f32() / 10.0;
        let (circling, flying, floating) = {
            (
                Vector2::new(time.sin(), time.cos()),
                Vector2::new(wrap(time * 4.0, -4.0..4.0), wrap(time * 4.0, -4.0..4.0)),
                Vector2::new(wrap(time * 1.5, -4.0..4.0), 0.0),
            )
        };

        let drawables = vec![
            Drawable {
                id: Id(0),
                transform: Transform {
                    position: Position(floating.neg()),
                    scale: Scale(Vector2::new(0.8, 0.8)),
                    rot: Default::default(),
                },
                asset_name: "cloud".to_string(),
            },
            Drawable {
                id: Id(1),
                transform: Transform {
                    position: Position(flying),
                    ..Default::default()
                },
                asset_name: "plane".to_string(),
            },
            Drawable {
                id: Id(2),
                transform: Transform {
                    position: Position(circling),
                    scale: Scale(Vector2::new(0.4, 0.4)),
                    rot: Default::default(),
                },
                asset_name: "house".to_string(),
            },
            Drawable {
                id: Id(3),
                transform: Transform {
                    position: Position(circling.neg()),
                    scale: Scale(Vector2::new(0.2, 0.2)),
                    rot: Rotation(time * 10.0),
                },
                asset_name: "dude".to_string(),
            },
            Drawable {
                id: Id(4),
                transform: Transform {
                    position: Position(floating),
                    ..Default::default()
                },
                asset_name: "cloud".to_string(),
            },
        ];

        if let Err(e) = state_send.send(drawables) {
            info!("Exiting simulation loop: {}", e);
            return;
        }

        sleep(fixed_delta_time);
    }
}

fn wrap<T: Float>(val: T, range: Range<T>) -> T {
    let width = range.end - range.start;
    ((val - range.start) % width) + range.start
}
