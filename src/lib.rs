use proc_macro::TokenStream;
use proc_macro2::Literal;
use quote::quote;
#[allow(unused_imports)]
use syn::{
    parse_macro_input, punctuated::Punctuated, token::Comma, Data, DeriveInput, Fields,
    FieldsUnnamed, Ident, Item, Type, Variant,
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

#[proc_macro_attribute]
pub fn derive_query(_args: TokenStream, input: TokenStream) -> TokenStream {
    let item = if let Item::Enum(x) = parse_macro_input!(input as Item) {
        x
    } else {
        return syn::Error::new(
            proc_macro2::Span::call_site(),
            "expected enum or match expression",
        )
        .to_compile_error()
        .into();
    };

    // eprintln!("{:#?}", item);

    let mut out = item.clone();
    for v in &mut out.variants {
        if let Fields::Unnamed(FieldsUnnamed { unnamed, .. }) = &mut v.fields {
            for field in unnamed {
                let ty = &field.ty;
                let ident = quote! { <#ty as proconio::source::Readable>::Output };
                field.ty = Type::Verbatim(ident.into());
            }
        }
    }

    let enum_name = &item.ident;

    let matcher = item.variants.iter().enumerate().map(|(i, v)| {
        let i = Literal::usize_unsuffixed(i);
        let ident = &v.ident;
        let fields =
            &unwrap_or_compile_error!(extract_fields_unnamed(&v.fields), &v.fields, "???").unnamed;
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
        #out

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

fn extract_fields_unnamed(fields: &Fields) -> Option<&FieldsUnnamed> {
    if let Fields::Unnamed(ref unnamed) = fields {
        Some(unnamed)
    } else {
        None
    }
}
