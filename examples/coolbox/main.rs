use cooltraption_playground::assets::file_asset_bundle::FileAssetBundle;

use log::info;
use std::env;
use std::fmt::{Display, Formatter};
use std::path::PathBuf;

#[allow(unused, dead_code)]
use cooltraption_playground::runtime::{Runtime, RuntimeImpl};

use bevy_ecs::prelude::*;

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
    /*match bundle.get_asset("strings").unwrap() {
        Asset::Strings(map) => {
            info!("{}", map.get("greet").unwrap());
        }
        _ => {
            warn!("Didn't find Strings asset");
        }
    }*/

    let mut world = World::new();
    world.insert_resource(bundle);

    /*let ent = world
        .spawn((
            Acceleration::default(),
            Velocity::default(),
            Position::default(),
        ))
        .id();
    let mut ent_mut = world.get_entity_mut(ent).unwrap();
    let mut vel = ent_mut.get_mut::<Velocity>().unwrap();
    vel.0.x = 3.0;
    vel.0.y = 1.0;*/

    let mut runtime = RuntimeImpl::new(world);
    /*for i in 0..3 {
        runtime.step_simulation(Duration::from_secs(i));
    }*/
}

fn play() {
    env::set_var("RUST_LOG", "test_game=debug,cooltraption_playground=debug");
    env_logger::init();

    set_working_dir().expect("Could not set working dir");
    info!(
        "Starting in {}",
        env::current_dir().unwrap().to_str().unwrap()
    );

    let bundle = FileAssetBundle::load(PathBuf::from("./assets"));

    let mut world = World::new();
    world.insert_resource(bundle);

    let mut runtime = RuntimeImpl::new(world);
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
