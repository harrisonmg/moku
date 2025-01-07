mod blinky {
    use moku::*;

    struct Top {}

    #[derive(Debug, Clone, Copy)]
    enum BlinkyState {}
    impl StateEnum for BlinkyState {}

    struct Bottom {}

    #[superstate(Top)]
    impl State<BlinkyState> for Bottom {}
}

fn main() {}
