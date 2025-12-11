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

    struct A;

    #[superstate(Top)]
    impl State<TesterState> for A {
        fn init(
            &mut self,
            _superstates: &mut Self::Superstates<'_>,
        ) -> impl Into<Next<TesterState>> {
            Some(TesterState::B)
        }
    }

    struct B;

    #[superstate(Top)]
    impl State<TesterState> for B {
        fn update(
            &mut self,
            _superstates: &mut Self::Superstates<'_>,
        ) -> impl Into<Next<TesterState>> {
            Some(TesterState::C)
        }
    }

    struct C;

    #[superstate(Top)]
    impl State<TesterState> for C {
        fn top_down_update(
            &mut self,
            _superstates: &mut Self::Superstates<'_>,
        ) -> impl Into<Next<TesterState>> {
            Some(TesterState::D)
        }
    }

    struct D;

    #[superstate(Top)]
    impl State<TesterState> for D {
        fn exit(self, _superstates: &mut Self::Superstates<'_>) -> impl Into<Next<TesterState>> {
            Some(TesterState::E)
        }
    }

    struct E;

    #[superstate(Top)]
    impl State<TesterState> for E {}

    struct F;

    #[superstate(Top)]
    impl State<TesterState> for F {
        fn enter(_superstates: &mut Self::Superstates<'_>) -> StateEntry<TesterState, Self> {
            StateEntry::Target(TesterState::Top)
        }
    }
}

#[test]
fn state_chart() {
    assert_eq!(
        TESTER_STATE_CHART,
        "Top
├─ A
├─ B
├─ C
├─ D
├─ E
└─ F"
    );
}

#[test]
fn init() {
    let mut machine = TesterMachineBuilder::new(Top).build();
    machine.transition(TesterState::A);
    assert!(matches!(machine.state(), TesterState::B));
}

#[test]
fn update() {
    let mut machine = TesterMachineBuilder::new(Top).build();
    machine.transition(TesterState::B);
    machine.update();
    assert!(matches!(machine.state(), TesterState::C));
}

#[test]
fn top_down_update() {
    let mut machine = TesterMachineBuilder::new(Top).build();
    machine.transition(TesterState::C);
    machine.top_down_update();
    assert!(matches!(machine.state(), TesterState::D));
}

#[test]
fn exit() {
    let mut machine = TesterMachineBuilder::new(Top).build();
    machine.transition(TesterState::D);
    machine.transition(TesterState::A);
    assert!(matches!(machine.state(), TesterState::E));
}

#[test]
fn enter() {
    let mut machine = TesterMachineBuilder::new(Top).build();
    machine.transition(TesterState::F);
    assert!(matches!(machine.state(), TesterState::Top));
}
