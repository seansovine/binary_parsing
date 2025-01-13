use libc::{c_char};

#[link(name = "BfdWrapper")]
extern "C" {
  fn try_find_arch(arch: *const c_char) -> bool;
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_bfd_scan_arch() {
    // Make sure we can call our C function and get expected result.

    let mut v = String::from("i386:x86-64\x00").into_bytes();
    let ptr = v.as_mut_ptr() as *mut i8;

    let result = unsafe {
      try_find_arch(ptr)
    };

    let status = if result { "was" } else { "was not" };
    println!("Architecture {} found.", status);

    let mut v = String::from("made-up-arch\x00").into_bytes();
    let ptr = v.as_mut_ptr() as *mut i8;

    let result = unsafe {
      try_find_arch(ptr)
    };

    let status = if result { "was" } else { "was not" };
    println!("Architecture {} found.", status);
  }
}
