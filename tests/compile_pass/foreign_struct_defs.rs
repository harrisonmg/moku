use moku::*;

struct Under;

#[state_machine]
mod blinky {
    use moku::*;

    #[machine_module]
    mod machine {}

    struct Event;
    type EventTy = Event;
    impl StateMachineEvent for EventTy {}

    struct Top;
    impl TopState for Top {}

    struct Bottom;
    type BottomTy = Bottom;

    impl Substate<Top> for BottomTy {
        fn enter(_ctx: &mut Self::Context<'_>) -> impl Into<Entry<Self::State, Self>> {
            Entry::State(Self {})
        }
    }

    use super::Under;

    impl Substate<BottomTy> for Under {
        fn enter(_ctx: &mut Self::Context<'_>) -> impl Into<Entry<Self::State, Self>> {
            Entry::State(Self {})
        }
    }

    struct Inside;

    impl Substate<Under> for Inside {}
}

fn main() {}
