use cc::Build;
use std::{env, path::PathBuf};

fn main() {
    let project_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let native = project_dir.join("native");

    Build::new()
        .include(&native)
        .file(native.join("stateful.cpp"))
        .cpp(true)
        .flag_if_supported("-std=c++17")
        .compile("stateful");
}
