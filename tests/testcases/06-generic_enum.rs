#[proconio_enum_query::derive_query]
#[allow(dead_code)]
#[derive(PartialEq, Debug)]
enum Query<T> {
    A(T),
    B,
}

fn main() {}
