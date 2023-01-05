use cooltraption_playground::asset_manager::file_asset_manager::FileAssetManager;
use cooltraption_playground::asset_manager::{Asset, AssetManager};
use log::{info, warn};
use std::env;
use std::path::PathBuf;

fn main() {
    env::set_var("RUST_LOG", "test_game=debug");
    env_logger::init();

    set_working_dir().expect("Could not set working dir");
    info!(
        "Starting in {}",
        env::current_dir().unwrap().to_str().unwrap()
    );

    let manager = FileAssetManager::load(PathBuf::from("./assets"));
    match manager.get_asset("strings.yml").unwrap() {
        Asset::Strings(map) => {
            info!("{}", map.get("greet").unwrap());
        }
        _ => {
            warn!("Didn't find Strings asset");
        }
    }
}

#[derive(Debug)]
enum Error {
    GetError,
    SetError,
}

fn set_working_dir() -> Result<(), Error> {
    let mut working_dir = env::current_exe().or(Err(Error::GetError))?;
    Some(working_dir.pop())
        .filter(|_| true)
        .ok_or(Error::GetError)?;
    Some(working_dir.pop())
        .filter(|_| true)
        .ok_or(Error::GetError)?;
    env::set_current_dir(working_dir).or(Err(Error::SetError))
}
