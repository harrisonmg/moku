use moku::*;

struct Bottom;

#[state_machine]
mod blinky {
    use moku::*;

    #[machine_module]
    mod machine {}

    use machine::BlinkyState;

    struct Top;
    impl TopState<BlinkyState> for Top {}

    use super::Bottom;

    #[superstate(Top)]
    impl State<BlinkyState> for Bottom {}
}

fn main() {}
