//
// Created by sean on 1/13/25.
//

// -------------
// Dependencies.

// clang-format off
#include "bfd_wrapper.h"

#include "config.h" // Must be included before bfd.
#include "bfd.h"
// clang-format on

// ---------------
// Implementation.

bool try_find_arch(const char *archName) {
  const bfd_arch_info_type *result = bfd_scan_arch(archName);

  // TODO: We will handle the return value later.
  // First we just want to get the dependencies straightened
  // out so that this compiles.

  return result != NULL;
}
