use proc_macro::TokenStream;
use quote::quote;
use syn;

#[proc_macro_derive(ArangoDocument)]
pub fn arango_document_derive(input: TokenStream) -> TokenStream {
    let ast: syn::DeriveInput = syn::parse(input).unwrap();
    let name = ast.ident;

    let gen = quote! {
        impl ArangoDocument for #name {
            fn get_insert() {
                println!("Hello, Macro! My name is {}!", stringify!(#name));
            }
        }
    };

    gen.into()
}
