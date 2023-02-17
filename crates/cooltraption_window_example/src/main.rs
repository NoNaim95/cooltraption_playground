use cgmath::num_traits::Float;
use cgmath::{InnerSpace, Vector2, Vector3, Zero};
use cooltraption_window::asset_bundle::file_asset_loader::FileAssetLoader;
use cooltraption_window::render::keyboard_state::{KeyboardState, VirtualKeyCode};
use cooltraption_window::render::{
    CameraControls, Drawable, Id, Position, Scale, WgpuWindow, WgpuWindowConfig, WorldState,
};
use std::env;
use std::env::current_dir;
use std::ops::{Neg, Range};
use std::sync::mpsc;
use std::thread::sleep;
use std::time::{Duration, Instant};

#[tokio::main]
async fn main() {
    env::set_var("RUST_LOG", "info");
    env_logger::init();

    let (state_send, state_recv) = mpsc::sync_channel(1);
    let (keyboard_send, keyboard_recv) = mpsc::channel();
    let (controls_send, controls_recv) = mpsc::channel();

    let config = WgpuWindowConfig {
        state_recv,
        keyboard_send,
        controls_recv,
        asset_loader: Box::new(FileAssetLoader::new(
            current_dir()
                .unwrap()
                .join("cooltraption_window_example/assets"),
        )),
    };

    tokio::spawn(async move {
        let start = Instant::now();
        let mut controls = CameraControls::default();

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

            while let Ok(keyboard_state) = keyboard_recv.try_recv() {
                controls = handle_controls(&keyboard_state);
            }

            if controls_send.send(controls.clone()).is_err() {
                return;
            }

            sleep(Duration::from_millis(10));
        }
    });

    WgpuWindow::run(config).await;
}

fn handle_controls(keyboard_state: &KeyboardState) -> CameraControls {
    let mut controls = CameraControls::default();

    let zoom_speed = 1.01;
    let move_speed = 0.01;

    if keyboard_state.is_down(VirtualKeyCode::Q) {
        controls.zoom /= zoom_speed;
    }
    if keyboard_state.is_down(VirtualKeyCode::E) {
        controls.zoom *= zoom_speed;
    }

    if keyboard_state.is_down(VirtualKeyCode::W) {
        controls.move_vec.y += 1.0;
    }
    if keyboard_state.is_down(VirtualKeyCode::A) {
        controls.move_vec.x -= 1.0;
    }
    if keyboard_state.is_down(VirtualKeyCode::S) {
        controls.move_vec.y -= 1.0;
    }
    if keyboard_state.is_down(VirtualKeyCode::D) {
        controls.move_vec.x += 1.0;
    }

    if controls.move_vec.magnitude() > 0.0 {
        controls.move_vec = controls.move_vec.normalize_to(move_speed);
    }

    controls
}

fn wrap<T: Float>(val: T, range: Range<T>) -> T {
    let width = range.end - range.start;
    ((val - range.start) % width) + range.start
}
