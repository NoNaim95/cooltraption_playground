mod controller;

use crate::controller::Controller;
use cgmath::num_traits::Float;
use cgmath::*;
use cooltraption_window::asset_bundle::{FileAssetLoader, LoadAssetBundle, TextureAtlasBuilder};
use cooltraption_window::gui::GuiInitializer;
use cooltraption_window::instance_renderer::world_state::{Drawable, Id, Position, Scale};
use cooltraption_window::instance_renderer::{InstanceRendererInitializer, WorldState};
use cooltraption_window::render::render_event_handler::RenderEventHandler;
use cooltraption_window::window::{EventLoopHandler, WindowEventHandler};
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

    let instance_renderer = {
        let mut texture_atlas_builder = TextureAtlasBuilder::default();
        let assets = FileAssetLoader::new(
            current_dir()
                .unwrap()
                .join("cooltraption_window_example/assets"),
        )
        .load(&mut texture_atlas_builder)
        .expect("load assets");

        Box::new(InstanceRendererInitializer {
            texture_atlas_builder,
            assets,
            state_recv,
        })
    };
    let gui = Box::new(GuiInitializer {});

    let mut render_event_handler = RenderEventHandler::default();
    render_event_handler.add_initializer(instance_renderer);
    render_event_handler.add_initializer(gui);

    let camera_controller = Controller::default();

    let mut event_loop_handler = EventLoopHandler::new().await;

    event_loop_handler.add_handler(Rc::new(RefCell::new(WindowEventHandler {})));
    event_loop_handler.add_handler(Rc::new(RefCell::new(render_event_handler)));
    event_loop_handler.add_handler(Rc::new(RefCell::new(camera_controller)));

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
                    asset_name: "house".to_string(),
                },
                Drawable {
                    id: Id(3),
                    position: Position(pos1.neg()),
                    scale: Scale(Vector2::new(0.2, 0.2)),
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
