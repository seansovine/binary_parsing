/// Rudimentary parser for ELF binaries.
///
/// Created by sean on 1/1/25.
///
mod parse;
mod file_read;

use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str;

use crate::parse::{read_header_64, Elf64Header};
use crate::file_read::FileReader;

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

  reader.ensure_length(64)?;
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

  let header: Elf64Header;

  // Ensure that file is 64-bit ELF.
  if buffer[4] == b'\x02' {
    header = read_header_64(buffer);
  } else {
    return Err("Reader for 32-bit ELF files is not implemented.".to_string());
  }

  // Pretty print struct in hex.
  println!("\nHeader data: {:#04x?}", header);

  // --------
  // Success!

  Ok(())
}
