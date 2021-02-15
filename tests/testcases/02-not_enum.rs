#[proconio_enum_query::derive_query]
struct Query1 {
    a: i64,
    b: char,
}
#[proconio_enum_query::derive_query]
struct Query2(i64, char);

fn main() {}
