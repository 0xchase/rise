extern crate bindgen;

use std::env;
use std::path::PathBuf;
use std::process::Command;

fn main() {
    // Build Rizin
    let build_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    let build_path = build_path.join("build/");
    let build_path = build_path.to_str().unwrap();

    println!("cargo:rustc-link-lib=rz");
    println!("cargo:rustc-link-search=native={}", build_path);
    build("../rizin", build_path);

    // Generate Rizin bindings

    // println!("cargo:rustc-link-search=/opt/homebrew/lib/");
    println!("cargo:rerun-if-changed=wrapper.h");

    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .clang_arg("-I/opt/homebrew/include/librz/**")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}

pub fn build(project_dir: &str, build_dir: &str) {
    run_meson(project_dir, build_dir);
}

fn run_meson(lib: &str, dir: &str) {
    if !is_configured(dir) {
        run_command(lib, "meson", &[".", dir]);
    }
    run_command(dir, "ninja", &[]);
}

fn run_command(dir: &str, name: &str, args: &[&str]) {
    let mut cmd = Command::new(name);
    cmd.current_dir(dir);
    if args.len() > 0 {
        cmd.args(args);
    }
    let status = cmd.status().expect("cannot run command");
    assert!(status.success());
}

fn is_configured(dir: &str) -> bool {
    let mut path = PathBuf::from(dir);
    path.push("build.ninja");
    return path.as_path().exists();
}
