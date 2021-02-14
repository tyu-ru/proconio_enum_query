use proc_macro::TokenStream;

#[proc_macro_derive(derive_query)]
pub fn derive(_input: TokenStream) -> TokenStream {
    TokenStream::new()
}
