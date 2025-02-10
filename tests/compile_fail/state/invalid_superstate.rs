use moku::*;

#[state_machine]
mod blinky {
    use moku::*;

    #[machine_module]
    mod machine {}

    struct Top;
    impl TopState<BlinkyState> for Top {}

    struct Bottom;

    #[superstate(Tob)]
    impl State<BlinkyState> for Bottom {}
}

fn main() {}
