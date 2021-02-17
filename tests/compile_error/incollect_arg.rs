#[proconio_enum_query::proconio_enum_query(hoge)]
enum Query1 {
    A,
}

#[proconio_enum_query::proconio_enum_query(fuga(piyo))]
enum Query2 {
    A,
}

#[proconio_enum_query::proconio_enum_query(foo = 10)]
enum Query3 {
    A,
}

#[proconio_enum_query::proconio_enum_query(start_index = abc)]
enum Query3 {
    A,
}

#[proconio_enum_query::proconio_enum_query(start_index = "abc")]
enum Query3 {
    A,
}

fn main() {}
