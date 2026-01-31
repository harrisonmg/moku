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

    impl TopState for Top {
        fn handle_event(&mut self, event: &Self::Event) -> impl Into<Next<Self::State>> {
            match event {
                Event::StomachGrumbled => Some(State::Hunting),
                Event::PreyCaught => Some(State::Cooking),
                _ => None,
            }
        }
    }

    struct Hunting;

    impl Substate<Top> for Hunting {
        // by default, states will defer all events
    }

    struct Cooking;

    impl Substate<Top> for Cooking {
        fn handle_event(
            &mut self,
            event: &Self::Event,
            _ctx: &mut Self::Context<'_>,
        ) -> impl Into<EventResponse<Self::State>> {
            match event {
                Event::MeatCooked => State::Top.into(),
                // ignore Top state's logic to start hunting when stomach grumbles
                Event::StomachGrumbled => EventResponse::Drop,
                // defer other events to superstates
                _ => None.into(),
            }
        }
    }
}

fn main() {
    let mut machine = Builder::new(Top).build();
    let mut events = VecDeque::new();

    // generate events
    events.push_back(Event::StomachGrumbled);
    events.push_back(Event::PreyCaught);
    events.push_back(Event::MeatCooked);

    for event in events {
        machine.handle_event(&event);
    }
}
