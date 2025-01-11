use moku::*;

#[state_machine]
mod blinky {
    use moku::*;

    #[machine_module]
    mod state_machine {}

    struct Top;
    impl TopState<BlinkyState> for Top {}

    struct Foo;

    #[superstate(Beef)]
    impl State<BlinkyState> for Foo {}

    struct Bar;

    #[superstate(Foo)]
    impl State<BlinkyState> for Bar {}

    struct Fizz;

    #[superstate(Foo)]
    impl State<BlinkyState> for Fizz {}

    struct Buzz;

    #[superstate(Fizz)]
    impl State<BlinkyState> for Buzz {}

    struct Dead;

    #[superstate(Buzz)]
    impl State<BlinkyState> for Dead {}

    struct Beef;

    #[superstate(Dead)]
    impl State<BlinkyState> for Beef {}
}

fn main() {}
