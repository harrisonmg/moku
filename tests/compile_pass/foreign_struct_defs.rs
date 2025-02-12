use moku::*;

struct Under;

#[state_machine]
mod blinky {
    use moku::*;

    #[machine_module]
    mod machine {}

    use machine::BlinkyState;

    struct Top;
    impl TopState<BlinkyState> for Top {}

    struct Bottom;

    type BottomTy = Bottom;

    #[superstate(Top)]
    impl State<BlinkyState> for BottomTy {
        fn enter(_superstates: &mut Self::Superstates<'_>) -> StateEntry<Self, BlinkyState> {
            StateEntry::State(Self {})
        }
    }

    use super::Under;

    #[superstate(BottomTy)]
    impl State<BlinkyState> for Under {
        fn enter(_superstates: &mut Self::Superstates<'_>) -> StateEntry<Self, BlinkyState> {
            StateEntry::State(Self {})
        }
    }

    struct Inside;

    #[superstate(Under)]
    impl State<BlinkyState> for Inside {}
}

fn main() {}
