use wattbuild::{Dependency, Source};

fn main() {
    wattbuild::build(
        &[Dependency {
            package: "proconio_enum_query_impl",
            source: Source::Path {
                path: "./impl",
                or: Some(Box::new(Source::Git {
                    git: "https://github.com/tyu-ru/proconio_enum_query",
                    rev: None,
                })),
            },
        }],
        None,
        None,
        Some("python3.9".as_ref()),
    );
}
