use proconio_enum_query::derive_query;

#[derive(derive_query)]
enum Query {
    A(i64),
    B(char),
}

fn main() {}
