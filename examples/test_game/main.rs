use cooltraption_playground::asset_manager::{Asset, AssetManager, FileAssetManager};
use std::path::PathBuf;

fn main() {
    let manager = FileAssetManager::load(PathBuf::from("target/debug/assets"));
    match manager.get_asset("strings.yml").unwrap() {
        Asset::Strings(map) => {
            println!("{}", map.get("greet").unwrap());
        }
        _ => {
            println!("Didn't find Strings asset");
        }
    }
}
