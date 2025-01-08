use proc_macro::TokenStream;

use quote::quote;

use syn::Data::Struct;
use syn::Fields::Named;
use syn::{
    parse_macro_input, DataStruct, DeriveInput, Expr, ExprLit, FieldsNamed, Lit, Type, TypeArray,
};

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

#[proc_macro_derive(FromBytes)]
pub fn parse(item: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(item as DeriveInput);
    let name = ast.ident;

    // This bit is borrowed directly from Overmeire.
    let fields = match ast.data {
        Struct(DataStruct {
            fields: Named(FieldsNamed { ref named, .. }),
            ..
        }) => named,
        _ => unimplemented!("Macro only implemented for structs with named fields."),
    };

    let mut current_byte: usize = 0;
    let builder_fields = fields.iter().map(|f| {
        let fname = &f.ident;

        let result = if let Type::Array(ta) = &f.ty {
            let length = array_len(ta);
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
                    unimplemented!("Field type not supported by this macro.");
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

        quote! { #fname: #result }
    });

    let parse_method = quote! {
        impl #name {
            pub fn parse_from_bytes(buffer: &[u8]) -> Self {
                Self{
                    #(#builder_fields,)*
                }
            }
        }
    };

    parse_method.into()
}
