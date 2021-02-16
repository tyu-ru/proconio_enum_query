use proc_macro::TokenStream;
use watt::WasmMacro;

static MACRO: WasmMacro = WasmMacro::new(include_bytes!(concat!(
    env!("OUT_DIR"),
    "/proconio_enum_query_impl.wasm",
)));

#[proc_macro_attribute]
pub fn proconio_enum_query(args: TokenStream, item: TokenStream) -> TokenStream {
    MACRO.proc_macro_attribute("proconio_enum_query", args, item)
}
