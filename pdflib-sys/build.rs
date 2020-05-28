extern crate bindgen;

use std::env;
use std::path::PathBuf;

fn main() {
    // for (k, v) in env::vars() {
    //     eprintln!("{:?} -> {:?}", k, v);
    // }
    env::set_var("LLVM_CONFIG_PATH", "/usr/local/opt/llvm/bin/llvm-config");

    if let Ok(pdflibdir) = env::var("PDFLIB_DIR") {
        println!("cargo:rustc-link-search=native={}/bind/c/lib", pdflibdir);
    } else {
        println!("cargo:warning=PDFLIB_DIR env var not set. Please point PDFLIB_DIR to the directory containing pdflib.a file.");
    }
    println!("cargo:rustc-link-lib=pdf");

    let target = std::env::var("TARGET").unwrap();
    if target.contains("apple") {
        println!("cargo:rustc-link-lib=dylib=c++");
        println!("cargo:rustc-link-lib=framework=ApplicationServices");
    } else if target.contains("linux") {
        println!("cargo:rustc-link-lib=dylib=stdc++");
    } else if target.contains("windows") {
        println!("cargo:rustc-link-lib=dylib=shell32");
    }

    // Tell cargo to invalidate the built crate whenever the wrapper changes
    println!("cargo:rerun-if-changed=wrapper.h");

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header("wrapper.h")
        .clang_arg(format!(
            "-I{}/bind/c/include",
            env::var("PDFLIB_DIR").unwrap_or_else(|_| ".".to_string())
        ))
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
