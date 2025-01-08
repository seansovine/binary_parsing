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

### `#[derive(FromBytes)]` macro experiment:

The branch `parse_macro` has some code for a procedural `derive` macro
for structs of the type used for representing low-level header data in this project.
This macro applies to a struct of the appropriate kind (as used in this
project) and will add a method to parse a slice of bytes into an instance of
the struct.

What we have so far adds a zero-initializer method appropriate
to the struct, but with a little more work (mostly understanding the `syn`
crate), we should be able to generate the desired `parse_from_bytes` method.
In writing this I'm closely following Sam Van Overmeire's book _Write Powerful
Rust Macros_. I surely have much to learn about best practices for procedural
macros, however.

There is also a project for testing the use of the macro. The `cargo expand`
command from the `cargo-expand` crate is very useful for debugging, when run
on the test project.

## Bitmap

The [`bitmap/`](bitmap/) folder contains code to parse BMP files. Actually,
it contains a bit more than that: It has a program to read a BMP file's
headers and data, and for files in a 24-bit color format, to convert
the pixel values to greyscale intensities and then write the converted
data out to a new BMP file. But most of the work was in writing and reading
the BMP files. The main library header is in [`bmp.h`](bitmap/bmp.h).

The file history for this code is in the
[numerical_projects](https://github.com/seansovine/numerical_projects) repo.
