extern crate bindgen;

use std::env;
use std::path::PathBuf;
use std::process::Command;

fn main() {
    // Build Rizin
    let build_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    let build_path = build_path.join("build/");
    let build_path = build_path.to_str().unwrap();

    // build("rizin", build_path);

    //println!("cargo:rustc-link-lib=rz_analysis");
    /*println!("cargo:rustc-link-lib=rz_asm");
    println!("cargo:rustc-link-lib=rz_bin");
    println!("cargo:rustc-link-lib=rz_bp");
    println!("cargo:rustc-link-lib=rz_config");
    println!("cargo:rustc-link-lib=rz_cons");*/
    // println!("cargo:rustc-link-lib=rz_core");
    /*println!("cargo:rustc-link-lib=rz_crypto");
    println!("cargo:rustc-link-lib=rz_debug");
    println!("cargo:rustc-link-lib=rz_demangler");
    println!("cargo:rustc-link-lib=rz_diff");
    println!("cargo:rustc-link-lib=rz_egg");
    println!("cargo:rustc-link-lib=rz_flag");
    println!("cargo:rustc-link-lib=rz_hash");
    println!("cargo:rustc-link-lib=rz_il");
    println!("cargo:rustc-link-lib=rz_io");
    println!("cargo:rustc-link-lib=rz_lang");
    println!("cargo:rustc-link-lib=rz_magic");
    println!("cargo:rustc-link-lib=rz_main");
    println!("cargo:rustc-link-lib=rz_parse");
    println!("cargo:rustc-link-lib=rz_reg");
    println!("cargo:rustc-link-lib=rz_search");
    println!("cargo:rustc-link-lib=rz_reg");*/

    println!("cargo:rustc-link-search=native={}", build_path);

    // Generate Rizin bindings

    // println!("cargo:rustc-link-search=/opt/homebrew/lib/");
    println!("cargo:rerun-if-changed=wrapper.h");

    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .blacklist_item("FP_NAN")
        .blacklist_item("FP_INFINITE")
        .blacklist_item("FP_ZERO")
        .blacklist_item("FP_SUBNORMAL")
        .blacklist_item("FP_NORMAL")
        .clang_arg("-Irizin/librz/include")
        .clang_arg("-Irizin/librz/util/sdb/src/")
        .clang_arg("-Irizin/build_include")
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
        run_command(lib, "meson", &["build_include", dir]);
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
