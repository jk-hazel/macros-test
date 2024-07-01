use proc_macro2::TokenStream;
use quote::quote;

pub(crate) fn enum_from(input: DeriveInput) -> TokenStream {
    // parse the input token stream as a DeriveInput
    let input = syn::parse_macro_input!(input as syn::DeriveInput);
    // println!("{:#?}", input);
    // get the ident
    let ident = input.ident;
    let generics = input.generics;
    // get enum variants
    let variants = match input.data {
        syn::Data::Enum(data) => data.variants,
        _ => panic!("EnumFrom only works on enums"),
    };
    // for each variant, get the ident and fields
    let from_impls = variants.iter().map(|variant| {
        let var = &variant.ident;
        match &variant.fields {
            syn::Fields::Unnamed(fields) => {
                // only support one field
                if fields.unnamed.len() != 1 {
                    quote! {}
                } else {
                    let field = fields.unnamed.first().expect("only surport 1 field");
                    let ty = &field.ty;
                    // impl From<DirectionUp> for Direction {
                    //     fn from(value: DirectionUp) -> Self {
                    //         Direction::UP(value)
                    //     }
                    // }
                    quote! {
                        impl #generics From<#ty> for #ident #generics {
                            fn from(v: #ty) -> Self {
                                #ident::#var(v)
                            }
                        }
                    }
                }
            }

            syn::Fields::Unit => quote! {},
            syn::Fields::Named(_fields) => quote! {},
        }
    });

    // quote return proce-macro2 TokenTtream so we need to convert it to TokenStream
    quote! {
        #(#from_impls)*
    }
}
