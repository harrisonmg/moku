use std::collections::VecDeque;

use moku::*;
use transition_table::*;

#[state_machine(Example)]
mod transition_table {
    use moku::*;

    #[machine_module]
    mod machine {}
    pub use machine::*;

    pub enum Event {
        A,
        B,
    }

    pub struct Top;

    impl TopState<ExampleState> for Top {}

    struct Foo;

    #[superstate(Top)]
    impl State<ExampleState> for Foo {}

    struct Bar;

    #[superstate(Top)]
    impl State<ExampleState> for Bar {}
}

fn handle_event(machine: &mut ExampleMachine, event: &Event) {
    let transition = match machine.state() {
        ExampleState::Top => match event {
            Event::A => Some(ExampleState::Foo),
            Event::B => Some(ExampleState::Bar),
        },
        ExampleState::Foo => match event {
            Event::A => Some(ExampleState::Bar),
            _ => None,
        },
        ExampleState::Bar => match event {
            Event::B => Some(ExampleState::Foo),
            _ => None,
        },
    };

    if let Some(state) = transition {
        machine.transition(state);
    }
}

fn main() {
    let mut machine = ExampleMachineBuilder::new(Top).build();
    let mut events = VecDeque::new();

    // generate events
    events.push_back(Event::A);
    events.push_back(Event::B);

    for event in events {
        handle_event(&mut machine, &event);
    }
}
