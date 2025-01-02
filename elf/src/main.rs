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

// ------------
// Some config.

// TODO: Path hardcoded for testing.
const FILE: &str = "/home/sean/Code/A-K/binary_parsing/elf/test/AudioPlayer";

const EXTRA_DEBUG: bool = false;

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

  if EXTRA_DEBUG {
    let buffer_len = buffer.len();
    println!("\nFirst {buffer_len} bytes are: {:x?}", buffer);
  }

  if &buffer[..4] == b"\x7F\x45\x4c\x46" {
    println!("Found ELF magic bytes; will continue parsing file as ELF.");
  } else {
    println!("Did not find ELF magic bytes; aborting file parse.");

    return Err("Parse failed: Bad file format.".to_string());
  }

  // ----------------
  // Read ELF header.

  let elf_header: Elf64Header;

  // Ensure that file is 64-bit ELF.
  if buffer[4] == b'\x02' {
    elf_header = read_header_64(buffer);
  } else {
    return Err("Reader for 32-bit ELF files is not implemented.".to_string());
  }

  println!("\n>> ELF main header. <<");
  // Pretty print struct in hex.
  println!("\nHeader data: {:#04x?}", elf_header);

  // ---------------------
  // Read program headers.

  let program_header_size =
    elf_header.program_header_entry_count as usize * elf_header.program_header_entry_size as usize;

  bytes_needed = elf_header.program_header_offset as usize + program_header_size;
  reader.ensure_length(bytes_needed)?;

  let program_headers = read_program_headers_64(reader.buffer(), &elf_header);
  println!("\n>> Program headers. <<");

  for program_header in program_headers {
    let segment_type = program_header_type_string(&program_header.segment_type);
    println!("\nProgram header type: {}", segment_type);
    // Pretty print struct in hex.
    println!("Data: {:#04x?}", program_header);
  }

  // ---------------------
  // Read section headers.

  let section_header_size =
    elf_header.section_header_entry_size as usize * elf_header.section_header_entry_count as usize;

  bytes_needed = elf_header.section_header_offset as usize + section_header_size;
  reader.ensure_length(bytes_needed)?;

  let section_headers = read_section_headers_64(reader.buffer(), &elf_header);
  println!("\n>> Section headers. <<");

  for section_header in section_headers {
    println!("Data: {:#04x?}", section_header);
  }

  // --------
  // Success!

  Ok(())
}
