use moku::*;
use top_trans::{
    state_machine::{TopTransMachineBuilder, TopTransState, TOP_TRANS_STATE_CHART},
    Top,
};

#[state_machine]
mod top_trans {
    use moku::*;

    #[machine_module]
    pub mod state_machine {}

    use state_machine::TopTransState;

    pub struct Top;

    impl TopState<TopTransState> for Top {
        fn init(&mut self) -> Option<TopTransState> {
            Some(TopTransState::A)
        }

        fn update(&mut self) -> Option<TopTransState> {
            Some(TopTransState::B)
        }

        fn top_down_update(&mut self) -> Option<TopTransState> {
            Some(TopTransState::C)
        }
    }

    struct A;

    #[superstate(Top)]
    impl State<TopTransState> for A {}

    struct B;

    #[superstate(Top)]
    impl State<TopTransState> for B {}

    struct C;

    #[superstate(Top)]
    impl State<TopTransState> for C {}
}

#[test]
fn state_chart() {
    assert_eq!(
        TOP_TRANS_STATE_CHART,
        "Top
├─ A
├─ B
└─ C"
    );
}

#[test]
fn init() {
    let machine = TopTransMachineBuilder::new(Top).build();
    assert!(matches!(machine.state(), TopTransState::A))
}

#[test]
fn update() {
    let mut machine = TopTransMachineBuilder::new(Top).build();
    machine.update();
    assert!(matches!(machine.state(), TopTransState::B))
}

#[test]
fn top_down_update() {
    let mut machine = TopTransMachineBuilder::new(Top).build();
    machine.top_down_update();
    assert!(matches!(machine.state(), TopTransState::C))
}
