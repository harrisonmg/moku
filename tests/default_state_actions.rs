use moku::*;
use test_log::test;
use tester::{machine::*, *};

#[state_machine]
mod tester {
    use moku::*;

    #[machine_module]
    pub mod machine {}

    use machine::TesterState;

    pub struct Top;

    impl TopState<TesterState> for Top {}

    struct Foo;

    #[superstate(Top)]
    impl State<TesterState> for Foo {}
}

#[test]
fn default_actions() {
    let mut machine = TesterMachineBuilder::new(Top).build();
    assert!(matches!(machine.state(), TesterState::Top));

    machine.update();
    assert!(matches!(machine.state(), TesterState::Top));

    machine.top_down_update();
    assert!(matches!(machine.state(), TesterState::Top));

    machine.transition(TesterState::Foo);
    assert!(matches!(machine.state(), TesterState::Foo));

    machine.update();
    assert!(matches!(machine.state(), TesterState::Foo));

    machine.top_down_update();
    assert!(matches!(machine.state(), TesterState::Foo));
}
