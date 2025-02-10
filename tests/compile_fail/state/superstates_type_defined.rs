use moku::*;

#[state_machine]
mod blinky {
    use moku::*;

    #[machine_module]
    mod machine {}

    struct Top;
    impl TopState<BlinkyState> for Top {}

    struct Bottom;

    #[superstate(Top)]
    impl State<BlinkyState> for Bottom {
        type Superstates<'a> = std::marker::PhantomData<&'a ()>;
    }
}

fn main() {}
