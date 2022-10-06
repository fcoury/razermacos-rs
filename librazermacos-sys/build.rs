use std::env;
use std::path::PathBuf;
use std::process::Command;

fn main() {
    let output = Command::new("/usr/bin/xcrun")
        .arg("--sdk")
        .arg("macosx")
        .arg("--show-sdk-path")
        .output()
        .unwrap()
        .stdout;
    let prefix_str = std::str::from_utf8(&output).expect("invalid output from `xcrun`");
    let prefix = prefix_str.trim_end();

    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .clang_arg(format!("-isysroot{}", &prefix))
        .clang_arg("-I")
        .clang_arg("librazermacos/src/include")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");

    Command::new("make")
        .current_dir("librazermacos")
        .status()
        .expect("failed to build librazermacos");

    println!("cargo:rustc-link-search={}", out_path.display());
    println!("cargo:rustc-link-lib=razermacos");
    println!("cargo:rerun-if-changed=wrapper.h");
}
