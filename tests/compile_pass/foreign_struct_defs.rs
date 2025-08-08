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
    impl StateMachineEvent for EventTy {}

    struct Top;
    impl TopState<BlinkyState, EventTy> for Top {}

    struct Bottom;
    type BottomTy = Bottom;

    #[superstate(Top)]
    impl State<BlinkyState, EventTy> for BottomTy {
        fn enter(_superstates: &mut Self::Superstates<'_>) -> StateEntry<Self, BlinkyState> {
            StateEntry::State(Self {})
        }
    }

    use super::Under;

    #[superstate(BottomTy)]
    impl State<BlinkyState, EventTy> for Under {
        fn enter(_superstates: &mut Self::Superstates<'_>) -> StateEntry<Self, BlinkyState> {
            StateEntry::State(Self {})
        }
    }

    struct Inside;

    #[superstate(Under)]
    impl State<BlinkyState, EventTy> for Inside {}
}

fn main() {}
