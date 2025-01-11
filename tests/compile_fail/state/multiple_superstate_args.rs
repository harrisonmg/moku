use moku::*;

#[state_machine]
mod blinky {
    use moku::*;

    #[machine_module]
    mod state_machine {}

    struct Top;
    impl TopState<BlinkyState> for Top {}

    #[superstate(Top, Top)]
    impl State<BlinkyState> for Bottom {}
}

fn main() {}
