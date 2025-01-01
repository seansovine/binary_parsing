/// Rudimentary parser for ELF binaries.
///
/// Created by sean on 1/1/25.
///
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str;

// TODO: Path hardcoded for testing.
const FILE: &str = "/home/sean/Code/A-K/binary_parsing/elf/test/AudioPlayer";

/// To start with, a low-level (uninterpreted)
/// representation of the data in the header.
#[derive(Debug)]
struct ElfHeader {
  bitness: u8,
  endianness: u8,
  elf_version: u8,
  os_abi: u8,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
  println!("Parsing binary file: {}", FILE);

  let file = File::open(FILE).unwrap();

  const BUFFER_SIZE: usize = 8;

  // In case we're reading a large file, we
  // don't read it into memory all at once.
  let mut reader = BufReader::with_capacity(BUFFER_SIZE, file);

  let buffer = reader.fill_buf().unwrap();

  if buffer.is_empty() {
    return Err(Box::new(std::io::Error::other(
      "File is empty.",
    )));
  }

  println!("\nFirst {BUFFER_SIZE} bytes are: {:x?}", buffer);

  if &buffer[..4] == b"\x7F\x45\x4c\x46" {
    println!("Found ELF magic bytes; will continue parsing file as ELF.");
  } else {
    println!("Did not find ELF magic bytes; aborting file parse.");

    return Err(Box::new(std::io::Error::other(
      "Parse failed: Bad file format.",
    )));
  }

  // NOTE: Probably not how we'd want to end up doing this, but it's fun. See:
  // https://stackoverflow.com/questions/42499049/how-to-transmute-a-u8-buffer-to-struct-in-rust

  let header_bytes = &buffer[4..8];
  let header: ElfHeader = unsafe { std::ptr::read(header_bytes.as_ptr() as *const _) };

  println!("Header data: {:x?}", header);

  Ok(())
}
