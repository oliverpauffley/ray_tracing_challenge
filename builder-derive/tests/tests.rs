#[test]
fn builder() {
    let t = trybuild::TestCases::new();
    t.pass("tests/builder/happy_path.rs");
    t.compile_fail("tests/builder/unrecognized-attribute.rs");
}
