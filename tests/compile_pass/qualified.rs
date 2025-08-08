#[moku::state_machine]
mod blinky {
    #[moku::machine_module]
    mod machine {}

    use machine::BlinkyState;

    struct Event;

    impl moku::StateMachineEvent for Event {}

    struct Top;

    impl moku::TopState<BlinkyState, Event> for Top {}

    struct Disabled;

    #[moku::superstate(Top)]
    impl moku::State<BlinkyState, Event> for Disabled {}

    struct Enabled;

    #[moku::superstate(Top)]
    impl moku::State<BlinkyState, Event> for Enabled {}
}

fn main() {}
