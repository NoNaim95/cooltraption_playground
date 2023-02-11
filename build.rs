use fs_extra::dir::CopyOptions;
use std::path::PathBuf;
use std::{env, fs};

fn main() {
    // Re-runs script if any files in assets are changed
    cargo_emit::rerun_if_changed!(
        "crates/cooltraption_coolbox/assets/*",
        "crates/cooltraption_window_example/assets/*"
    );

    let crate_name = env::var("CARGO_PKG_NAME").unwrap();
    let crate_version = env::var("CARGO_PKG_VERSION").unwrap();
    let build_type = &env::var("PROFILE").unwrap();
    println!("Building crate {} version {}", crate_name, crate_version);

    clear_output(build_type).expect("remove old assets");

    copy_to_output("crates/cooltraption_coolbox/assets/", build_type).expect("copy assets");
    copy_to_output("crates/cooltraption_window_example/assets/", build_type).expect("copy assets");
}

pub fn clear_output(build_type: &str) -> Result<(), fs_extra::error::Error> {
    let out_path = format!("target/{}/assets/", build_type);
    fs::remove_dir(PathBuf::from(out_path))?;

    Ok(())
}

pub fn copy_to_output(path: &str, build_type: &str) -> Result<(), fs_extra::error::Error> {
    let mut options = CopyOptions::new();
    let mut from_path = Vec::new();
    let out_path = format!("target/{}", build_type);

    // Overwrite existing files with same name
    options.overwrite = true;

    from_path.push(path);
    fs_extra::copy_items(&from_path, &out_path, &options)?;

    Ok(())
}
