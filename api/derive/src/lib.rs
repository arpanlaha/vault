use proc_macro::TokenStream;
use quote::quote;
use syn::{self, Data, Fields};

#[proc_macro_derive(ArangoDocument)]
pub fn arango_document_derive(input: TokenStream) -> TokenStream {
    let ast: syn::DeriveInput = syn::parse(input).unwrap();
    let name = ast.ident;
    let mut fields: Vec<String> = vec![];

    if let Data::Struct(derive_struct) = ast.data {
        if let Fields::Named(named_field) = derive_struct.fields {
            for name in named_field.named {
                fields.push(name.ident.unwrap().to_string());
            }
        }
    }

    let fields_string = fields.join(",");

    let gen = quote! {
        impl ArangoDocument for #name {
            fn get_insert(&self, db) -> String {
                format!("INSERT {{ {} }} INTO {}", #fields_string.split(",").map(|field| format!("{}: {}", field, self[field])).join(", "), db)
                // format!("INSERT {{ {} }} INTO {}", "hello", db)
            }
        }
    };

    gen.into()
}
