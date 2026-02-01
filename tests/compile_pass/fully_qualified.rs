#[::moku::state_machine]
mod blinky {
    #[::moku::machine_module]
    mod machine {}

    struct Event;

    impl ::moku::StateMachineEvent for Event {}

    struct Top;

    impl ::moku::TopState for Top {}

    struct Disabled;

    impl ::moku::Substate<Top> for Disabled {}

    struct Enabled;

    impl ::moku::Substate<Top> for Enabled {}
}

fn main() {}
