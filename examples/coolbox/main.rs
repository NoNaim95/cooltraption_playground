use std::env;
use std::fmt::{Display, Formatter};
use std::path::PathBuf;
use std::time::Duration;

use log::info;
use winit::event_loop::EventLoop;
use winit::window::Window;

use cooltraption_playground::asset_bundle::file_asset_bundle::FileAssetBundle;
use cooltraption_playground::asset_bundle::strings_asset::StringsAsset;
use cooltraption_playground::asset_bundle::AssetBundle;
use cooltraption_playground::render::wgpu_state::WgpuState;
use cooltraption_playground::runtime::RuntimeOptions;
use cooltraption_playground::scene::file_loader::FileLoader;
use cooltraption_playground::scene::Load;
mod entities;

use cooltraption_playground::runtime::{Runtime, RuntimeImpl};

fn main() {
    env::set_var("RUST_LOG", "coolbox=debug,cooltraption_playground=debug");
    env_logger::init();

    set_working_dir().expect("Could not set working dir");
    info!(
        "Starting in {}",
        env::current_dir().unwrap().to_str().unwrap()
    );

    let event_loop = EventLoop::default();
    let window = Window::new(&event_loop).expect("Could not create window");
    todo!("tokio?");
    let wgpu_state = WgpuState::new(&window).await;

    let bundle = FileAssetBundle::load(PathBuf::from("./assets"), &wgpu_state)
        .expect("Could not load assets");
    let strings: &StringsAsset = bundle
        .get_asset("strings")
        .expect("Could not find strings asset");
    info!("{}", strings.map.get("greet").unwrap());

    let loader = FileLoader::from(PathBuf::from("./scenes/scene1"));
    let options = RuntimeOptions {
        initial_scene: Box::new(loader.load(&wgpu_state).unwrap()),
    };
    let mut runtime = RuntimeImpl::new(options);
    for i in 0..3 {
        runtime.step_simulation(Duration::from_secs(i));
    }
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
