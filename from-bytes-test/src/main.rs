/// Test program for the FromBytes derive macro.
///
/// Created by sean on 1/7/25.
///
mod test;

#[macro_use]
extern crate from_bytes_macro;

// ---------------------------------
// Data struct from our `elf` crate.

#[derive(FromBytes, Debug)]
pub struct Elf64Header {
    //
    pub magic_bytes: [u8; 4],
    pub bitness: u8,
    pub endianness: u8,
    pub elf_version: u8,
    pub abi_os: u8,
    //
    pub abi_version: u8,
    pub abi_padding: [u8; 7],
    //
    pub object_type: [u8; 2],
    pub machine: [u8; 2],
    pub version: u32,
    //
    pub entry_point: u64,
    //
    pub program_header_offset: u64,
    //
    pub section_header_offset: u64,
    //
    pub flags: u32,
    pub header_size: u16,
    pub program_header_entry_size: u16,
    //
    pub program_header_entry_count: u16,
    pub section_header_entry_size: u16,
    pub section_header_entry_count: u16,
    pub section_header_names_index: u16,
}

// ------------
// Test config.

const TEST_MACRO: bool = true;
const TEST_SYN: bool = false;

// --------
// Main fn.

fn main() {
    println!("\nTesting dummy parse method:\n");

    if TEST_MACRO {
        let dummy_data = [0u8; 64];
        let strct = Elf64Header::parse_from_bytes(&dummy_data);
        println!("{:#?}", strct);
    }

    // Now let's test quote and syn where we can
    // debug them directly, to try to learn more
    // about how they work.

    if TEST_SYN {
        test::test_syn();
    }
}
