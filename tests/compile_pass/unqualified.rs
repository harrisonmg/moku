use moku::*;

#[state_machine]
mod blinky {
    use moku::*;

    #[machine_module]
    mod machine {}

    use machine::BlinkyState;

    struct Event;

    impl StateMachineEvent for Event {}

    struct Top;

    impl TopState<BlinkyState, Event> for Top {}

    struct Disabled;

    #[superstate(Top)]
    impl State<BlinkyState, Event> for Disabled {}

    struct Enabled;

    #[superstate(Top)]
    impl State<BlinkyState, Event> for Enabled {}
}

fn main() {}
