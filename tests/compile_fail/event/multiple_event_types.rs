use moku::*;

#[state_machine]
mod blinky {
    use moku::*;

    #[machine_module]
    mod machine {}

    use machine::BlinkyState;

    struct EventA;

    impl StateMachineEvent for EventA {}

    struct EventB;

    impl StateMachineEvent for EventB {}

    struct Top;

    impl TopState<BlinkyState, EventA> for Top {}
}

fn main() {}
