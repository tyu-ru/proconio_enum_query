#[proconio_enum_query::proconio_enum_query(start_index=0)]
#[derive(PartialEq, Debug)]
enum Query {
    A(i64),
    B,
    C(i64, char),
}

#[test]
fn test() {
    let source = proconio::source::auto::AutoSource::from(
        r#"0 12
           1
           2 34 Q"#,
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
