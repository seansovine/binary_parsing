/// Tools for parsing ELF files.
///
mod utils;

use crate::file_read::FileReader;
use crate::parse::utils::*;

use from_bytes_macro::FromBytes;
use std::mem::size_of;

// ----------------
// Main ELF header.

pub const ELF64_HEADER_LEN: usize = 64;

/// To start with, a low-level (uninterpreted)
/// representation of the data in the header.
#[derive(Debug, FromBytes)]
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

// ---------------------
// Program header table.

#[derive(Debug, FromBytes)]
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

        let ph = Elf64ProgramHeaderEntry::parse_from_bytes(&buffer[start_offset..end_offset]);

        let type_string = program_header_type_string(&ph.segment_type);

        entries.push(Elf64ProgramHeaderInfo {
            header_data: ph,
            type_string: type_string,
        });
    }

    entries
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

#[derive(Debug, FromBytes)]
pub struct Elf64SectionHeaderEntry {
    pub name_offset: u32,
    pub section_type: [u8; 4],
    pub flags: u64,
    pub addr: u64,
    pub offset: u64,
    pub size: u64,
    pub link: u32,
    pub info: u32,
    pub addr_align: u64,
    pub entry_size: u64,
}

#[derive(Debug)]
pub struct Elf64SectionHeaderInfo {
    // Raw data from file.
    pub header_data: Elf64SectionHeaderEntry,

    // Structured, extracted data.
    pub name: String,
    pub type_string: String,
}

pub fn read_section_headers_64(
    reader: &mut FileReader,
    elf_header: &Elf64Header,
) -> Vec<Elf64SectionHeaderInfo> {
    let mut buffer = reader.buffer();

    let mut entries = read_section_header_entries_64(&buffer, &elf_header);
    let mut headers: Vec<Elf64SectionHeaderInfo> = vec![];

    let mut string_table_entries: Vec<&Elf64SectionHeaderEntry> = vec![];

    // Find string table entries.
    for entry in &entries {
        let type_string = section_header_type_string(&entry.section_type);
        if type_string == "SHT_STRTAB" {
            string_table_entries.push(entry);
        }
    }

    // NOTE: We assume the last table section entry is the one with section names.
    // Evidence from examples suggests this, but we should find it in the docs.

    let section_name_table_entry = string_table_entries.last().unwrap();
    let table_start = section_name_table_entry.offset as usize;
    let table_end = table_start + section_name_table_entry.size as usize;

    reader
        .ensure_length(table_end)
        .expect("Not enough bytes in file to accommodate offset and size of string table entry.");
    buffer = reader.buffer();

    let table_buffer = &buffer[table_start..table_end];

    for entry in entries {
        let type_string = section_header_type_string(&entry.section_type);
        let name_offset = entry.name_offset as usize;
        let name_string = read_string(&table_buffer, name_offset).unwrap();

        headers.push(Elf64SectionHeaderInfo {
            header_data: entry,
            name: name_string,
            type_string,
        })
    }

    headers
}

pub fn read_section_header_entries_64(
    buffer: &[u8],
    elf_header: &Elf64Header,
) -> Vec<Elf64SectionHeaderEntry> {
    let mut entries = vec![];

    let sh_offset = elf_header.section_header_offset as usize;
    let sh_size = elf_header.section_header_entry_size as usize;

    // Read each entry from the section header table.
    for i in 0..elf_header.section_header_entry_count as usize {
        let start_offset = sh_offset + i * sh_size;
        let end_offset = start_offset + sh_size;

        let sh = crate::parse::Elf64SectionHeaderEntry::parse_from_bytes(
            &buffer[start_offset..end_offset],
        );
        entries.push(sh)
    }

    entries
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
