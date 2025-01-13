cmake_minimum_required(VERSION 3.20...3.31)

project(
		BfdWrapper
		VERSION 1.0
		LANGUAGES C
)


# Configure project paths.
#

# Add our cmake modules folder.
list(APPEND CMAKE_MODULE_PATH "${CMAKE_CURRENT_SOURCE_DIR}/cmake")

# Get parent directory of this file.
cmake_path(GET CMAKE_CURRENT_LIST_DIR PARENT_PATH PARENT_DIR)

include_directories("${PARENT_DIR}/binutils-build/bfd" "${PARENT_DIR}/binutils-gdb/include")


# Create static lib target for system zstd.
#  This is not really the best way, but it should work for now.
#  Later we can add a proper find module to look for this.

add_library(zstd STATIC IMPORTED)
set_target_properties(
		zstd
		PROPERTIES
		IMPORTED_LOCATION
		"/usr/lib/x86_64-linux-gnu/libzstd.a"
)


# Create targets for static libs built by binutils.
#  These are dependencies of the libbfd.

add_library(libiberty STATIC IMPORTED)
set_target_properties(
		libiberty
		PROPERTIES
		IMPORTED_LOCATION
		"${PARENT_DIR}/binutils-build/libiberty/libiberty.a"
)

add_library(libsframe STATIC IMPORTED)
set_target_properties(
		libsframe
		PROPERTIES
		IMPORTED_LOCATION
		"${PARENT_DIR}/binutils-build/libsframe/.libs/libsframe.a"
)

add_library(zlib STATIC IMPORTED)
set_target_properties(
		zlib
		PROPERTIES
		IMPORTED_LOCATION
		"${PARENT_DIR}/binutils-build/zlib/libz.a"
)

# Create a static lib target from pre-built libbfd.
#

add_library(bfd STATIC IMPORTED)
set_target_properties(
		bfd
		PROPERTIES
		IMPORTED_LOCATION
		"${PARENT_DIR}/binutils-build/bfd/.libs/libbfd.a"
)
target_link_libraries(bfd INTERFACE libiberty libsframe zlib zstd)

# Add wrapper library exposing some bfd functionality.
#

add_library(
		BfdWrapper
		bfd_wrapper.c
)
target_include_directories(BfdWrapper PUBLIC ${CMAKE_CURRENT_SOURCE_DIR})
target_link_libraries(BfdWrapper PRIVATE bfd)

# Build our test program.
#

add_executable(bfd_test test.c)
target_link_libraries(bfd_test PRIVATE BfdWrapper)
