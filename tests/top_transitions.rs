use moku::*;
use test_log::test;
use tester::{machine::*, *};

#[state_machine]
mod tester {
    use moku::*;

    #[machine_module]
    pub mod machine {}

    use machine::State;

    pub struct Top;

    impl TopState for Top {
        fn init(&mut self) -> impl Into<Next<Self::State>> {
            State::A
        }

        fn update(&mut self) -> impl Into<Next<Self::State>> {
            State::B
        }

        fn top_down_update(&mut self) -> impl Into<Next<Self::State>> {
            State::C
        }
    }

    struct A;

    impl Substate<Top> for A {}

    struct B;

    impl Substate<Top> for B {}

    struct C;

    impl Substate<Top> for C {}
}

#[test]
fn state_chart() {
    assert_eq!(
        STATE_CHART,
        "Top
├─ A
├─ B
└─ C"
    );
}

#[test]
fn init() {
    let machine = Builder::new(Top).build();
    assert!(matches!(machine.state(), State::A));
}

#[test]
fn update() {
    let mut machine = Builder::new(Top).build();
    machine.update();
    assert!(matches!(machine.state(), State::B));
}

#[test]
fn top_down_update() {
    let mut machine = Builder::new(Top).build();
    machine.top_down_update();
    assert!(matches!(machine.state(), State::C));
}
