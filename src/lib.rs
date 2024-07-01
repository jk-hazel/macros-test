mod auto;
mod enum_fom_darling;
mod enum_from;
use enum_fom_darling::process_enum_from_darling;
use enum_from::process_enum_from;
use proc_macro::TokenStream;
use syn::DeriveInput;
//for eum, we need generate Form impls for all the enum variants
#[proc_macro_derive(EnumFrom)]
pub fn enum_from(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as syn::DeriveInput);
    process_enum_from(input).into()
}

#[proc_macro_derive(EnumFromDarling)]
pub fn enum_from_darling(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as syn::DeriveInput);
    process_enum_from_darling(input).into()
}

#[proc_macro_derive(AutoDref, attributes(auto_deref))]
pub fn derive_auto_deref(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as DeriveInput);
    println!("{:#?}", input);
    proc_macro2::TokenStream::new().into()
}
