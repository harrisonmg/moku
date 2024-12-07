#![allow(unused)]

mod output;

use enumset::EnumSet;
pub use enumset::EnumSetType;

pub trait StateEnum: Copy + EnumSetType {
    fn get_decendents(&mut self) -> EnumSet<Self>;
}

pub enum StateEntry<T: StateEnum, U: State<T>> {
    State(U),
    Transition(T),
}

pub trait State<T: StateEnum>: Sized {
    fn enter() -> StateEntry<T, Self>;

    fn update(&mut self) -> Option<T> {
        None
    }

    fn exit(self) -> Option<T> {
        None
    }
}

pub trait BranchState<T: StateEnum>: State<T> {
    fn init(&mut self) -> T;
}

pub mod internal {
    use std::marker::PhantomData;

    use enumset::EnumSet;

    use super::*;

    pub enum TransitionResult<T: StateEnum> {
        Done,
        MoveUp,
        NewTransition(T),
    }

    pub trait SubStateEnum<T: StateEnum, U: State<T>> {}

    pub enum NodeEntry<T: StateEnum, U: State<T>, V: Node<T, U>> {
        Node(V, PhantomData<U>),
        Transition(T),
    }

    pub struct Node<T: StateEnum, U: State<T>, V: SubStateEnum<T, U>> {
        state: U,
        substate: V,
        phantom: PhantomData<T>,

        fn enter() -> NodeEntry<T, U, Self> {
            match U::enter() {
                StateEntry::State(state) => NodeEntry::Node(Self::from_state(state), PhantomData),
                StateEntry::Transition(target) => NodeEntry::Transition(target),
            }
        }

        fn update(&mut self) -> Option<T> {
            self.state.update()
        }
    }

    //pub trait Leaf<T: StateEnum, U: State<T>>: Node<T, U> {
    //    const STATE: EnumSet<T>;

    //    fn transition(&mut self, target: T) -> TransitionResult<T> {
    //        if Self::STATE.contains(target) {
    //            TransitionResult::Done
    //        } else {
    //            TransitionResult::MoveUp
    //        }
    //    }
    //}

    //pub trait Branch<T: StateEnum, U: BranchState<T>>: Node<T, U> {
    //    const STATE: EnumSet<T>;
    //    const CHILDREN: EnumSet<T>;

    //    fn transition_child(&mut self, target: T) -> TransitionResult<T>;

    //    fn transition(&mut self, target: T) -> TransitionResult<T> {
    //        if Self::STATE.contains(target) {
    //            TransitionResult::NewTransition(self.get_state().init())
    //        } else if Self::CHILDREN.contains(target) {
    //            self.transition_child(target)
    //        } else {
    //            TransitionResult::MoveUp
    //        }
    //    }
    //}
}
