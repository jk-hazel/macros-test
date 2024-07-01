use enum_from::enum_from;
use proc_macro::TokenStream;
//for eum, we need generate Form impls for all the enum variants
#[proc_macro_derive(EnumFrom)]
pub fn enum_from(input: TokenStream) -> TokenStream {
    enum_from.into()
}
