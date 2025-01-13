// libbfd requires this to be included first.
#include "config.h"

#include "bfd.h"

int main() {
  const char* archName = "x86_64";
  bfd_scan_arch(archName);

  // TODO: We will handle the return value later.
  // First we just want to get the dependencies straightened
  // out so that this compiles.
}
