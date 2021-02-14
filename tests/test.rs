#[test]
fn tests() {
    let t = trybuild::TestCases::new();
    t.pass("tests/testcases/01-can_build.rs");
}
