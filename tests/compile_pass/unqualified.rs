use moku::*;

#[state_machine]
mod blinky {
    use moku::*;

    #[machine_module]
    mod machine {}

    use machine::BlinkyState;

    struct Top;

    impl TopState<BlinkyState> for Top {}

    struct Disabled;

    #[superstate(Top)]
    impl State<BlinkyState> for Disabled {}

    struct Enabled;

    #[superstate(Top)]
    impl State<BlinkyState> for Enabled {}
}

fn main() {}
