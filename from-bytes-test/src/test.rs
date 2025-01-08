/// Some functions for testing the use of syn and quote crates
/// interactively using the debugger.
///
use quote::quote;

use syn::Data::Struct;
use syn::Fields::Named;
use syn::{DataStruct, DeriveInput, Expr, ExprLit, FieldsNamed, Lit, Type};

pub fn test_syn() {
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

fn field_type_name(field: &syn::Field) -> String {
    match &field.ty {
        Type::Path(p) => p.path.segments.first().unwrap().ident.to_string(),

        Type::Array(_) => "array".into(),

        _ => unimplemented!("Field type not supported by this macro."),
    }
}

fn array_len(array: &syn::TypeArray) -> usize {
    let Expr::Lit(ExprLit {
                      lit: Lit::Int(i),
                      attrs: _,
                  }) = &array.len
    else {
        panic!()
    };

    i.base10_parse().unwrap()
}
