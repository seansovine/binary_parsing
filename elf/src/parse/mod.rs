/// Tools for parsing ELF files.
///
use std::mem::size_of;

// ----------------
// Main ELF header.

/// To start with, a low-level (uninterpreted)
/// representation of the data in the header.
#[derive(Debug)]
pub struct Elf64Header {
    // 8 bytes
    magic_bytes: [u8; 4],
    bitness: u8,
    endianness: u8,
    elf_version: u8,
    abi_os: u8,
    // 8 bytes
    abi_version: u8,
    abi_padding: [u8; 7],
    // 8 bytes
    object_type: [u8; 2],
    machine: [u8; 2],
    version: u32,
    // 8 bytes
    entry_point: u64,
    // 8 bytes
    program_header_offset: u64,
    // 8 bytes
    section_header_offset: u64,
    // 8 bytes
    flags: u32,
    header_size: u16,
    program_header_entry_size: u16,
    // 8 bytes
    program_header_entry_count: u16,
    section_header_entry_size: u16,
    // 8 bytes
    section_header_entry_count: u16,
    section_header_names_index: u16,
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
        //
        section_header_entry_count: u16_from_le_slice(buffer, 60),
        section_header_names_index: u16_from_le_slice(buffer, 62),
        //
    }
}

// ---------------------
// Program header table.

pub struct Elf64ProgramHeaderEntry {
    // TODO: Add fields.
}

pub fn read_program_header_64(buffer: &[u8]) -> Vec<Elf64ProgramHeaderEntry> {
    let entries = vec![];

    // TODO: Read each entry from the program header table.

    entries
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
