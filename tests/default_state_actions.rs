use dft::{
    machine::{DftMachineBuilder, DftState},
    Top,
};
use moku::*;
use test_log::test;

#[state_machine]
mod dft {
    use moku::*;

    #[machine_module]
    pub mod machine {}

    use machine::DftState;

    pub struct Top;

    impl TopState<DftState> for Top {}

    struct Foo;

    #[superstate(Top)]
    impl State<DftState> for Foo {}
}

#[test]
fn default_actions() {
    let mut machine = DftMachineBuilder::new(Top).build();
    assert!(matches!(machine.state(), DftState::Top));

    machine.update();
    assert!(matches!(machine.state(), DftState::Top));

    machine.top_down_update();
    assert!(matches!(machine.state(), DftState::Top));

    machine.transition(DftState::Foo);
    assert!(matches!(machine.state(), DftState::Foo));

    machine.update();
    assert!(matches!(machine.state(), DftState::Foo));

    machine.top_down_update();
    assert!(matches!(machine.state(), DftState::Foo));
}
