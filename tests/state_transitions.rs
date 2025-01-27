use moku::*;
use state_trans::{
    state_machine::{StateTransMachineBuilder, StateTransState, STATE_TRANS_STATE_CHART},
    Top,
};
use test_log::test;

#[state_machine]
mod state_trans {
    use moku::*;

    #[machine_module]
    pub mod state_machine {}

    use state_machine::StateTransState;

    pub struct Top;

    impl TopState<StateTransState> for Top {}

    struct A;

    #[superstate(Top)]
    impl State<StateTransState> for A {
        fn init(&mut self, _superstates: &mut Self::Superstates<'_>) -> Option<StateTransState> {
            Some(StateTransState::B)
        }
    }

    struct B;

    #[superstate(Top)]
    impl State<StateTransState> for B {
        fn update(&mut self, _superstates: &mut Self::Superstates<'_>) -> Option<StateTransState> {
            Some(StateTransState::C)
        }
    }

    struct C;

    #[superstate(Top)]
    impl State<StateTransState> for C {
        fn top_down_update(
            &mut self,
            _superstates: &mut Self::Superstates<'_>,
        ) -> Option<StateTransState> {
            Some(StateTransState::D)
        }
    }

    struct D;

    #[superstate(Top)]
    impl State<StateTransState> for D {
        fn exit(self, _superstates: &mut Self::Superstates<'_>) -> Option<StateTransState> {
            Some(StateTransState::E)
        }
    }

    struct E;

    #[superstate(Top)]
    impl State<StateTransState> for E {}

    struct F;

    #[superstate(Top)]
    impl State<StateTransState> for F {
        fn enter(_superstates: &mut Self::Superstates<'_>) -> StateEntry<Self, StateTransState> {
            StateEntry::Transition(StateTransState::Top)
        }
    }
}

#[test]
fn state_chart() {
    assert_eq!(
        STATE_TRANS_STATE_CHART,
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
    let mut machine = StateTransMachineBuilder::new(Top).build();
    machine.transition(StateTransState::A);
    assert!(matches!(machine.state(), StateTransState::B));
}

#[test]
fn update() {
    let mut machine = StateTransMachineBuilder::new(Top).build();
    machine.transition(StateTransState::B);
    machine.update();
    assert!(matches!(machine.state(), StateTransState::C));
}

#[test]
fn top_down_update() {
    let mut machine = StateTransMachineBuilder::new(Top).build();
    machine.transition(StateTransState::C);
    machine.top_down_update();
    assert!(matches!(machine.state(), StateTransState::D));
}

#[test]
fn exit() {
    let mut machine = StateTransMachineBuilder::new(Top).build();
    machine.transition(StateTransState::D);
    machine.transition(StateTransState::Top);
    assert!(matches!(machine.state(), StateTransState::E));
}

#[test]
fn enter() {
    let mut machine = StateTransMachineBuilder::new(Top).build();
    machine.transition(StateTransState::F);
    assert!(matches!(machine.state(), StateTransState::Top));
}
