use std::path::Path;
use std::env;

fn main() {
    let dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let rel_path = String::from("../binutils/binutils-build/bfd/.libs/");
    println!("cargo:rustc-link-search=native={}", Path::new(&dir).join(rel_path).display());
}
