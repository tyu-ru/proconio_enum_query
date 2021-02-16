use proc_macro2::{Literal, TokenStream};
use quote::quote;
use syn::{Fields, FieldsNamed, FieldsUnnamed, Ident, ItemEnum, Type};

#[no_mangle]
pub extern "C" fn proconio_enum_query(args: TokenStream, input: TokenStream) -> TokenStream {
    run(args, input).unwrap_or_else(|e| e.to_compile_error())
}

fn run(_args: TokenStream, input: TokenStream) -> syn::Result<TokenStream> {
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

    let matcher = item.variants.iter().enumerate().map(|(i, v)| {
        let variant_ident = &v.ident;
        let stmt = match &v.fields {
            Fields::Unnamed(unnamed) => tuple_field_input(enum_ident, variant_ident, unnamed),
            Fields::Named(named) => strct_field_input(enum_ident, variant_ident, named),
            Fields::Unit => quote! { #enum_ident::#variant_ident },
        };
        let i = Literal::usize_unsuffixed(i);
        quote! { #i => { #stmt }}
    });

    let gen = quote! {
        #output_enum

        impl proconio::source::Readable for #enum_ident {
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
