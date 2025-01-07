use moku::*;

#[state_machine]
mod blinky {
    use moku::*;

    enum BlinkyState {}
    impl StateEnum for BlinkyState {}

    struct Top {}
    impl TopState<BlinkyState> for Top {}
}

fn main() {}
