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

    impl TopState<TesterState> for Top {
        fn init(&mut self) -> impl Into<Next<TesterState>> {
            Some(TesterState::A)
        }

        fn update(&mut self) -> impl Into<Next<TesterState>> {
            Some(TesterState::B)
        }

        fn top_down_update(&mut self) -> impl Into<Next<TesterState>> {
            Some(TesterState::C)
        }
    }

    struct A;

    #[superstate(Top)]
    impl State<TesterState> for A {}

    struct B;

    #[superstate(Top)]
    impl State<TesterState> for B {}

    struct C;

    #[superstate(Top)]
    impl State<TesterState> for C {}
}

#[test]
fn state_chart() {
    assert_eq!(
        TESTER_STATE_CHART,
        "Top
├─ A
├─ B
└─ C"
    );
}

#[test]
fn init() {
    let machine = TesterMachineBuilder::new(Top).build();
    assert!(matches!(machine.state(), TesterState::A));
}

#[test]
fn update() {
    let mut machine = TesterMachineBuilder::new(Top).build();
    machine.update();
    assert!(matches!(machine.state(), TesterState::B));
}

#[test]
fn top_down_update() {
    let mut machine = TesterMachineBuilder::new(Top).build();
    machine.top_down_update();
    assert!(matches!(machine.state(), TesterState::C));
}
