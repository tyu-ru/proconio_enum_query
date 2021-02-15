#[test]
fn tests() {
    let t = trybuild::TestCases::new();
    t.pass("tests/testcases/01-can_build.rs");
    t.compile_fail("tests/testcases/02-not_enum.rs");
    t.pass("tests/testcases/03-only_primitive_variants.rs");
    t.pass("tests/testcases/04-translate_output_type.rs");
    t.pass("tests/testcases/05-unit_variant.rs");
    t.compile_fail("tests/testcases/06-generic_enum.rs");
}
