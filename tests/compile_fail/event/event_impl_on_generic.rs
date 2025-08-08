use moku::*;

#[state_machine]
mod blinky {
    use moku::*;

    #[machine_module]
    mod machine {}

    use machine::BlinkyState;

    struct Event<T>;

    impl<T> StateMachineEvent for Event<T> {}

    struct Top;

    impl TopState<BlinkyState, Event> for Top {}
}

fn main() {}
