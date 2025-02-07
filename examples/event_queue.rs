use event_queue::*;
use moku::*;

#[state_machine(Example)]
mod event_queue {
    use std::collections::VecDeque;

    use moku::*;

    #[machine_module]
    mod state_machine {}
    pub use state_machine::*;

    pub enum Event {
        A,
        B,
    }

    #[derive(Default)]
    pub struct Top {
        events: VecDeque<Event>,
    }

    impl Top {
        fn current_event(&self) -> Option<&Event> {
            self.events.front()
        }

        pub fn pop_event(&mut self) {
            self.events.pop_front();
        }

        pub fn push_event(&mut self, event: Event) {
            self.events.push_back(event)
        }

        pub fn has_events(&self) -> bool {
            !self.events.is_empty()
        }
    }

    impl TopState<ExampleState> for Top {
        fn update(&mut self) -> Option<ExampleState> {
            // handle events here
            if let Some(event) = self.current_event() {
                match event {
                    Event::A => Some(ExampleState::Foo),
                    Event::B => Some(ExampleState::Bar),
                }
            } else {
                None
            }
        }
    }

    struct Foo;

    #[superstate(Top)]
    impl State<ExampleState> for Foo {
        fn update(&mut self, superstates: &mut Self::Superstates<'_>) -> Option<ExampleState> {
            // handle events here
            // can override superstate event handling with transitions
            if let Some(event) = superstates.top.current_event() {
                match event {
                    Event::A => Some(ExampleState::Bar),
                    _ => None,
                }
            } else {
                None
            }
        }
    }

    struct Bar;

    #[superstate(Top)]
    impl State<ExampleState> for Bar {
        fn update(&mut self, superstates: &mut Self::Superstates<'_>) -> Option<ExampleState> {
            // handle events here
            // can override superstate event handling with transitions
            if let Some(event) = superstates.top.current_event() {
                match event {
                    Event::B => Some(ExampleState::Foo),
                    _ => None,
                }
            } else {
                None
            }
        }
    }
}

fn main() {
    let mut machine = ExampleMachineBuilder::new(Top::default()).build();

    // generate events
    machine.top_mut().push_event(Event::A);
    machine.top_mut().push_event(Event::B);

    while machine.top_ref().has_events() {
        // process one event
        machine.update();

        // clear that event
        machine.top_mut().pop_event();
    }
}
