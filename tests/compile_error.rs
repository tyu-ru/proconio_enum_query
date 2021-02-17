#[test]
fn tests() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/compile_error/not_enum.rs");
    t.compile_fail("tests/compile_error/generic_enum.rs");
    t.compile_fail("tests/compile_error/incollect_arg.rs");
}
