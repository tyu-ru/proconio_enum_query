use proconio::{input, source::auto::AutoSource};
#[derive(proconio_enum_query::derive_query)]
enum Query {
    A(i64),
}

#[test]
#[should_panic(expected = "unknown query type '2'")]
fn test_unknown_query_type() {
    let source = AutoSource::from("2 23");
    input! {
        from source,
        _q: Query,
    }
}

#[test]
#[should_panic(expected = "query number parse error: ParseIntError { kind: InvalidDigit }")]
fn test_unknown_query_type2() {
    let source = AutoSource::from("hoge 23");
    input! {
        from source,
        _q: Query,
    }
}
