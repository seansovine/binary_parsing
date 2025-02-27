# Project for Parsing Various Binary Formats

Part of our effort to learn more about compilers, operating systems,
and other low-level bits and pieces.
We will start with ELF files, but we plan to work with others.

## ELF

[Here](elf/src/main.rs) we have the start of a Rust program for parsing
ELF-format binary files.

This reads the main, program, and section headers from a 64-bit, little
endian ELF file and prints their data to the console. It retrieves the
string names for the sections from the string table section of the file,
and it extracts the type name for each program and section header. Note
that it doesn't yet give the OS-specific program header type names,
though we plan to add those soon (most likely by finding how they're
handled in the GNU binutils `readelf` source code).

[Here](./elf/src/README.md) are a few notes
on the code for this project.

Ambitiously, I'd eventually like to get this to the point of extracting
processor instructions from the binary data. That would take
some time, but would be educational to work towards, in any case.

__Struct parsing `derive` macro:__

The subfolder `from-bytes-macro` has some code for a procedural macro `#[derive(FromBytes)]`,
for use with structs of the type used for representing low-level header data in this project.
When applied to a struct of the appropriate kind, this macro adds a static method to
parse a slice of bytes into an instance of that struct.

In writing this macro I'm closely following Sam Van Overmeire's book _Write Powerful
Rust Macros_. However, I surely have much to learn about best practices for procedural
macros, as I'm relatively new to creating them. The `from-bytes-test` subfolder has
a project for testing the use of the macro, and the `cargo expand` command from the `cargo-expand`
crate is very useful for debugging, when run on the test project.

The `#[derive(FromBytes)]` macro is now used for
the low-level parsing of header fields in the ELF parser.

__GNU binutils / BFD wrapper for Rust:__

We have an initial proof-of-concept working towards providing an interface
to some of the GNU binutils code through Rust. In particular, I've been (just) starting
to look at how `objdump` does its disassembly. It turns out it uses the BFD library
(which has an interesting history, see [Wikipedia](https://en.wikipedia.org/wiki/Binary_File_Descriptor_library)).
So I've set up some infrastructure to build and link to the binutils code, and started
writing a wrapper to expose some binutils / BFD functionality through the Rust FFI.

See the `bfd-rust` / `binutils` folders for this. What we have so far is really just
setting up infrastructure for building and linking and provides a simple wrapper for
a `libbfd` function that is called from Rust, as a test that things are working.

I chose to write a wrapper rather than to just make Rust bindings directly to
the binutils code because there is some difficulty with using function pointers and C
enum types across the Rust C FFI. The wrapper will create a simpler interface for some
functionality that is easy for Rust to interop with.

## Bitmap

The [`bitmap/`](bitmap/) folder contains code to parse BMP files. Actually,
it contains a bit more than that: It has a program to read a BMP file's
headers and data, and for files in a 24-bit color format, to convert
the pixel values to greyscale intensities and then write the converted
data out to a new BMP file. But most of the work was in writing and reading
the BMP files. The main library header is in [`bmp.h`](bitmap/bmp.h).

The file history for this code is in the
[numerical_projects](https://github.com/seansovine/numerical_projects) repo.
