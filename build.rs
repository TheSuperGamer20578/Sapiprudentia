#![warn(clippy::pedantic)]

use std::env;
use build_pretty::{build_pretty, CommandBuilder};

fn main() {
    println!("cargo:rerun-if-changed=migrations");
    println!("cargo:rerun-if-changed=frontend");
    println!("cargo:rerun-if-changed=package.json");
    println!("cargo:rerun-if-changed=webpack.config.js");
    println!("cargo:rerun-if-changed=tsconfig.json");
    let mode = match &*env::var("PROFILE").unwrap() {
        "release" => "production",
        "debug" => "development",
        _ => unreachable!()
    };
    let npm = env::var("NPM").unwrap_or("npm".into());
    build_pretty!()
        .enque_command(
            "Install npm dependencies",
            CommandBuilder::new_with_args(
                &*npm,
                &["install", "--color"],
            ).into(),
        )
        .enque_command(
            "Run npm build",
            CommandBuilder::new_with_args(
                &*npm,
                &["run", "build", "--mode", mode, "--color"],
            ).into(),
        );
}
