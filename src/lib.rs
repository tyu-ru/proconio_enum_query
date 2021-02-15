use proc_macro::TokenStream;
use proc_macro2::Literal;
use quote::quote;
use syn::{parse_macro_input, Fields, Item, Type};

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
        match &mut v.fields {
            Fields::Unnamed(unnamed) => {
                for field in &mut unnamed.unnamed {
                    let ty = &field.ty;
                    let ident = quote! { <#ty as proconio::source::Readable>::Output };
                    field.ty = Type::Verbatim(ident.into());
                }
            }
            Fields::Named(named) => {
                for field in &mut named.named {
                    let ty = &field.ty;
                    let ident = quote! { <#ty as proconio::source::Readable>::Output };
                    field.ty = Type::Verbatim(ident.into());
                }
            }
            Fields::Unit => {}
        };
    }

    let enum_name = &item.ident;

    let matcher = item.variants.iter().enumerate().map(|(i, v)| {
        let ident = &v.ident;
        let stmt = match &v.fields {
            Fields::Unnamed(unnamed) => {
                let fields = &unnamed.unnamed;
                let len = fields.len();
                let input_fields = if len == 1 {
                    quote! { (#fields,) }
                } else {
                    quote! { (#fields) }
                };
                let i = (0..len).map(|i| Literal::usize_unsuffixed(i));
                let output_fields= quote! { #(temp.#i),* };
                quote! { proconio::input!{ from source, temp: #input_fields } #enum_name::#ident(#output_fields) }
            }
            Fields::Named(named) => {
                let fields = &named.named;
                let (temp_input, temp_output):(Vec<_>,Vec<_>) = fields.iter().map(|f|{
                    let (ident, ty) = (f.ident.as_ref().expect("parse error?"), &f.ty);
                    (quote!{ #ident:#ty }, quote!{ #ident })
                }).unzip();
                quote! { proconio::input! { from source, #(#temp_input),*} #enum_name::#ident{ #(#temp_output),* } }
            }
            Fields::Unit => {
                quote! { #enum_name::#ident }
            }
        };
        let i = Literal::usize_unsuffixed(i);
        quote! { #i => { #stmt }}
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
