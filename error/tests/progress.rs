#[test]
fn tests() {
    let t = trybuild::TestCases::new();
    t.pass("tests/01-simply-unwrap.rs");
    t.pass("tests/02-match-result.rs");
    t.pass("test/03-custom-error.rs")
}