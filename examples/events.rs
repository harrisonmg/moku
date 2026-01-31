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
        fn handle_event(&mut self, event: &Self::Event) -> Self::Next {
            match event {
                Event::StomachGrumbled => State::Hunting.into(),
                Event::PreyCaught => State::Cooking.into(),
                _ => Next::None,
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
        ) -> Self::Response {
            match event {
                Event::MeatCooked => State::Top.into(),
                // ignore Top state's logic to start hunting when stomach grumbles
                Event::StomachGrumbled => Response::Drop,
                // defer other events to superstates
                _ => Response::default(),
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
