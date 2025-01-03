/// Tools for parsing ELF files.
///
use std::mem::size_of;

// ----------------
// Main ELF header.

pub const ELF64_HEADER_LEN: usize = 64;

/// To start with, a low-level (uninterpreted)
/// representation of the data in the header.
#[derive(Debug)]
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

pub fn read_header_64(buffer: &[u8]) -> Elf64Header {
  Elf64Header {
    //
    magic_bytes: buffer[0..4].try_into().unwrap(),
    bitness: buffer[4],
    endianness: buffer[5],
    elf_version: buffer[6],
    abi_os: buffer[7],
    //
    abi_version: buffer[8],
    abi_padding: buffer[9..16].try_into().unwrap(),
    //
    object_type: buffer[16..18].try_into().unwrap(),
    machine: buffer[18..20].try_into().unwrap(),
    version: u32_from_le_slice(buffer, 20),
    //
    entry_point: u64_from_le_slice(buffer, 24),
    //
    program_header_offset: u64_from_le_slice(buffer, 32),
    //
    section_header_offset: u64_from_le_slice(buffer, 40),
    //
    flags: u32_from_le_slice(buffer, 48),
    header_size: u16_from_le_slice(buffer, 52),
    program_header_entry_size: u16_from_le_slice(buffer, 54),
    //
    program_header_entry_count: u16_from_le_slice(buffer, 56),
    section_header_entry_size: u16_from_le_slice(buffer, 58),
    section_header_entry_count: u16_from_le_slice(buffer, 60),
    section_header_names_index: u16_from_le_slice(buffer, 62),
    //
  }
}

// ---------------------
// Program header table.

#[derive(Debug)]
pub struct Elf64ProgramHeaderEntry {
  pub segment_type: [u8; 4],
  pub flags: u32,
  //
  pub offset: u64,
  pub virtual_address: u64,
  pub physical_address: u64,
  pub file_size: u64,
  pub mem_size: u64,
  pub align: u64,
}

pub struct Elf64ProgramHeaderInfo {
  // Raw data from file.
  pub header_data: Elf64ProgramHeaderEntry,
  // Structured, extracted data.
  pub type_string: String,
}

pub fn read_program_headers_64(
  buffer: &[u8],
  elf_header: &Elf64Header,
) -> Vec<Elf64ProgramHeaderInfo> {
  let mut entries = vec![];

  let ph_offset = elf_header.program_header_offset as usize;
  let ph_size = elf_header.program_header_entry_size as usize;

  // Read each entry from the program header table.
  for i in 0..elf_header.program_header_entry_count as usize {
    let start_offset = ph_offset + i * ph_size;
    let end_offset = start_offset + ph_size;

    let ph = read_program_header_64(&buffer[start_offset..end_offset]);

    let type_string = program_header_type_string(&ph.segment_type);

    entries.push(Elf64ProgramHeaderInfo {
      header_data: ph,
      type_string: type_string,
    });
  }

  entries
}

pub fn read_program_header_64(buffer: &[u8]) -> Elf64ProgramHeaderEntry {
  Elf64ProgramHeaderEntry {
    segment_type: buffer[0..4].try_into().unwrap(),
    flags: u32_from_le_slice(&buffer, 4),
    //
    offset: u64_from_le_slice(&buffer, 8),
    virtual_address: u64_from_le_slice(&buffer, 16),
    physical_address: u64_from_le_slice(&buffer, 24),
    file_size: u64_from_le_slice(&buffer, 32),
    mem_size: u64_from_le_slice(&buffer, 40),
    align: u64_from_le_slice(&buffer, 48),
  }
}

pub fn program_header_type_string(buffer: &[u8; 4]) -> String {
  let str_val = match buffer {
    // NOTE: File bytes are little endian; hence reverse order here.
    b"\x00\x00\x00\x00" => "PT_NULL",
    b"\x01\x00\x00\x00" => "PT_LOAD",
    b"\x02\x00\x00\x00" => "PT_DYNAMIC",
    b"\x03\x00\x00\x00" => "PT_INTERP",
    b"\x04\x00\x00\x00" => "PT_NOTE",
    b"\x05\x00\x00\x00" => "PT_SHLIB",
    b"\x06\x00\x00\x00" => "PT_PHDR",
    b"\x07\x00\x00\x00" => "PT_TLS",

    _ => {
      let buff_in_be: Vec<&u8> = buffer.iter().rev().collect();
      let type_str = match buffer {
        buf if buf[3] & b'\xf0' == b'\x60' => "OS_SPECIFIC",
        buf if buf[3] & b'\xf0' == b'\x70' => "PROCESSOR_SPECIFIC",

        _ => "OTHER",
      };

      &format!("{type_str}: {:02x?}", buff_in_be)
    }
  };

  str_val.to_owned()
}

// ---------------------
// Section header table.

#[derive(Debug)]
pub struct Elf64SectionHeaderEntry {
  pub name_offset: u32,
  pub section_type: [u8; 4],
  // TODO: Add remaining fields.
}

#[derive(Debug)]
pub struct Elf64SectionHeaderInfo {
  // Raw data from file.
  pub header_data: Elf64SectionHeaderEntry,
  // Structured, extracted data.
  pub type_string: String,
}

pub fn read_section_headers_64(
  buffer: &[u8],
  elf_header: &Elf64Header,
) -> Vec<Elf64SectionHeaderInfo> {
  let mut entries = vec![];

  let sh_offset = elf_header.section_header_offset as usize;
  let sh_size = elf_header.section_header_entry_size as usize;

  // Read each entry from the program header table.
  for i in 0..elf_header.section_header_entry_count as usize {
    let start_offset = sh_offset + i * sh_size;
    let end_offset = start_offset + sh_size;

    let sh = read_section_header_64(&buffer[start_offset..end_offset]);

    let type_string = section_header_type_string(&sh.section_type);

    entries.push(Elf64SectionHeaderInfo{
      header_data: sh,
      type_string: type_string,
    })
  }

  entries
}

pub fn read_section_header_64(buffer: &[u8]) -> Elf64SectionHeaderEntry {
  Elf64SectionHeaderEntry {
    name_offset: u32_from_le_slice(&buffer, 4),
    section_type: buffer[4..8].try_into().unwrap(),

    // TODO: Implement rest of section header parsing.
  }
}

pub fn section_header_type_string(buffer: &[u8; 4]) -> String {
  let str_val = match buffer {
    // NOTE: File bytes are little endian; hence reverse order here.
    b"\x00\x00\x00\x00" => "SHT_NULL",
    b"\x01\x00\x00\x00" => "SHT_PROGBITS",
    b"\x02\x00\x00\x00" => "SHT_SYNTAB",
    b"\x03\x00\x00\x00" => "SHT_STRTAB",
    b"\x04\x00\x00\x00" => "SHT_RELA",
    b"\x05\x00\x00\x00" => "SHT_HASH",
    b"\x06\x00\x00\x00" => "SHT_DYNAMIC",
    b"\x07\x00\x00\x00" => "SHT_NOTE",
    b"\x08\x00\x00\x00" => "SHT_NOBITS",
    b"\x09\x00\x00\x00" => "SHT_REL",
    b"\x0A\x00\x00\x00" => "SHT_SHLIB",
    b"\x0B\x00\x00\x00" => "SHT_DYNSYM",
    b"\x0E\x00\x00\x00" => "SHT_INIT_ARRAY",
    b"\x0F\x00\x00\x00" => "SHT_FINI_ARRAY",
    b"\x10\x00\x00\x00" => "SHT_PREINIT_ARRAY",
    b"\x11\x00\x00\x00" => "SHT_GROUP",
    b"\x12\x00\x00\x00" => "SHT_SYMTAB_SHNDX",
    b"\x13\x00\x00\x00" => "SHT_NUM",

    // 0x60000000 and above are OS-specific.
    buf if buf[3] & b'\xf0' >= b'\x60' => "OS_SPECIFIC",

    _ => {
      let buff_in_be: Vec<&u8> = buffer.iter().rev().collect();
      &format!("UNRECOGNIZED TYPE: {:02x?}", buff_in_be)
    }
  };

  str_val.to_owned()
}

// -----------------------------------------
// Some utility functions for reading bytes.

fn u16_from_le_slice(slice: &[u8], start: usize) -> u16 {
  let bound = start + size_of::<u16>();
  let array = slice[start..bound].try_into().unwrap();
  u16::from_le_bytes(array)
}

fn u32_from_le_slice(slice: &[u8], start: usize) -> u32 {
  let bound = start + size_of::<u32>();
  let array = slice[start..bound].try_into().unwrap();
  u32::from_le_bytes(array)
}

fn u64_from_le_slice(slice: &[u8], start: usize) -> u64 {
  let bound = start + size_of::<u64>();
  let array = slice[start..bound].try_into().unwrap();
  u64::from_le_bytes(array)
}
