use std::env;
use std::path::Path;

fn main() {
  let this_dir = env::var("CARGO_MANIFEST_DIR").unwrap();

  let wrapper_path = String::from("../binutils/bfd-wrapper/cmake-build-debug");
  println!(
    "cargo:rustc-link-search=native={}",
    Path::new(&this_dir).join(wrapper_path).display()
  );

  // Link bfd and its dependencies.

  let bfd_path = String::from("../binutils/binutils-build/bfd/.libs");
  println!(
    "cargo:rustc-link-search=native={}",
    Path::new(&this_dir).join(bfd_path).display()
  );
  println!("cargo:rustc-link-lib=static=bfd");

  let iberty_path = String::from("../binutils/binutils-build/libiberty");
  println!(
    "cargo:rustc-link-search=native={}",
    Path::new(&this_dir).join(iberty_path).display()
  );
  println!("cargo:rustc-link-lib=static=iberty");

  let sframe_path = String::from("../binutils/binutils-build/libsframe/.libs");
  println!(
    "cargo:rustc-link-search=native={}",
    Path::new(&this_dir).join(sframe_path).display()
  );
  println!("cargo:rustc-link-lib=static=sframe");

  let zlib_path = String::from("../binutils/binutils-build/zlib");
  println!(
    "cargo:rustc-link-search=native={}",
    Path::new(&this_dir).join(zlib_path).display()
  );
  println!("cargo:rustc-link-lib=static=z");

  // zstd is a system library.
  println!(
    "cargo:rustc-link-search=native={}",
    Path::new("/usr/lib/x86_64-linux-gnu").display()
  );
  println!("cargo:rustc-link-lib=static=zstd");
}
