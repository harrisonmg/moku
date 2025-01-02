#[test]
fn compile_fail() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/machine_module/*.rs");
    t.compile_fail("tests/superstate/*.rs");
    t.compile_fail("tests/state_machine/*.rs");
}
