use std::env;
use std::fmt::{Display, Formatter};
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use std::time::Instant;

use log::info;
use winit::event::{Event, KeyboardInput, VirtualKeyCode, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};

use cooltraption_playground::asset_bundle::file_asset_bundle::FileAssetBundle;
use cooltraption_playground::asset_bundle::strings_asset::StringsAsset;
use cooltraption_playground::asset_bundle::AssetBundle;
use cooltraption_playground::runtime::RuntimeImpl;
use cooltraption_playground::runtime::RuntimeOptions;
use cooltraption_playground::scene::file_scene_loader::MockFileSceneLoader;
use cooltraption_playground::scene::LoadScene;

mod entities;

#[tokio::main]
async fn main() {
    env::set_var("RUST_LOG", "coolbox=debug,cooltraption_playground=debug");
    env_logger::init();

    set_working_dir().expect("Could not set working dir");
    info!(
        "Starting in {}",
        env::current_dir().unwrap().to_str().unwrap()
    );

    let loader = MockFileSceneLoader::from(PathBuf::from("./scenes/scene1"));
    let options = RuntimeOptions {
        scene_loader: Box::new(loader),
    };

    let runtime = RuntimeImpl::start(&options).await;
    /*
    let bundle = FileAssetBundle::load(PathBuf::from("./assets"), &mut wgpu_state)
        .expect("Could not load assets");
    let strings: &StringsAsset = bundle
        .get_asset("strings")
        .expect("Could not find strings asset");
    info!("{}", strings.map.get("greet").unwrap());*/
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
