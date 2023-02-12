use cgmath::num_traits::Float;
use cgmath::Vector2;
use cooltraption_window::asset_bundle::file_asset_loader::FileAssetLoader;
use cooltraption_window::render::world_state::{Drawable, Position, WorldState};
use cooltraption_window::render::{WgpuWindow, WgpuWindowConfig};
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

    let config = WgpuWindowConfig {
        state_recv,
        asset_loader: Box::new(FileAssetLoader::new(
            current_dir()
                .unwrap()
                .join("cooltraption_window_example/assets"),
        )),
    };

    tokio::spawn(async move {
        let start = Instant::now();

        loop {
            let (pos1, pos2) = {
                let time = start.elapsed().as_secs_f32() / 10.0;

                (
                    Vector2::new(time.sin(), time.cos()),
                    Vector2::new(wrap(time * 4.0, -4.0..4.0), wrap(time * 4.0, -4.0..4.0)),
                )
            };

            let world_state = WorldState {
                drawables: vec![
                    Drawable {
                        id: 0,
                        position: Position(pos2),
                        asset_name: "plane".to_string(),
                    },
                    Drawable {
                        id: 1,
                        position: Position(pos1),
                        asset_name: "house".to_string(),
                    },
                    Drawable {
                        id: 2,
                        position: Position(pos1.neg()),
                        asset_name: "dude".to_string(),
                    },
                ],
            };

            if state_send.send(world_state).is_err() {
                return;
            }
            sleep(Duration::from_millis(10));
        }
    });

    WgpuWindow::run(config).await;
}

fn wrap<T: Float>(val: T, range: Range<T>) -> T {
    let width = range.end - range.start;
    ((val - range.start) % width) + range.start
}
