use proc_macro::TokenStream;

use quote::{quote, quote_spanned};

use syn::Data::Struct;
use syn::Fields::Named;
use syn::{parse_macro_input, DataStruct, DeriveInput, Expr, FieldsNamed, Lit, Type, TypeArray};

struct FieldData {
    name: String,
    type_name: RecognizedField,
}

enum RecognizedField {
    U8Array(usize),
    U8,
    U16,
    U32,
    U64,
}

#[proc_macro_derive(FromBytes)]
pub fn hello(item: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(item as DeriveInput);
    let name = ast.ident;

    let fields = match ast.data {
        Struct(DataStruct {
            fields: Named(FieldsNamed { ref named, .. }),
            ..
        }) => named,
        _ => unimplemented!("Macro only implemented for structs with named fields."),
    };

    let builder_fields = fields.iter().map(|f| {
        let fname = &f.ident;
        let ty = &f.ty;

        let type_info = format!("{:?}", ty);

        let init_fn = match ty {
            Type::Array(TypeArray { elem, len, .. }) => {
                let n = if let Expr::Lit(l) = len {
                    if let Lit::Int(i) = &l.lit {
                        i //.base10_parse::<usize>()
                    } else {
                        panic!("Expected int literal.");
                    }
                } else {
                    panic!("Expected literal expression for array length.")
                };

                quote! { [0; #n] }
            },

            _ => quote! { 0 },
        };

        quote! {  #fname: #init_fn }
    }); //.collect::<Vec<_>>();

    // TODO: This initial version generates a zero-initializer.
    // In the next version, we need to iterate over the fields
    // and use the type information along with a running byte
    // total to actually generate the appropriate function calls
    // to initialize the data from the provided byte slice.

    let parse_method = quote! {
        impl #name {
            pub fn parse_from_bytes(bytes: &[u8]) -> Self {
                Self{
                    #(#builder_fields,)*
                }
            }
        }
    };

    parse_method.into()
}
