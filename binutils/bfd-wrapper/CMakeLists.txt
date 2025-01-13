cmake_minimum_required(VERSION 3.20...3.31)

project(
	BfdWrapper
	VERSION 1.0
	LANGUAGES C)

cmake_path(GET CMAKE_CURRENT_LIST_DIR PARENT_PATH PARENT_DIR)

include_directories("${PARENT_DIR}/binutils-build/bfd" "${PARENT_DIR}/binutils-gdb/include")

add_library(libiberty STATIC IMPORTED)
set_target_properties(libiberty PROPERTIES IMPORTED_LOCATION "${PARENT_DIR}/binutils-build/libiberty/libiberty.a")

add_library(bfd STATIC IMPORTED)
set_target_properties(bfd PROPERTIES IMPORTED_LOCATION "${PARENT_DIR}/binutils-build/bfd/.libs/libbfd.a")
target_link_libraries(bfd INTERFACE libiberty)

add_executable(bfd_test test.c)
target_link_libraries(bfd_test PRIVATE bfd)
