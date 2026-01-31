use moku::*;

#[state_machine]
mod blinky {
    use moku::*;

    #[machine_module]
    mod machine {}

    struct Event;

    impl StateMachineEvent for Event {}

    struct Top;

    impl TopState for Top {}

    struct Disabled;

    impl Substate<Top> for Disabled {}

    struct Enabled;

    impl Substate<Top> for Enabled {}
}

fn main() {}
