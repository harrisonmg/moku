#![allow(clippy::upper_case_acronyms)]

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
    impl Substate<Top> for A {}

    struct AA;
    impl Substate<A> for AA {
        fn enter(_ctx: &mut Self::Context<'_>) -> impl Into<Entry<Self::State, Self>> {
            Self
        }

        fn init(&mut self, _ctx: &mut Self::Context<'_>) -> impl Into<Next<Self::State>> {
            State::A
        }

        fn update(&mut self, _ctx: &mut Self::Context<'_>) -> impl Into<Next<Self::State>> {
            State::A
        }

        fn top_down_update(
            &mut self,
            _ctx: &mut Self::Context<'_>,
        ) -> impl Into<Next<Self::State>> {
            State::A
        }

        fn handle_event(
            &mut self,
            _ctx: &mut Self::Context<'_>,
            _event: &Self::Event,
        ) -> impl Into<Response<Self::State>> {
            State::A
        }

        fn exit(self, _ctx: &mut Self::Context<'_>) -> impl Into<Next<Self::State>> {
            State::Top
        }
    }

    struct B;
    impl Substate<Top> for B {
        fn update(&mut self, _ctx: &mut Self::Context<'_>) -> impl Into<Next<Self::State>> {
            Next::ExactTarget(State::Top)
        }

        fn top_down_update(
            &mut self,
            _ctx: &mut Self::Context<'_>,
        ) -> impl Into<Next<Self::State>> {
            Next::ExactTarget(State::Top)
        }

        fn handle_event(
            &mut self,
            _ctx: &mut Self::Context<'_>,
            _event: &Self::Event,
        ) -> impl Into<Response<Self::State>> {
            Next::ExactTarget(State::Top)
        }

        fn exit(self, _ctx: &mut Self::Context<'_>) -> impl Into<Next<Self::State>> {
            Next::ExactTarget(State::Top)
        }
    }

    struct BA;
    impl Substate<B> for BA {
        fn enter(_ctx: &mut Self::Context<'_>) -> impl Into<Entry<Self::State, Self>> {
            Entry::ExactTarget(State::Top)
        }
    }

    struct BB;
    impl Substate<B> for BB {
        fn init(&mut self, _ctx: &mut Self::Context<'_>) -> impl Into<Next<Self::State>> {
            Next::ExactTarget(State::Top)
        }
    }
}

#[test]
fn state_machine() {
    let mut machine = Builder::new(Top {}).build();

    machine.transition(State::A);
    assert!(matches!(machine.state(), State::A));

    machine.transition(State::Top);
    assert!(matches!(machine.state(), State::A));

    machine.exact_transition(State::Top);
    assert!(matches!(machine.state(), State::Top));
}

#[test]
fn normal_transition() {
    let mut machine = Builder::new(Top {}).build();

    machine.transition(State::AA);
    assert!(matches!(machine.state(), State::AA));

    machine.update();
    assert!(matches!(machine.state(), State::AA));

    machine.top_down_update();
    assert!(matches!(machine.state(), State::AA));

    machine.handle_event(&());
    assert!(matches!(machine.state(), State::AA));

    machine.exact_transition(State::A);
    assert!(matches!(machine.state(), State::A));
}

#[test]
fn exact_transition() {
    let mut machine = Builder::new(Top {}).build();

    machine.transition(State::B);
    assert!(matches!(machine.state(), State::B));
    machine.update();
    assert!(matches!(machine.state(), State::Top));

    machine.transition(State::B);
    assert!(matches!(machine.state(), State::B));
    machine.top_down_update();
    assert!(matches!(machine.state(), State::Top));

    machine.transition(State::B);
    assert!(matches!(machine.state(), State::B));
    machine.handle_event(&());
    assert!(matches!(machine.state(), State::Top));

    machine.transition(State::B);
    assert!(matches!(machine.state(), State::B));
    machine.exact_transition(State::A);
    assert!(matches!(machine.state(), State::Top));

    machine.transition(State::BA);
    assert!(matches!(machine.state(), State::Top));

    machine.transition(State::BB);
    assert!(matches!(machine.state(), State::Top));
}
