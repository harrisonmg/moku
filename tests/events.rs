use moku::*;
use test_log::test;
use tester::{machine::*, *};

#[test]
fn state_chart() {
    assert_eq!(
        TESTER_STATE_CHART,
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

    use machine::TesterState;

    pub enum Event {
        A,
        B,
        C,
    }

    impl StateMachineEvent for Event {}

    pub struct Top;

    impl TopState<TesterState, Event> for Top {
        fn handle_event(&mut self, event: &Event) -> impl Into<Next<TesterState>> {
            match event {
                Event::A => Some(TesterState::Foo),
                Event::B => Some(TesterState::Bar),
                Event::C => None,
            }
        }
    }

    struct Foo;
    #[superstate(Top)]
    impl State<TesterState, Event> for Foo {}

    struct Bar;
    #[superstate(Top)]
    impl State<TesterState, Event> for Bar {}

    struct Dropper;

    #[superstate(Top)]
    impl State<TesterState, Event> for Dropper {
        fn handle_event(
            &mut self,
            _event: &Event,
            _superstates: &mut Self::Superstates<'_>,
        ) -> impl Into<EventResponse<TesterState>> {
            EventResponse::Drop
        }
    }

    struct FooPasser;

    #[superstate(Top)]
    impl State<TesterState, Event> for FooPasser {
        fn handle_event(
            &mut self,
            event: &Event,
            _superstates: &mut Self::Superstates<'_>,
        ) -> impl Into<EventResponse<TesterState>> {
            match event {
                Event::C => Some(TesterState::Foo),
                _ => None,
            }
        }
    }

    struct BarPasser;

    #[superstate(FooPasser)]
    impl State<TesterState, Event> for BarPasser {
        fn handle_event(
            &mut self,
            event: &Event,
            _superstates: &mut Self::Superstates<'_>,
        ) -> impl Into<EventResponse<TesterState>> {
            match event {
                Event::A => Some(TesterState::Bar),
                _ => None,
            }
        }
    }
}

#[test]
fn basic() {
    let mut machine = TesterMachineBuilder::new(Top).build();
    assert!(matches!(machine.state(), TesterState::Top));

    machine.handle_event(&Event::A);
    assert!(matches!(machine.state(), TesterState::Foo));

    machine.handle_event(&Event::B);
    assert!(matches!(machine.state(), TesterState::Bar));
}

#[test]
fn drop() {
    let mut machine = TesterMachineBuilder::new(Top).build();
    machine.transition(TesterState::Dropper);
    assert!(matches!(machine.state(), TesterState::Dropper));

    machine.handle_event(&Event::A);
    assert!(matches!(machine.state(), TesterState::Dropper));

    machine.handle_event(&Event::B);
    assert!(matches!(machine.state(), TesterState::Dropper));
}

#[test]
fn foo_pass() {
    let mut machine = TesterMachineBuilder::new(Top).build();
    machine.transition(TesterState::FooPasser);
    assert!(matches!(machine.state(), TesterState::FooPasser));

    machine.handle_event(&Event::A);
    assert!(matches!(machine.state(), TesterState::Foo));

    machine.transition(TesterState::FooPasser);
    assert!(matches!(machine.state(), TesterState::FooPasser));

    machine.handle_event(&Event::B);
    assert!(matches!(machine.state(), TesterState::Bar));

    machine.transition(TesterState::FooPasser);
    assert!(matches!(machine.state(), TesterState::FooPasser));

    machine.handle_event(&Event::C);
    assert!(matches!(machine.state(), TesterState::Foo));
}

#[test]
fn bar_pass() {
    let mut machine = TesterMachineBuilder::new(Top).build();
    machine.transition(TesterState::BarPasser);
    assert!(matches!(machine.state(), TesterState::BarPasser));

    machine.handle_event(&Event::A);
    assert!(matches!(machine.state(), TesterState::Bar));

    machine.transition(TesterState::BarPasser);
    assert!(matches!(machine.state(), TesterState::BarPasser));

    machine.handle_event(&Event::B);
    assert!(matches!(machine.state(), TesterState::Bar));

    machine.transition(TesterState::BarPasser);
    assert!(matches!(machine.state(), TesterState::BarPasser));

    machine.handle_event(&Event::C);
    assert!(matches!(machine.state(), TesterState::Foo));
}
