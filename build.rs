use std::env;

use copy_to_output::copy_to_output;

fn main() {
    // Re-runs script if any files in assets are changed
    println!("cargo:rerun-if-changed=crates/cooltraption_coolbox/assets/*");

    let crate_name = env::var("CARGO_PKG_NAME").unwrap();
    let crate_version = env::var("CARGO_PKG_VERSION").unwrap();
    println!("Building crate {} version {}", crate_name, crate_version);

    if crate_name == "cooltraption_coolbox" {
        copy_to_output(
            "crates/cooltraption_coolbox/assets/",
            &env::var("PROFILE").unwrap(),
        )
        .expect("Could not copy");
    }
}
