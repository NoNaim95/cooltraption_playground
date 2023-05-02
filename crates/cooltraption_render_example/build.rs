use std::fs;
use std::path::Path;
use std::{env, io};

fn main() {
    println!("cargo:rerun-if-changed=assets");

    let target_dir = Path::new(&env::var("OUT_DIR").unwrap()).join("../../../assets");

    let _ = fs::remove_dir_all(&target_dir);

    // Copy the assets directory to the target directory.
    fs::create_dir_all(&target_dir).unwrap();
    copy_dir_all("assets", target_dir).unwrap();
}

fn copy_dir_all(src: impl AsRef<Path>, dst: impl AsRef<Path>) -> io::Result<()> {
    fs::create_dir_all(&dst)?;
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        if ty.is_dir() {
            copy_dir_all(entry.path(), dst.as_ref().join(entry.file_name()))?;
        } else {
            fs::copy(entry.path(), dst.as_ref().join(entry.file_name()))?;
        }
    }
    Ok(())
}
