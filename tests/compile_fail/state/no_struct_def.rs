use moku::*;

#[state_machine]
mod blinky {
    use moku::*;

    #[machine_module]
    mod machine {}

    struct Top;
    impl TopState<BlinkyState> for Top {}

    struct Bottom;

    type BottomTy = Bottom;

    #[superstate(Top)]
    impl State<BlinkyState> for BottomTy {}
}

fn main() {}
