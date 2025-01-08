/// Test program for the FromBytes derive macro.
///
/// Created by sean on 1/7/25.
///
use quote::quote;

use syn::Data::Struct;
use syn::Fields::Named;
use syn::__private::TokenStream2;
use syn::{parse_macro_input, DataStruct, DeriveInput, Expr, FieldsNamed, Lit, Type};

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
    println!("\nTesting dummy parse method:\n");

    const TEST_MACRO: bool = true;

    if TEST_MACRO {
        let dummy_data = [0u8; 64];
        let strct = Elf64Header::parse_from_bytes(&dummy_data);
        println!("{:?}", strct);
    }

    // Now let's test quote and syn where we can
    // debug them directly, to try to learn more
    // about how they work.

    const TEST_SYN: bool = false;

    if TEST_SYN {
        test_syn();
    }
}

fn field_type_name(field: &syn::Field) -> String {
    match &field.ty {
        Type::Path(p) => {
            // println!("'Path' field data: {:#?}", p);

            p.path.segments.first().unwrap().ident.to_string()
        }

        Type::Array(_) => {
            // println!("'Array' field data: {:#?}", a);

            "array".into()
        }

        _ => unimplemented!("Type not supported"),
    }
}

fn array_len(array: &syn::TypeArray) -> usize {
    let Expr::Lit(l) = &array.len else { panic!() };
    let Lit::Int(i) = &l.lit else { panic!() };

    i.base10_parse().unwrap()
}

fn test_syn() {
    let test_syntax = quote! {
      pub struct Elf64Header {
        pub magic_bytes: [u8; 4],
        pub bitness: u8,
        pub endianness: u64,
        pub version: u32,
        pub other_array: [u8; 6],
      }
    }
    .into();

    let ast: DeriveInput = syn::parse2(test_syntax).unwrap();

    let _name = ast.ident;

    let fields = match ast.data {
        Struct(DataStruct {
            fields: Named(FieldsNamed { ref named, .. }),
            ..
        }) => named,
        _ => unimplemented!("Macro only implemented for structs with named fields."),
    };

    let mut current_byte: usize = 0;
    let builder_fields = fields.iter().map(|f| {
        let idnt = field_type_name(f);
        println!("Found field with typename: '{}'", idnt);

        let result = if let Type::Array(ta) = &f.ty {
            let length = array_len(ta);
            println!(" Array has length: '{}'", length);

            let last_current = current_byte;
            current_byte += length;

            quote! { buffer[#last_current..#current_byte].try_into().unwrap() }
        } else {
            let last_current = current_byte;
            match field_type_name(f).as_str() {
                "u8" => {
                    current_byte += 1;
                }

                "u16" => {
                    current_byte += 2;
                }

                "u32" => {
                    current_byte += 4;
                }

                "u64" => {
                    current_byte += 8;
                }

                _ => {
                    panic!()
                }
            };

            let ty = &f.ty;

            quote! {
                <#ty>::from_le_bytes(
                    buffer[#last_current..#current_byte]
                    .try_into()
                    .unwrap(),
                )
            }
        };

        println!(" -- We've now read this many bytes: {}", current_byte);
        result
    });

    let strings = builder_fields
        .collect::<Vec<_>>()
        .iter()
        .map(|f| f.to_string())
        .collect::<Vec<_>>();

    println!("\nResult: {:#?}", strings);
}
