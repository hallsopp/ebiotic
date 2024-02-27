//! This crate contains (WIP) procedural macros to help with concisely metadata-ing large numbers of structs and enums representing databases etc.

// use proc_macro::TokenStream;
// use quote::quote;
// use syn::{parse_macro_input, Data, DeriveInput, Meta};
//
// #[proc_macro_derive(Display, attributes(display))]
// pub fn derive_display(input: TokenStream) -> TokenStream {
//     let input = parse_macro_input!(input as DeriveInput);
//
//     let name = input.ident;
//
//     let data_enum = match input.data {
//         Data::Enum(data_enum) => data_enum,
//         _ => panic!("Display can only be implemented for enums"),
//     };
//
//     let match_arms = data_enum.variants.iter().map(|variant| {
//         let variant_ident = &variant.ident;
//         let display_attr = variant
//             .attrs
//             .iter()
//             .find_map(|attr| {
//                 Some(attr.parse_nested_meta(|meta| Ok({
//                     if meta.path.is_ident("display") {
//                         meta.value();
//                     };
//                 })))
//             })
//             .unwrap_or_else(|| variant_ident.to_string());
//
//         quote! {
//             #name::#variant_ident => write!(f, #display_attr),
//         }
//     });
//
//     let expanded = quote! {
//         impl std::fmt::Display for #name {
//             fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//                 match self {
//                     #(#match_arms)*
//                 }
//             }
//         }
//     };
//
//     TokenStream::from(expanded)
// }
