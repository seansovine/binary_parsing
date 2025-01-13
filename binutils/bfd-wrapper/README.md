# Rust-Friendly C Interface for libbfd

This project will build a (static) library that provides
a simplified C interface to a subset of the `libbfd` functionality,
so that we can more easily use BFD through Rust's C FFI.

We get `libbfd` from GNU the binutils project, and we just build
it by building binutils and grabbing any libraries we need from its
build artifacts.

## To Do

__Figure out how to build binutils without optimization.__

We can possibly do this by just editing the top-level Makefile.
But it might be better if we can find an option in the `configure`
script to leave the `-O2` flag out of the C compiler flags list.

__Consider adding binutils build / CMake calls to Rust build script.__

And last but not least:

__Figure out how `objdump` really uses `libbfd` and imitate.__

We want to figure out the process `objdump` uses to disassemble
sections of data from an ELF file. We have found the functions in
`objdump.c` where it interfaces with `libbfd`. So we could possibly
rewrite these functions appropriately and call them from our own
interface function. To help with this we can run `objdump` on an
ELF file and step through the execution using GDB, to better see
how things are working in an example.

If we take the approach just described, our goal in the Rust code should be
to pass into the C interface a pointer to some bytes and a length, and
whatever other config parameters are needed, and then from the C code
get back maybe an array of (pointer to) strings that contain assembly
instructions extracted from the section's data.
