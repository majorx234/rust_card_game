use proc_macro::{Span, TokenStream};
use quote::{format_ident, quote};
use syn::{parse_macro_input, Data, DeriveInput, Error, Fields};

macro_rules! derive_error {
    ($string: tt) => {
        Error::new(Span::call_site().into(), $string)
            .to_compile_error()
            .into()
    };
}

#[proc_macro_derive(EnumToStr)]
pub fn derive_to_string(input: TokenStream) -> TokenStream {
    let input: DeriveInput = parse_macro_input!(input as DeriveInput);
    // get enum name
    let ref enum_name = input.ident;
    // get enum elements
    let ref data = input.data;

    match data {
        Data::Enum(data_enum) => {
            let mut enum_items = Vec::new();
            for variant in &data_enum.variants {
                // Variant's name
                let ref variant_name = variant.ident;
                enum_items.push(quote! {
                    #enum_name::#variant_name => stringify!(#variant_name).to_string()

                });
            }
            let gen = quote! {
               impl #enum_name {
                    fn to_string(&self) -> String {
                        match *self {
                            #(#enum_items),*
                        }
                    }
                },
            };
            return TokenStream::from(gen);
        }
        _ => return derive_error!("EnumToStr is only useable with enums"),
    }
}
