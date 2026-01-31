use moku::*;
use test_log::test;
use tester::{machine::*, *};

#[test]
fn state_chart() {
    assert_eq!(
        STATE_CHART,
        "Top
├─ Foo
├─ Bar
├─ Dropper
└─ FooPasser
   └─ BarPasser"
    );
}

#[state_machine]
mod tester {
    use moku::*;

    #[machine_module]
    pub mod machine {}

    use machine::State;

    pub enum Event {
        A,
        B,
        C,
    }

    impl StateMachineEvent for Event {}

    pub struct Top;

    impl TopState for Top {
        fn handle_event(&mut self, event: &Self::Event) -> impl Into<Next<Self::State>> {
            match event {
                Event::A => Some(State::Foo),
                Event::B => Some(State::Bar),
                Event::C => None,
            }
        }
    }

    struct Foo;
    impl Substate<Top> for Foo {}

    struct Bar;
    impl Substate<Top> for Bar {}

    struct Dropper;

    impl Substate<Top> for Dropper {
        fn handle_event(
            &mut self,
            _event: &Self::Event,
            _ctx: &mut Self::Context<'_>,
        ) -> impl Into<EventResponse<Self::State>> {
            EventResponse::Drop
        }
    }

    struct FooPasser;

    impl Substate<Top> for FooPasser {
        fn handle_event(
            &mut self,
            event: &Self::Event,
            _ctx: &mut Self::Context<'_>,
        ) -> impl Into<EventResponse<Self::State>> {
            match event {
                Event::C => Some(State::Foo),
                _ => None,
            }
        }
    }

    struct BarPasser;

    impl Substate<FooPasser> for BarPasser {
        fn handle_event(
            &mut self,
            event: &Self::Event,
            _ctx: &mut Self::Context<'_>,
        ) -> impl Into<EventResponse<Self::State>> {
            match event {
                Event::A => Some(State::Bar),
                _ => None,
            }
        }
    }
}

#[test]
fn basic() {
    let mut machine = Builder::new(Top).build();
    assert!(matches!(machine.state(), State::Top));

    machine.handle_event(&Event::A);
    assert!(matches!(machine.state(), State::Foo));

    machine.handle_event(&Event::B);
    assert!(matches!(machine.state(), State::Bar));
}

#[test]
fn drop() {
    let mut machine = Builder::new(Top).build();
    machine.transition(State::Dropper);
    assert!(matches!(machine.state(), State::Dropper));

    machine.handle_event(&Event::A);
    assert!(matches!(machine.state(), State::Dropper));

    machine.handle_event(&Event::B);
    assert!(matches!(machine.state(), State::Dropper));
}

#[test]
fn foo_pass() {
    let mut machine = Builder::new(Top).build();
    machine.transition(State::FooPasser);
    assert!(matches!(machine.state(), State::FooPasser));

    machine.handle_event(&Event::A);
    assert!(matches!(machine.state(), State::Foo));

    machine.transition(State::FooPasser);
    assert!(matches!(machine.state(), State::FooPasser));

    machine.handle_event(&Event::B);
    assert!(matches!(machine.state(), State::Bar));

    machine.transition(State::FooPasser);
    assert!(matches!(machine.state(), State::FooPasser));

    machine.handle_event(&Event::C);
    assert!(matches!(machine.state(), State::Foo));
}

#[test]
fn bar_pass() {
    let mut machine = Builder::new(Top).build();
    machine.transition(State::BarPasser);
    assert!(matches!(machine.state(), State::BarPasser));

    machine.handle_event(&Event::A);
    assert!(matches!(machine.state(), State::Bar));

    machine.transition(State::BarPasser);
    assert!(matches!(machine.state(), State::BarPasser));

    machine.handle_event(&Event::B);
    assert!(matches!(machine.state(), State::Bar));

    machine.transition(State::BarPasser);
    assert!(matches!(machine.state(), State::BarPasser));

    machine.handle_event(&Event::C);
    assert!(matches!(machine.state(), State::Foo));
}
