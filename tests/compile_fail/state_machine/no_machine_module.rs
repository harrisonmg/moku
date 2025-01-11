use moku::*;

#[state_machine]
mod blinky {
    use moku::*;

    struct Top {}
    impl TopState<BlinkyState> for Top {}
}

fn main() {}
