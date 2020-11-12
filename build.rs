use std::path::Path;
use std::env;

fn main() {
    println!("cargo:rerun-if-changed=src/wrapper.c");
    println!("cargo:rerun-if-changed=src/bindings.h");

    let crate_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    cbindgen::Builder::new()
        .with_language(cbindgen::Language::C)
        .with_crate(crate_dir)
        .generate()
        .expect("Unable to generate bindings")
        .write_to_file("src/bindings.h");

    // let r_inc_path = Path::new("/usr/lib64/R/include");
    cc::Build::new()
        // .include(r_inc_path)
        .cargo_metadata(true)
        .file("src/wrapper.c")
        .compile("wrapper");
}

