use cgmath::num_traits::Float;
use cgmath::*;
use cooltraption_window::asset_bundle::{FileAssetLoader, LoadAssetBundle, TextureAtlasBuilder};
use cooltraption_window::camera::controller::CameraController;
use cooltraption_window::gui::Gui;
use cooltraption_window::instance_renderer::world_state::{Drawable, Id, Position, Scale};
use cooltraption_window::instance_renderer::{InstanceRenderer, WorldState};
use cooltraption_window::render::render_event_handler::RenderEventHandler;
use cooltraption_window::window_event_handler::WindowEventHandler;
use cooltraption_window::EventLoopHandler;
use std::cell::RefCell;
use std::env;
use std::env::current_dir;
use std::ops::{Neg, Range};
use std::rc::Rc;
use std::sync::mpsc;
use std::thread::sleep;
use std::time::{Duration, Instant};

#[tokio::main]
async fn main() {
    env::set_var("RUST_LOG", "info");
    env_logger::init();

    let (state_send, state_recv) = mpsc::sync_channel(0);

    tokio::spawn(async move {
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

            if state_send.send(world_state).is_err() {
                return;
            }

            sleep(Duration::from_millis(10));
        }
    });

    let asset_loader = FileAssetLoader::new(
        current_dir()
            .unwrap()
            .join("cooltraption_window_example/assets"),
    );

    let instance_renderer = {
        let mut texture_atlas_builder = TextureAtlasBuilder::default();
        let assets = asset_loader
            .load(&mut texture_atlas_builder)
            .expect("load assets");

        Rc::new(RefCell::new(InstanceRenderer::new(
            assets,
            texture_atlas_builder,
            state_recv,
        )))
    };
    let gui = Rc::new(RefCell::new(Gui::default()));

    let render_event_handler =
        RenderEventHandler::new(vec![instance_renderer.clone(), gui.clone()]);
    let camera_controller = CameraController::default();

    let mut event_loop_handler = EventLoopHandler::new().await;

    event_loop_handler.add_handler(Rc::new(RefCell::new(WindowEventHandler {})));
    event_loop_handler.add_handler(Rc::new(RefCell::new(render_event_handler)));
    event_loop_handler.add_handler(Rc::new(RefCell::new(camera_controller)));
    event_loop_handler.add_handler(instance_renderer);
    event_loop_handler.add_handler(gui);

    event_loop_handler.run_event_loop();
}

fn wrap<T: Float>(val: T, range: Range<T>) -> T {
    let width = range.end - range.start;
    ((val - range.start) % width) + range.start
}
