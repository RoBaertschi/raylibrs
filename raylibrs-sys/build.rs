use std::{path::PathBuf, env};

extern crate bindgen;

fn main() {
    let raylib = vcpkg::find_package("raylib").unwrap();
    let mut paths = String::new();
    for item in raylib.found_libs {
        paths += item.to_str().unwrap();
        paths += ";";
    }
    println!("cargo:rustc-link-search={}", paths);
    println!("cargo:rustc-link-lib=raylib");
    println!("cargo:rerun-if-changed=wrapper.h");

    let bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header("wrapper.h")
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!")

    // println!(format!("cargo:rustc-link-search={}", r ));
}
