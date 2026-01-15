#![allow(clippy::upper_case_acronyms)]

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
    impl State<TesterState> for A {}

    struct AA;
    #[superstate(A)]
    impl State<TesterState> for AA {
        fn enter(_superstates: &mut Self::Superstates<'_>) -> StateEntry<TesterState, Self> {
            Self.into()
        }

        fn init(
            &mut self,
            _superstates: &mut Self::Superstates<'_>,
        ) -> impl Into<Next<TesterState>> {
            TesterState::A
        }

        fn update(
            &mut self,
            _superstates: &mut Self::Superstates<'_>,
        ) -> impl Into<Next<TesterState>> {
            TesterState::A
        }

        fn top_down_update(
            &mut self,
            _superstates: &mut Self::Superstates<'_>,
        ) -> impl Into<Next<TesterState>> {
            TesterState::A
        }

        fn handle_event(
            &mut self,
            _event: &(),
            _superstates: &mut Self::Superstates<'_>,
        ) -> impl Into<EventResponse<TesterState>> {
            TesterState::A
        }

        fn exit(self, _superstates: &mut Self::Superstates<'_>) -> impl Into<Next<TesterState>> {
            TesterState::Top
        }
    }

    struct B;
    #[superstate(Top)]
    impl State<TesterState> for B {
        fn update(
            &mut self,
            _superstates: &mut Self::Superstates<'_>,
        ) -> impl Into<Next<TesterState>> {
            Next::ExactTarget(TesterState::Top)
        }

        fn top_down_update(
            &mut self,
            _superstates: &mut Self::Superstates<'_>,
        ) -> impl Into<Next<TesterState>> {
            Next::ExactTarget(TesterState::Top)
        }

        fn handle_event(
            &mut self,
            _event: &(),
            _superstates: &mut Self::Superstates<'_>,
        ) -> impl Into<EventResponse<TesterState>> {
            Next::ExactTarget(TesterState::Top)
        }

        fn exit(self, _superstates: &mut Self::Superstates<'_>) -> impl Into<Next<TesterState>> {
            Next::ExactTarget(TesterState::Top)
        }
    }

    struct BA;
    #[superstate(B)]
    impl State<TesterState> for BA {
        fn enter(_superstates: &mut Self::Superstates<'_>) -> StateEntry<TesterState, Self> {
            StateEntry::ExactTarget(TesterState::Top)
        }
    }

    struct BB;
    #[superstate(B)]
    impl State<TesterState> for BB {
        fn init(
            &mut self,
            _superstates: &mut Self::Superstates<'_>,
        ) -> impl Into<Next<TesterState>> {
            Next::ExactTarget(TesterState::Top)
        }
    }
}

#[test]
fn state_machine() {
    let mut machine = TesterMachineBuilder::new(Top {}).build();

    machine.transition(TesterState::A);
    assert!(matches!(machine.state(), TesterState::A));

    machine.transition(TesterState::Top);
    assert!(matches!(machine.state(), TesterState::A));

    machine.exact_transition(TesterState::Top);
    assert!(matches!(machine.state(), TesterState::Top));
}

#[test]
fn normal_transition() {
    let mut machine = TesterMachineBuilder::new(Top {}).build();

    machine.transition(TesterState::AA);
    assert!(matches!(machine.state(), TesterState::AA));

    machine.update();
    assert!(matches!(machine.state(), TesterState::AA));

    machine.top_down_update();
    assert!(matches!(machine.state(), TesterState::AA));

    machine.handle_event(&());
    assert!(matches!(machine.state(), TesterState::AA));

    machine.exact_transition(TesterState::A);
    assert!(matches!(machine.state(), TesterState::A));
}

#[test]
fn exact_transition() {
    let mut machine = TesterMachineBuilder::new(Top {}).build();

    machine.transition(TesterState::B);
    assert!(matches!(machine.state(), TesterState::B));
    machine.update();
    assert!(matches!(machine.state(), TesterState::Top));

    machine.transition(TesterState::B);
    assert!(matches!(machine.state(), TesterState::B));
    machine.top_down_update();
    assert!(matches!(machine.state(), TesterState::Top));

    machine.transition(TesterState::B);
    assert!(matches!(machine.state(), TesterState::B));
    machine.handle_event(&());
    assert!(matches!(machine.state(), TesterState::Top));

    machine.transition(TesterState::B);
    assert!(matches!(machine.state(), TesterState::B));
    machine.exact_transition(TesterState::A);
    assert!(matches!(machine.state(), TesterState::Top));

    machine.transition(TesterState::BA);
    assert!(matches!(machine.state(), TesterState::Top));

    machine.transition(TesterState::BB);
    assert!(matches!(machine.state(), TesterState::Top));
}
