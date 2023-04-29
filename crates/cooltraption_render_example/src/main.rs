mod controller;

use crate::controller::Controller;
use cgmath::num_traits::Float;
use cgmath::*;
use cooltraption_assets::asset_bundle::*;
use cooltraption_assets::texture_atlas::TextureAtlasBuilder;
use cooltraption_render::gui::GuiInitializer;
use cooltraption_render::renderer::WgpuInitializer;
use cooltraption_render::window::{WindowEventHandler, WinitEventLoopHandler};
use cooltraption_render::world_renderer::world_state::{Drawable, Id, Position, Scale};
use cooltraption_render::world_renderer::{WorldRendererInitializer, WorldState};
use log::info;
use std::cell::RefCell;
use std::env;
use std::env::current_dir;
use std::ops::{Neg, Range};
use std::rc::Rc;
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
            state_recv,
        })
    };
    let (gui, gui_event_handler) = GuiInitializer::new();

    let mut wgpu_initializer = WgpuInitializer::default();
    wgpu_initializer.add_initializer(world_renderer);
    wgpu_initializer.add_initializer(Box::new(gui));

    let mut event_loop_handler = WinitEventLoopHandler::new().await;

    event_loop_handler.register_event_handler(Rc::new(RefCell::new(WindowEventHandler {})));
    event_loop_handler.register_event_handler(Rc::new(RefCell::new(gui_event_handler)));
    event_loop_handler.register_event_handler(Rc::new(RefCell::new(wgpu_initializer)));
    event_loop_handler.register_event_handler(Rc::new(RefCell::new(controller_event_handler)));

    event_loop_handler.run_event_loop();
}

fn run_mock_simulation(state_send: SyncSender<WorldState>) {



    let start = Instant::now();
    loop {
        let (pos1, _pos2, _pos3) = {
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
                    id: Id(3),
                    position: Position(pos1.neg()),
                    scale: Scale(Vector2::new(0.2, 0.2)),
                    asset_name: "dude".to_string(),
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
