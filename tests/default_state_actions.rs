use moku::*;
use test_log::test;
use tester::{machine::*, *};

#[state_machine]
mod tester {
    use moku::*;

    #[machine_module]
    pub mod machine {}

    pub struct Top;

    impl TopState for Top {}

    struct Foo;

    impl Substate<Top> for Foo {}
}

#[test]
fn default_actions() {
    let mut machine = Builder::new(Top).build();
    assert!(matches!(machine.state(), State::Top));

    machine.update();
    assert!(matches!(machine.state(), State::Top));

    machine.top_down_update();
    assert!(matches!(machine.state(), State::Top));

    machine.transition(State::Foo);
    assert!(matches!(machine.state(), State::Foo));

    machine.update();
    assert!(matches!(machine.state(), State::Foo));

    machine.top_down_update();
    assert!(matches!(machine.state(), State::Foo));
}
