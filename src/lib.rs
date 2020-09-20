extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Data};

#[proc_macro_derive(From)]
pub fn derive(input: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree
    let input = parse_macro_input!(input as DeriveInput);

    let ident = input.ident;
    let mut fields = vec![];
    if let Data::Struct(data) = input.data {
        for field in data.fields {
            let ident = field.ident;
            fields.push(quote!{self.#ident = other.#ident;});
        }
    }
    // Build the output, possibly using quasi-quotation
    let expanded = quote! {
        impl #ident {
            fn from(&mut self, other: Self) {
                #(#fields)*
            }
        }
    };

    // Hand the output tokens back to the compiler
    TokenStream::from(expanded)
}