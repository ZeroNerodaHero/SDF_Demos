#[test]
fn ui_tests() {
    let t = trybuild::TestCases::new();

    t.pass("ui-tests/pass/pass_*.rs");
    t.compile_fail("ui-tests/fail/fail_*.rs");
}
