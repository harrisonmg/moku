use moku::*;

#[state_machine]
mod blinky {
    use moku::*;

    #[machine_module]
    mod machine {}

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    enum BlonkyState {}
    impl StateEnum for BlonkyState {}

    struct Top;
    impl TopState<BlinkyState> for Top {}

    #[superstate(Top)]
    impl State<BlonkyState> for Bottom {}
}

fn main() {}
