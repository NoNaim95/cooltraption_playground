use cooltraption_playground::assets::file_asset_bundle::FileAssetBundle;
use cooltraption_playground::assets::{Asset, AssetBundle};
use log::{info, warn};
use std::env;
use std::path::PathBuf;

#[allow(unused, dead_code)]
use cooltraption_playground::runtime::{Runtime, RuntimeImpl};

use bevy_ecs::prelude::*;
use cooltraption_playground::stages::physics_stage::{Acceleration, Position, Velocity};
use std::time::Duration;

mod entities;

fn main() {
    env::set_var("RUST_LOG", "test_game=debug,cooltraption_playground=debug");
    env_logger::init();

    set_working_dir().expect("Could not set working dir");
    info!(
        "Starting in {}",
        env::current_dir().unwrap().to_str().unwrap()
    );

    let bundle = FileAssetBundle::load(PathBuf::from("./assets"));
    match bundle.get_asset("strings").unwrap() {
        Asset::Strings(map) => {
            info!("{}", map.get("greet").unwrap());
        }
        _ => {
            warn!("Didn't find Strings asset");
        }
    }

    let mut world = World::new();
    world.insert_resource(bundle);

    let ent = world
        .spawn((
            Acceleration::default(),
            Velocity::default(),
            Position::default(),
        ))
        .id();
    let mut ent_mut = world.get_entity_mut(ent).unwrap();
    let mut vel = ent_mut.get_mut::<Velocity>().unwrap();
    vel.0.x = 3.0;
    vel.0.y = 1.0;

    let mut runtime = RuntimeImpl::new(world);
    for i in 0..3 {
        runtime.step_simulation(Duration::from_secs(i));
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
