#[test]
fn compile_fail() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/compile_fail/machine_module/*.rs");
    t.compile_fail("tests/compile_fail/superstate/*.rs");
    t.compile_fail("tests/compile_fail/state_machine/*.rs");
}
