struct Twice;
impl proconio::source::Readable for Twice {
    type Output = i64;

    fn read<R: std::io::BufRead, S: proconio::source::Source<R>>(source: &mut S) -> Self::Output {
        proconio::input! { from source, x: i64 }
        x * 2
    }
}

#[allow(dead_code)]
#[proconio_enum_query::derive_query]
#[derive(PartialEq, Debug)]
enum Query {
    A(proconio::marker::Usize1),
    B(proconio::marker::Isize1, proconio::marker::Chars),
    C(Twice),
}

fn main() {
    let source = proconio::source::auto::AutoSource::from(
        r#"1 12
           2 34 ABC
           3 56"#,
    );

    proconio::input! {
        from source,
        q1: Query,
        q2: Query,
        q3: Query,
    }

    assert_eq!(q1, Query::A(12usize - 1));
    assert_eq!(q2, Query::B(34isize - 1, vec!['A', 'B', 'C']));
    assert_eq!(q3, Query::C(56i64 * 2));
}
