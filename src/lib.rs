use proc_macro::TokenStream;

use syn::{parse_macro_input, Data, DeriveInput};

#[proc_macro_derive(derive_query)]
pub fn derive(input: TokenStream) -> TokenStream {
    let item = parse_macro_input!(input as DeriveInput);

    if let Data::Enum(_) = &item.data {
    } else {
        return syn::Error::new_spanned(item, "derive_query can only derive from enum.")
            .to_compile_error()
            .into();
    };

    TokenStream::new()
}
