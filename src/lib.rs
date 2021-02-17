use proc_macro::TokenStream;
use watt::WasmMacro;

static MACRO: WasmMacro = WasmMacro::new(include_bytes!(concat!(
    env!("OUT_DIR"),
    "/proconio_enum_query_impl.wasm",
)));

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
pub fn proconio_enum_query(args: TokenStream, item: TokenStream) -> TokenStream {
    MACRO.proc_macro_attribute("proconio_enum_query", args, item)
}
