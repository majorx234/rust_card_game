use proc_macro::TokenStream;

#[proc_macro_derive(EnumToStr)]
pub fn derive_to_string(input: TokenStream) -> TokenStream {
    TokenStream::new()
}
