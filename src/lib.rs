use proc_macro2::{Literal, TokenStream};
use quote::quote;
use syn::{
    punctuated::Punctuated, token::Comma, Fields, FieldsNamed, FieldsUnnamed, Ident, ItemEnum, Lit,
    MetaNameValue, Type, Variant,
};

/// Derive [proconio::source::Readable](https://docs.rs/proconio/0.4.1/proconio/source/trait.Readable.html).
///
/// Allow queries that can be expressed as enum types to be handled directly in [proconio::input!](https://docs.rs/proconio/0.4.1/proconio/macro.input.html).
///
/// # Example
/// ```no_run
/// # use proconio::{input, marker::Usize1};
/// //#[macro_use]
/// //extern crate proconio_enum_query as _;
///
/// #[proconio_enum_query::proconio_enum_query]
/// #[derive(PartialEq, Debug)]
/// enum Query {
///     A(i64),
///     B,
///     C(char, Usize1),
/// }
///
/// // stdin:
/// // 1 12
/// // 2
/// // 3 X 34
/// input! {
///     query: [Query; 3]
/// }
///
/// assert_eq!(query[0], Query::A(12 - 1));
/// assert_eq!(query[1], Query::B);
/// assert_eq!(query[2], Query::C('X', 34));
/// ```

#[proc_macro_attribute]
pub fn proconio_enum_query(
    args: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    run(args.into(), input.into())
        .map(|res| res.into())
        .unwrap_or_else(|e| e.to_compile_error().into())
}

fn run(args: TokenStream, input: TokenStream) -> syn::Result<TokenStream> {
    let item = syn::parse2::<ItemEnum>(input)?;

    let mut output_enum = item.clone();
    for v in &mut output_enum.variants {
        type_convert_to_readable_output(&mut v.fields);
    }

    let enum_ident = &item.ident;

    if !item.generics.params.is_empty() {
        return Err(syn::Error::new_spanned(
            item.generics,
            "generic enum not supported.",
        ));
    }

    let start_index = start_index(args)?;

    let arms = arms(enum_ident, &item.variants, start_index);
    let gen = quote! {
        #output_enum

        impl proconio::source::Readable for #enum_ident {
            type Output = Self;
            fn read<R: std::io::BufRead, S: proconio::source::Source<R>>(source: &mut S) -> Self::Output {
                let t = source.next_token_unwrap().parse::<isize>().expect("query number parse error");
                match t {
                    #arms
                    _ => panic!("unknown query type '{}'", t),
                }
            }
        }
    };
    Ok(gen.into())
}

fn type_convert_to_readable_output(fields: &mut Fields) {
    match fields {
        Fields::Unnamed(FieldsUnnamed {
            unnamed: ref mut fields,
            ..
        })
        | Fields::Named(FieldsNamed {
            named: ref mut fields,
            ..
        }) => {
            for field in fields {
                let ty = &field.ty;
                let ident = quote! { <#ty as proconio::source::Readable>::Output };
                field.ty = Type::Verbatim(ident.into());
            }
        }
        Fields::Unit => {}
    }
}

fn start_index(args: TokenStream) -> syn::Result<isize> {
    if args.is_empty() {
        return Ok(1);
    }

    let meta_name_value = syn::parse2::<MetaNameValue>(args)?;
    if !meta_name_value.path.is_ident("start_index") {
        return Err(syn::Error::new_spanned(
            meta_name_value.path,
            "unknown args",
        ));
    }
    if let Lit::Int(ref lit_int) = meta_name_value.lit {
        lit_int.base10_parse()
    } else {
        Err(syn::Error::new_spanned(
            meta_name_value.lit,
            concat!("must be isize literal"),
        ))
    }
}

fn arms(
    enum_ident: &Ident,
    variants: &Punctuated<Variant, Comma>,
    start_index: isize,
) -> TokenStream {
    let arms = variants.iter().scan(start_index, |i, v| {
        let variant_ident = &v.ident;
        let stmt = match &v.fields {
            Fields::Unnamed(unnamed) => tuple_field_input(enum_ident, variant_ident, unnamed),
            Fields::Named(named) => strct_field_input(enum_ident, variant_ident, named),
            Fields::Unit => quote! { #enum_ident::#variant_ident },
        };
        let lit_i = Literal::isize_suffixed(*i);
        *i += 1;
        Some(quote! { #lit_i => { #stmt }})
    });
    quote! { #(#arms)* }
}

fn tuple_field_input(
    enum_ident: &Ident,
    variant_ident: &Ident,
    field: &FieldsUnnamed,
) -> TokenStream {
    let fields = &field.unnamed;
    let len = fields.len();
    let input_fields = if len == 1 {
        quote! { (#fields,) }
    } else {
        quote! { (#fields) }
    };
    let i = (0..len).map(|i| Literal::usize_unsuffixed(i));
    let output_fields = quote! { #(temp.#i),* };
    quote! { proconio::input!{ from source, temp: #input_fields } #enum_ident::#variant_ident(#output_fields) }
}

fn strct_field_input(
    enum_ident: &Ident,
    variant_ident: &Ident,
    field: &FieldsNamed,
) -> TokenStream {
    let fields = &field.named;
    let (temp_input, temp_output): (Vec<_>, Vec<_>) = fields
        .iter()
        .map(|f| {
            let (ident, ty) = (f.ident.as_ref().expect("parse error?"), &f.ty);
            (quote! { #ident:#ty }, quote! { #ident })
        })
        .unzip();
    quote! { proconio::input! { from source, #(#temp_input),*} #enum_ident::#variant_ident{ #(#temp_output),* } }
}
