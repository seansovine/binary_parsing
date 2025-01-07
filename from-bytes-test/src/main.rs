/// Test program for the FromBytes derive macro.
///
/// Created by sean on 1/7/25.
///

#[macro_use]
extern crate from_bytes_macro;

#[derive(FromBytes, Debug)]
pub struct Elf64Header {
  // 8 bytes
  pub magic_bytes: [u8; 4],
  pub bitness: u8,
  pub endianness: u8,
  pub elf_version: u8,
  pub abi_os: u8,
  // 8 bytes
  pub abi_version: u8,
  pub abi_padding: [u8; 7],
  // 8 bytes
  pub object_type: [u8; 2],
  pub machine: [u8; 2],
  pub version: u32,
  // 8 bytes
  pub entry_point: u64,
  // 8 bytes
  pub program_header_offset: u64,
  // 8 bytes
  pub section_header_offset: u64,
  // 8 bytes
  pub flags: u32,
  pub header_size: u16,
  pub program_header_entry_size: u16,
  // 8 bytes
  pub program_header_entry_count: u16,
  pub section_header_entry_size: u16,
  pub section_header_entry_count: u16,
  pub section_header_names_index: u16,
}

fn main() {
  println!("Testing dummy parse method:");

  let dummy_data = [0u8; 64];

  let strct = Elf64Header::parse_from_bytes(&dummy_data);

  println!("{:?}", strct);
}
