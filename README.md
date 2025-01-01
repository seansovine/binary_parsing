# Project for Parsing Various Binary Formats

Part of our effort to learn more about compilers, operating systems,
and other low-level bits and pieces.
We will start with ELF files, but we plan to work with others.

## ELF

[Here](elf/src/main.rs) we have the start of a Rust program for parsing
ELF-format binary files.

So far it verifies the ELF magic bytes and parses a few header fields.
Ambitiously, we'd eventually like to get it to the point of extracting
a vector of processor instructions from the binary data. It will be
educational to work towards that, in any case. Even if it only parses
the ELF header and the section headers and displays those in a readable
format, it will be a useful project to work on.

## Bitmap

The [`bitmap/`](bitmap/) folder contains code to parse BMP files. Actually,
it contains a bit more than that: It has a program to read a BMP file's
headers and data, and for files in a 24-bit color format, to convert
the pixel values to greyscale intensities and then write the converted
data out to a new BMP file. But most of the work was in writing and reading
the BMP files. The main library header is in [`bmp.h`](bitmap/bmp.h).

The file history for this code is in the
[numerical_projects](https://github.com/seansovine/numerical_projects) repo.
