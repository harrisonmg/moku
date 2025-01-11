use moku::*;

#[state_machine]
mod blinky {
    use moku::*;

    #[machine_module]
    mod state_machine {}

    struct Top;
    impl TopState<BlinkyState> for Top {}

    struct Foo;

    #[superstate(Bar)]
    impl State<BlinkyState> for Foo {}

    struct Bar;

    #[superstate(Foo)]
    impl State<BlinkyState> for Bar {}
}

fn main() {}
