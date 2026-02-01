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

    impl TopState for Top {}

    struct A;

    impl Substate<Top> for A {
        fn init(&mut self, _ctx: &mut Self::Context<'_>) -> impl Into<Next<Self::State>> {
            State::B
        }
    }

    struct B;

    impl Substate<Top> for B {
        fn update(&mut self, _ctx: &mut Self::Context<'_>) -> impl Into<Next<Self::State>> {
            State::C
        }
    }

    struct C;

    impl Substate<Top> for C {
        fn top_down_update(
            &mut self,
            _ctx: &mut Self::Context<'_>,
        ) -> impl Into<Next<Self::State>> {
            State::D
        }
    }

    struct D;

    impl Substate<Top> for D {
        fn exit(self, _ctx: &mut Self::Context<'_>) -> impl Into<Next<Self::State>> {
            State::E
        }
    }

    struct E;

    impl Substate<Top> for E {}

    struct F;

    impl Substate<Top> for F {
        fn enter(_ctx: &mut Self::Context<'_>) -> impl Into<Entry<Self::State, Self>> {
            Entry::Target(State::Top)
        }
    }
}

#[test]
fn state_chart() {
    assert_eq!(
        STATE_CHART,
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
    let mut machine = Builder::new(Top).build();
    machine.transition(State::A);
    assert!(matches!(machine.state(), State::B));
}

#[test]
fn update() {
    let mut machine = Builder::new(Top).build();
    machine.transition(State::B);
    machine.update();
    assert!(matches!(machine.state(), State::C));
}

#[test]
fn top_down_update() {
    let mut machine = Builder::new(Top).build();
    machine.transition(State::C);
    machine.top_down_update();
    assert!(matches!(machine.state(), State::D));
}

#[test]
fn exit() {
    let mut machine = Builder::new(Top).build();
    machine.transition(State::D);
    machine.transition(State::A);
    assert!(matches!(machine.state(), State::E));
}

#[test]
fn enter() {
    let mut machine = Builder::new(Top).build();
    machine.transition(State::F);
    assert!(matches!(machine.state(), State::Top));
}
