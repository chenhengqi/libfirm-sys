use std::env;
use std::path;
use std::process;

#[cfg(debug_assertions)]
static VARIANT: &str = "debug";

#[cfg(not(debug_assertions))]
static VARIANT: &str = "optimize";

fn main() {
    let src_dir = path::PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").unwrap());
    let libfirm_dir = src_dir.join("libfirm");

    let status = process::Command::new("make")
        .arg(format!("variant={}", VARIANT))
        .current_dir(&libfirm_dir)
        .status()
        .expect("could not execute make");

    assert!(status.success(), "make failed");

    println!("cargo:include={}/include/libfirm", libfirm_dir.display());
    println!(
        "cargo:include={}/build/gen/include/libfirm",
        libfirm_dir.display()
    );
    println!(
        "cargo:rustc-link-search={}/build/{}",
        libfirm_dir.display(),
        VARIANT
    );
    println!("cargo:rustc-link-lib=static=firm");

    bindgen::Builder::default()
        .header("bindings.h")
        .clang_arg(format!("-I{}/include/libfirm", libfirm_dir.display()))
        .clang_arg(format!(
            "-I{}/build/gen/include/libfirm",
            libfirm_dir.display()
        ))
        .generate()
        .expect("Unable to generate bindings")
        .write_to_file(src_dir.join("src/bindings.rs"))
        .expect("Couldn't write bindings");
}
