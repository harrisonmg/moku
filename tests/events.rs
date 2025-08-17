use event::{machine::*, *};
use moku::*;
use test_log::test;

#[state_machine]
mod event {
    use moku::*;

    #[machine_module]
    pub mod machine {}

    use machine::EventState;

    pub enum Event {
        A,
        B,
        C,
    }

    impl StateMachineEvent for Event {}

    pub struct Top;

    impl TopState<EventState, Event> for Top {
        fn handle_event(&mut self, event: &Event) -> Option<EventState> {
            match event {
                Event::A => Some(EventState::Foo),
                Event::B => Some(EventState::Bar),
                Event::C => None,
            }
        }
    }

    struct Foo;
    #[superstate(Top)]
    impl State<EventState, Event> for Foo {}

    struct Bar;
    #[superstate(Top)]
    impl State<EventState, Event> for Bar {}

    struct Dropper;

    #[superstate(Top)]
    impl State<EventState, Event> for Dropper {
        fn handle_event(
            &mut self,
            event: &Event,
            _superstates: &mut Self::Superstates<'_>,
        ) -> EventResult<EventState> {
            EventResult::Drop
        }
    }

    struct FooPasser;

    #[superstate(Top)]
    impl State<EventState, Event> for FooPasser {
        fn handle_event(
            &mut self,
            event: &Event,
            _superstates: &mut Self::Superstates<'_>,
        ) -> EventResult<EventState> {
            match event {
                Event::C => EventResult::Transition(EventState::Foo),
                _ => EventResult::Defer,
            }
        }
    }

    struct BarPasser;

    #[superstate(FooPasser)]
    impl State<EventState, Event> for BarPasser {
        fn handle_event(
            &mut self,
            event: &Event,
            _superstates: &mut Self::Superstates<'_>,
        ) -> EventResult<EventState> {
            match event {
                Event::A => EventResult::Transition(EventState::Bar),
                _ => EventResult::Defer,
            }
        }
    }
}

#[test]
fn state_chart() {
    assert_eq!(
        EVENT_STATE_CHART,
        "Top
├─ Foo
├─ Bar
├─ Dropper
└─ FooPasser
   └─ BarPasser"
    );
}

#[test]
fn basic() {
    let mut machine = EventMachineBuilder::new(Top).build();
    assert!(matches!(machine.state(), EventState::Top));

    machine.handle_event(&Event::A);
    assert!(matches!(machine.state(), EventState::Foo));

    machine.transition(EventState::Top);
    assert!(matches!(machine.state(), EventState::Top));

    machine.handle_event(&Event::B);
    assert!(matches!(machine.state(), EventState::Bar));
}

#[test]
fn drop() {
    let mut machine = EventMachineBuilder::new(Top).build();
    machine.transition(EventState::Dropper);
    assert!(matches!(machine.state(), EventState::Dropper));

    machine.handle_event(&Event::A);
    assert!(matches!(machine.state(), EventState::Dropper));

    machine.handle_event(&Event::B);
    assert!(matches!(machine.state(), EventState::Dropper));
}

#[test]
fn foo_pass() {
    let mut machine = EventMachineBuilder::new(Top).build();
    machine.transition(EventState::FooPasser);
    assert!(matches!(machine.state(), EventState::FooPasser));

    machine.handle_event(&Event::A);
    assert!(matches!(machine.state(), EventState::Foo));

    machine.transition(EventState::FooPasser);
    assert!(matches!(machine.state(), EventState::FooPasser));

    machine.handle_event(&Event::B);
    assert!(matches!(machine.state(), EventState::Bar));

    machine.transition(EventState::FooPasser);
    assert!(matches!(machine.state(), EventState::FooPasser));

    machine.handle_event(&Event::C);
    assert!(matches!(machine.state(), EventState::Foo));
}

#[test]
fn bar_pass() {
    let mut machine = EventMachineBuilder::new(Top).build();
    machine.transition(EventState::BarPasser);
    assert!(matches!(machine.state(), EventState::BarPasser));

    machine.handle_event(&Event::A);
    assert!(matches!(machine.state(), EventState::Bar));

    machine.transition(EventState::BarPasser);
    assert!(matches!(machine.state(), EventState::BarPasser));

    machine.handle_event(&Event::B);
    assert!(matches!(machine.state(), EventState::Bar));

    machine.transition(EventState::BarPasser);
    assert!(matches!(machine.state(), EventState::BarPasser));

    machine.handle_event(&Event::C);
    assert!(matches!(machine.state(), EventState::Foo));
}
