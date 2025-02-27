/// Rudimentary parser for ELF binaries.
///
/// Created by sean on 1/1/25.
///
mod file_read;
mod parse;

use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str;

use crate::file_read::FileReader;
use crate::parse::*;

use colored::Colorize;

#[macro_use]
extern crate from_bytes_macro;

// ------------
// Some config.

// TODO: Path hardcoded for testing.
const FILE: &str = "/home/sean/Code/A-K/binary_parsing/elf/test/AudioPlayer";

// -------------------
// Program entrypoint.

fn main() -> Result<(), String> {
    println!("Parsing binary file: {}", FILE);

    // -----------------------------
    // Create reader for file bytes.

    let file = File::open(FILE).unwrap();

    // In case we're reading a large file, we don't read it into memory all at once.
    let mut reader = FileReader::new(file);

    let mut bytes_needed = ELF64_HEADER_LEN;
    reader.ensure_length(bytes_needed)?;

    let buffer = reader.buffer();

    // -------------------
    // Verify magic bytes.

    if &buffer[..4] == b"\x7F\x45\x4c\x46" {
        println!("Found ELF magic bytes; will continue parsing file as ELF.");
    } else {
        println!("Did not find ELF magic bytes; aborting file parse.");

        return Err("Parse failed: Bad file format.".to_string());
    }

    // ----------------
    // Read ELF header.

    // Ensure that file is 64-bit ELF; we currently only support that combo.
    let elf_header: Elf64Header = if buffer[4] == b'\x02' && buffer[5] == b'\x01' {
        Elf64Header::parse_from_bytes(buffer)
    } else {
        return Err(
            "This reader currently only supports 64-bit little endian ELF files.".to_string(),
        );
    };

    println!("\n>> {} <<", "ELF main header.".red());
    println!("\n{}: {:#04x?}", "Header data".green().bold(), elf_header);

    // ---------------------
    // Read program headers.

    let program_header_size = elf_header.program_header_entry_count as usize
        * elf_header.program_header_entry_size as usize;

    bytes_needed = elf_header.program_header_offset as usize + program_header_size;
    reader.ensure_length(bytes_needed)?;

    let program_headers = read_program_headers_64(reader.buffer(), &elf_header);
    println!("\n>> {} <<", "Program headers.".red());

    program_headers.iter().for_each(|program_header| {
        println!(
            "\n{} type: {}",
            "Program header".blue().bold(),
            program_header.type_string
        );
        println!("Data: {:#04x?}", program_header.header_data);
    });

    // ---------------------
    // Read section headers.

    let section_header_size = elf_header.section_header_entry_size as usize
        * elf_header.section_header_entry_count as usize;

    bytes_needed = elf_header.section_header_offset as usize + section_header_size;
    reader.ensure_length(bytes_needed)?;

    let section_headers = read_section_headers_64(&mut reader, &elf_header);
    println!("\n>> {} <<", "Section headers.".red());

    section_headers.iter().for_each(|section_header| {
        println!(
            "\n{} type: {}",
            "Section header".yellow().bold(),
            section_header.type_string
        );
        println!("Section header name: {}", section_header.name);
        println!("Data: {:#04x?}", section_header.header_data);
    });

    // --------
    // Success!

    Ok(())
}
