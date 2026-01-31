use moku::*;

#[state_machine]
mod blinky {
    use moku::*;

    #[machine_module]
    mod machine {}

    struct EventA;

    impl StateMachineEvent for EventA {}

    struct EventB;

    impl StateMachineEvent for EventB {}

    struct Top;

    impl TopState for Top {}
}

fn main() {}
