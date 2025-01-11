use moku::*;

#[state_machine]
mod blinky {
    use moku::*;

    #[machine_module]
    mod state_machine {}

    struct Top;
    impl TopState<BlinkyState> for Top {}

    struct OtherTop;
    impl TopState<BlinkyState> for OtherTop {}
}

fn main() {}
