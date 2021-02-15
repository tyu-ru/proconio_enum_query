use proc_macro::TokenStream;
use proc_macro2::Literal;
use quote::quote;
use syn::{
    parse_macro_input, punctuated::Punctuated, token::Comma, Data, DeriveInput, Fields,
    FieldsUnnamed, Variant,
};

macro_rules! unwrap_or_compile_error {
    ($option:expr, $tokens:expr,$msg:expr) => {
        if let Some(data) = $option {
            data
        } else {
            return syn::Error::new_spanned($tokens, $msg)
                .to_compile_error()
                .into();
        }
    };
}

#[proc_macro_derive(derive_query)]
pub fn derive(input: TokenStream) -> TokenStream {
    let item = parse_macro_input!(input as DeriveInput);

    // eprintln!("{:#?}", item);

    let enum_name = &item.ident;
    let variants = unwrap_or_compile_error!(
        extract_enum_variants(&item),
        item,
        "derive_query can only derive from enum."
    );

    let matcher = variants.iter().enumerate().map(|(i, v)| {
        let i = Literal::usize_unsuffixed(i);
        let ident = &v.ident;
        let fields =
            &unwrap_or_compile_error!(extract_fields_unnamed(&v.fields), &v.fields, "fxxx").unnamed;
        let len = fields.len();
        let fields = if len == 1 {
            quote! { #fields }
        } else {
            quote! { (#fields) }
        };
        let fields2 = if len == 1 {
            quote! { x }
        } else {
            let i = (0..len).map(|i| Literal::usize_unsuffixed(i));
            quote! { #(x.#i),* }
        };
        quote! { #i => { proconio::input!{ from source, x: #fields } #enum_name::#ident(#fields2) }}
    });

    let gen = quote! {
        impl proconio::source::Readable for #enum_name {
            type Output = Self;
            fn read<R: std::io::BufRead, S: proconio::source::Source<R>>(source: &mut S) -> Self::Output {
                let t = source.next_token_unwrap().parse::<usize>().expect("query number parse error");
                match t - 1 {
                    #(#matcher)*
                    _ => panic!("unknown query type '{}'", t),
                }
            }
        }
    };
    gen.into()
}

fn extract_enum_variants(item: &DeriveInput) -> Option<&Punctuated<Variant, Comma>> {
    if let Data::Enum(ref data_enum) = item.data {
        Some(&data_enum.variants)
    } else {
        None
    }
}

fn extract_fields_unnamed(fields: &Fields) -> Option<&FieldsUnnamed> {
    if let Fields::Unnamed(ref unnamed) = fields {
        Some(unnamed)
    } else {
        None
    }
}
