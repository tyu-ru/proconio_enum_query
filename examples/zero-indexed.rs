use proconio::{input, marker::*, source::auto::AutoSource};

#[macro_use]
extern crate proconio_enum_query as _;

#[proconio_enum_query(start_index = 0)]
#[derive(PartialEq, Debug)]
enum Query {
    A(Usize1),
    B,
    C { x: i64, y: char },
}

fn main() {
    let source = AutoSource::from(
        r#"1 12
           2
           3 34 X"#,
    );

    input! {
        from source,
        query: [Query; 3],
    }

    assert_eq!(query[0], Query::A(12 - 1));
    assert_eq!(query[1], Query::B);
    assert_eq!(query[2], Query::C { x: 34, y: 'X' });
}
