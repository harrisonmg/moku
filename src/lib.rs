#![allow(unused)]

mod output;

pub use enumset::EnumSetType;

pub trait StateList: Copy + EnumSetType {}

pub enum StateEntry<T: StateList, U: State<T>> {
    State(U),
    Transition(T),
}

pub trait State<T: StateList>: Sized {
    fn enter() -> StateEntry<T, Self>;

    fn update(&mut self) -> Option<T> {
        None
    }

    fn exit(self) -> Option<T> {
        None
    }
}

pub trait BranchState<T: StateList>: State<T> {
    fn init(&mut self) -> T;
}

pub mod internal {
    use enumset::EnumSet;

    use super::*;

    pub trait Leaf<T: StateList, U: State<T>> {
        const STATE: EnumSet<T>;

        fn get_state(&mut self) -> &mut U;

        fn take_state(self) -> U;

        fn transition(&mut self, target: T) -> TransitionResult<T> {
            if Self::STATE.contains(target) {
                TransitionResult::Done
            } else {
                TransitionResult::MoveUp
            }
        }
    }

    pub trait Branch<T: StateList, U: BranchState<T>> {
        const STATE: EnumSet<T>;
        const CHILDREN: EnumSet<T>;

        fn get_state(&mut self) -> &mut U;

        fn take_state(self) -> U;

        fn transition_child(&mut self, target: T) -> TransitionResult<T>;

        fn transition(&mut self, target: T) -> TransitionResult<T> {
            if Self::STATE.contains(target) {
                TransitionResult::NewTransition(self.get_state().init())
            } else if Self::CHILDREN.contains(target) {
                self.transition_child(target)
            } else {
                TransitionResult::MoveUp
            }
        }
    }

    pub enum TransitionResult<T: StateList> {
        Done,
        MoveUp,
        NewTransition(T),
    }
}
