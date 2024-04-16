extern crate proc_macro;


use proc_macro::TokenStream;
use quote::quote;
use syn;


#[proc_macro_attribute(HelloMacro)]
pub fn hello_macro(input: TokenStream) -> TokenStream {

    let ast = syn::parse(input).unwrap();

    impl_hello_macro(&ast)
}

fn impl_hello_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;

    let gen = quote! {
        impl HelloMacro for #name {
            fn hello_macro() {
                println!(
                    "Hello macro, from name {}!", 
                    stringify!(#name)
                )
            }
        }
    };

    gen.into() 
}