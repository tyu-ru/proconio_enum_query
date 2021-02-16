#[proconio_enum_query::proconio_enum_query]
#[derive(PartialEq, Debug)]
enum Query {
    A(i64),
    B(char),
    C(i64, char),
    D { x: i64, y: char },
}

#[test]
fn test() {
    let source = proconio::source::auto::AutoSource::from(
        r#"1 12
           2 P
           3 34 Q
           4 56 R"#,
    );

    proconio::input! {
        from source,
        q1: Query,
        q2: Query,
        q3: Query,
        q4: Query,
    }

    assert_eq!(q1, Query::A(12));
    assert_eq!(q2, Query::B('P'));
    assert_eq!(q3, Query::C(34, 'Q'));
    assert_eq!(q4, Query::D { x: 56, y: 'R' });
}
