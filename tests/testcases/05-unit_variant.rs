#[proconio_enum_query::proconio_enum_query]
#[derive(PartialEq, Debug)]
enum Query {
    A(i64),
    B,
    C(i64, char),
}

fn main() {
    let source = proconio::source::auto::AutoSource::from(
        r#"1 12
           2
           3 34 Q"#,
    );

    proconio::input! {
        from source,
        q1: Query,
        q2: Query,
        q3: Query,
    }

    assert_eq!(q1, Query::A(12));
    assert_eq!(q2, Query::B);
    assert_eq!(q3, Query::C(34, 'Q'));
}
