use cgmath::num_traits::Float;
use cgmath::Vector2;
use cooltraption_window::asset_bundle::file_asset_loader::FileAssetLoader;
use cooltraption_window::render::{Drawable, Position, WgpuWindow, WgpuWindowConfig, WorldState};
use log::info;
use std::env;
use std::ops::{Neg, Range};
use std::sync::mpsc;
use std::thread::sleep;
use std::time::{Duration, Instant};

#[tokio::main]
async fn main() {
    env::set_var(
        "RUST_LOG",
        "coolbox=debug,cooltraption_window_example=debug",
    );
    env_logger::init();

    let (state_send, state_recv) = mpsc::sync_channel(1);

    let config = WgpuWindowConfig {
        state_recv,
        asset_loader: Box::new(FileAssetLoader::new(
            env::current_exe()
                .unwrap()
                .parent()
                .unwrap()
                .join("./assets"),
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
                state: vec![
                    Drawable {
                        position: Position(pos2),
                        asset_name: "does this asset exist?".to_string(),
                    },
                    Drawable {
                        position: Position(pos1),
                        asset_name: "house".to_string(),
                    },
                    Drawable {
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
