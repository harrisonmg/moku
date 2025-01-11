use moku::*;

#[state_machine]
mod blinky {
    use moku::*;

    #[machine_module]
    mod state_machine {}

    struct Top;
    impl TopState<BlinkyState> for Top {}

    struct Bottom<T> {
        t: T,
    }

    #[superstate(Top)]
    impl<T> State<BlinkyState> for Bottom<T> {}
}

fn main() {}
