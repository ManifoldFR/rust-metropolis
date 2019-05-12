use cbindgen;
use std::env;

fn main() {
    let crate_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    println!("CARGO_MANIFEST_DIR: {:?}", crate_dir);

    cbindgen::Builder::new()
        .with_crate(crate_dir)
        .with_namespace("ffi")
        .generate()
        .expect("Unable to generate bindings.")
        .write_to_file("cffi/bindings.hpp");
}
