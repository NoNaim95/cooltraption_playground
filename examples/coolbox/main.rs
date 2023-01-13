use std::env;
use std::fmt::{Display, Formatter};
use std::path::PathBuf;
use std::time::Instant;

use log::info;
use winit::event::{Event, KeyboardInput, VirtualKeyCode, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::Window;

use cooltraption_playground::asset_bundle::file_asset_bundle::FileAssetBundle;
use cooltraption_playground::asset_bundle::strings_asset::StringsAsset;
use cooltraption_playground::asset_bundle::AssetBundle;
use cooltraption_playground::render::wgpu_state::WgpuState;
use cooltraption_playground::runtime::RuntimeOptions;
use cooltraption_playground::scene::file_scene_loader::MockFileSceneLoader;
use cooltraption_playground::scene::LoadScene;
mod entities;

use cooltraption_playground::runtime::{Runtime, RuntimeImpl};

#[tokio::main]
async fn main() {
    env::set_var("RUST_LOG", "coolbox=debug,cooltraption_playground=debug");
    env_logger::init();

    set_working_dir().expect("Could not set working dir");
    info!(
        "Starting in {}",
        env::current_dir().unwrap().to_str().unwrap()
    );

    let event_loop = EventLoop::new();
    let window = Window::new(&event_loop).expect("Could not create window");
    let mut wgpu_state = WgpuState::new(&window).await;

    let bundle = FileAssetBundle::load(PathBuf::from("./assets"), &wgpu_state)
        .expect("Could not load assets");
    let strings: &StringsAsset = bundle
        .get_asset("strings")
        .expect("Could not find strings asset");
    //info!("{}", strings.map.get("greet").unwrap());

    let loader = MockFileSceneLoader::from(PathBuf::from("./scenes/scene1"));
    let options = RuntimeOptions {
        initial_scene: Box::new(loader.load(&wgpu_state).unwrap()),
    };

    let mut runtime = RuntimeImpl::new(options);
    let start_instant = Instant::now();

    event_loop.run(move |event, _, control_flow| {
        match event {
            Event::WindowEvent {
                ref event,
                window_id,
            } if window_id == window.id() => {
                if !wgpu_state.input(event) {
                    match event {
                        WindowEvent::CloseRequested
                        | WindowEvent::KeyboardInput {
                            input:
                                KeyboardInput {
                                    state: winit::event::ElementState::Pressed,
                                    virtual_keycode: Some(VirtualKeyCode::Escape),
                                    ..
                                },
                            ..
                        } => *control_flow = ControlFlow::Exit,
                        WindowEvent::Resized(physical_size) => {
                            wgpu_state.resize(*physical_size);
                        }
                        WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                            wgpu_state.resize(**new_inner_size);
                        }
                        _ => {}
                    }
                }
            }
            Event::RedrawRequested(window_id) if window_id == window.id() => {
                runtime.step_simulation(Instant::now() - start_instant);

                match wgpu_state.render() {
                    Ok(_) => {}
                    // Reconfigure the surface if lost
                    Err(wgpu::SurfaceError::Lost) => wgpu_state.resize(wgpu_state.size()),
                    // The system is out of memory, we should probably quit
                    Err(wgpu::SurfaceError::OutOfMemory) => *control_flow = ControlFlow::Exit,
                    // All other errors (Outdated, Timeout) should be resolved by the next frame
                    Err(e) => eprintln!("{:?}", e),
                }
            }
            Event::MainEventsCleared => {
                // RedrawRequested will only trigger once, unless we manually
                // request it.
                window.request_redraw();
            }
            _ => {}
        }
    });
}

#[derive(Debug)]
enum Error {
    CurrentExe,
    ChangeDir,
    CurrentDir,
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::CurrentExe => write!(f, "Could not find current exe directory"),
            Error::ChangeDir => write!(f, "Could not change directory two levels up"),
            Error::CurrentDir => write!(f, "Could not change current directory"),
        }
    }
}

fn set_working_dir() -> Result<(), Error> {
    let mut working_dir = env::current_exe().or(Err(Error::CurrentExe))?;
    Some(working_dir.pop())
        .filter(|_| true)
        .ok_or(Error::ChangeDir)?;
    Some(working_dir.pop())
        .filter(|_| true)
        .ok_or(Error::ChangeDir)?;
    env::set_current_dir(working_dir).or(Err(Error::CurrentDir))
}
