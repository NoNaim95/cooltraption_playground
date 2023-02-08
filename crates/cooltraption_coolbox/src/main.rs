use std::env;
use std::fmt::{Display, Formatter};
use std::path::PathBuf;
use std::sync::mpsc;

use cooltraption_window::asset_bundle::file_asset_loader::FileAssetLoader;
use cooltraption_window::render::{RenderMachine, RenderMachineOptions, RenderWorld};
use log::info;

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

    let (state_send, state_recv) = mpsc::sync_channel::<RenderWorld>(1);

    tokio::spawn(async {
        let simulation_loader = MockFileSimulationLoader::from(PathBuf::from("./scenes/scene1"));
        let options = SimulationOptions {
            simulation_loader: Box::new(simulation_loader),
            state_send,
        };

        SimulationImpl::new(options).run();
    });

    let asset_loader = FileAssetLoader::new("./assets".into());
    let options = RenderMachineOptions {
        asset_loader: Box::new(asset_loader),
        state_recv,
    };
    RenderMachine::run(options).await;
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
