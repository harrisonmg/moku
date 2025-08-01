use moku::*;

struct Under;

#[state_machine]
mod blinky {
    use moku::*;

    #[machine_module]
    mod machine {}

    use machine::BlinkyState;

    struct Event;
    type EventTy = Event;
    impl StateMachineEvent for EventTy;

    struct Top;
    type TopTy = Top;

    impl TopState<BlinkyState> for TopTy {}

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
