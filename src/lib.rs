use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(SerdeJsonInto)]
pub fn derive_serde_json_into(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    let expanded = quote! {
        impl From<#name> for Vec<u8> {
            fn from(value: #name) -> Self {
                serde_json::to_vec(&value).expect("Failed to serialize to Vec<u8>")
            }
        }

        impl<'a> From<&'a #name> for Vec<u8> {
            fn from(value: &'a #name) -> Self {
                serde_json::to_vec(value).expect("Failed to serialize to Vec<u8>")
            }
        }

        impl TryFrom<Vec<u8>> for #name {
            type Error = serde_json::Error;

            fn try_from(value: Vec<u8>) -> Result<Self, Self::Error> {
                serde_json::from_slice(&value)
            }
        }

        impl TryFrom<&[u8]> for #name {
            type Error = serde_json::Error;

            fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
                serde_json::from_slice(value)
            }
        }
    };

    TokenStream::from(expanded)
}
