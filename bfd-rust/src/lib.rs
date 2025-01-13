use libc::c_char;

#[link(name = "bfd")]
extern "C" {
  fn bfd_scan_arch(arch: *const c_char) -> *const c_char;
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_bfd_scan_arch() {
    // TODO: We need to call our function and see what happens.
  }
}
