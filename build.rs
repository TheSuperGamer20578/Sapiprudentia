#![warn(clippy::pedantic)]

use std::env;
use std::process::Command;

fn main() {
    println!("cargo:rerun-if-changed=migrations");
    println!("cargo:rerun-if-changed=ts");
    println!("cargo:rerun-if-changed=sass");
    let out = env::var("OUT_DIR").unwrap();
    assert!(Command::new("tsc")
        .args(["--project", "tsconfig.json", "--outDir", &format!("{out}/js")])
        .status().expect("Failed to run tsc")
        .success());
    assert!(Command::new("sass")
        .args(["--no-source-map", &format!("sass:{out}/css")])
        .status().expect("Failed to run sass")
        .success());
}
