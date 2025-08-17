use std::collections::VecDeque;

use hunter::*;
use moku::*;

#[state_machine]
mod hunter {
    use moku::*;

    #[machine_module]
    mod machine {}
    pub use machine::*;

    pub enum Event {
        StomachGrumbled,
        PreyCaught,
        MeatCooked,
    }

    impl StateMachineEvent for Event {}

    pub struct Top;

    impl TopState<HunterState, Event> for Top {
        fn handle_event(&mut self, event: &Event) -> Option<HunterState> {
            match event {
                Event::StomachGrumbled => Some(HunterState::Hunting),
                Event::PreyCaught => Some(HunterState::Cooking),
                _ => None,
            }
        }
    }

    struct Hunting;

    #[superstate(Top)]
    impl State<HunterState, Event> for Hunting {
        // by default, states will defer all events
    }

    struct Cooking;

    #[superstate(Top)]
    impl State<HunterState, Event> for Cooking {
        fn handle_event(
            &mut self,
            event: &Event,
            _superstates: &mut Self::Superstates<'_>,
        ) -> EventResult<HunterState> {
            match event {
                Event::MeatCooked => EventResult::Transition(HunterState::Top),
                // ignore Top state's logic to start hunting when stomach grumbles
                Event::StomachGrumbled => EventResult::Drop,
                // defer other events to superstates
                _ => EventResult::Defer,
            }
        }
    }
}

fn main() {
    let mut machine = HunterMachineBuilder::new(Top).build();
    let mut events = VecDeque::new();

    // generate events
    events.push_back(Event::StomachGrumbled);
    events.push_back(Event::PreyCaught);
    events.push_back(Event::MeatCooked);

    for event in events {
        machine.handle_event(&event);
    }
}
