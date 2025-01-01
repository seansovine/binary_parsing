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
