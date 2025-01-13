#include "bfd_wrapper.h"

#include <stdio.h>

int main() {
  const char *archName = "i386:x86-64";
  bool success = try_find_arch(archName);

  const char *message = success ? "was" : "was not";
  printf("The architecture '%s' %s found by libbfd.", archName, message);

  return 0;
}
