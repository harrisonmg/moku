use moku::*;

#[state_machine]
mod blinky {
    use moku::*;

    #[machine_module]
    mod state_machine {}

    #[derive(Debug, Clone, Copy)]
    enum BlinkyState {}
    impl StateEnum for BlinkyState {}

    struct Top {}
    impl TopState<BlinkyState> for Top {}

    #[superstate(Top)]
    #[superstate(Top)]
    impl State<BlinkyState> for Bottom {
        type Superstates<'a> = std::marker::PhantomData<&'a ()>;

        fn enter<'a>(_superstates: &mut Self::Superstates<'a>) -> StateEntry<Self, BlinkyState> {
            StateEntry::State(Self {})
        }
    }
}

fn main() {}
