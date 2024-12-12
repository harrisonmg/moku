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

    fn init(&mut self) -> T;

    fn update(&mut self) -> Option<T> {
        None
    }

    fn top_down_update(&mut self) -> Option<T> {
        None
    }

    fn exit(self) -> Option<T> {
        None
    }
}

pub mod internal {
    use std::{collections::btree_set::Union, marker::PhantomData};

    use enumset::{enum_set, EnumSet};

    use super::*;

    pub enum TransitionResult<T: StateEnum> {
        Done,
        MoveUp,
        NewTransition(T),
    }

    pub trait SubstateEnum<T: StateEnum, U: State<T>> {
        const STATE: EnumSet<T>;
        const DECENDENTS: EnumSet<T> = enum_set!();

        fn none_variant() -> Self;

        fn update(&mut self) -> Option<T> {
            None
        }

        fn top_down_update(&mut self) -> Option<T> {
            None
        }

        fn exit(&mut self) -> Option<T> {
            None
        }

        fn transition(&mut self, target: T) -> TransitionResult<T> {
            TransitionResult::MoveUp
        }

        fn enter_substate_towards(&mut self, target: T) -> Option<T> {
            unreachable!()
        }
    }

    pub enum NodeEntry<T: StateEnum, U: State<T>, V: SubstateEnum<T, U>> {
        Node(Node<T, U, V>),
        Transition(T),
    }

    pub struct Node<T: StateEnum, U: State<T>, V: SubstateEnum<T, U>> {
        state: U,
        substate: V,
        phantom: PhantomData<T>,
    }

    impl<T: StateEnum, U: State<T>, V: SubstateEnum<T, U>> Node<T, U, V> {
        fn enter() -> NodeEntry<T, U, V> {
            match U::enter() {
                StateEntry::State(state) => NodeEntry::Node(Self {
                    state,
                    substate: V::none_variant(),
                    phantom: PhantomData,
                }),
                StateEntry::Transition(target) => NodeEntry::Transition(target),
            }
        }

        fn update(&mut self) -> Option<T> {
            match self.substate.update() {
                Some(target) => Some(target),
                None => self.state.update(),
            }
        }

        fn top_down_update(&mut self) -> Option<T> {
            match self.state.update() {
                Some(target) => Some(target),
                None => self.substate.update(),
            }
        }

        fn exit(self) -> Option<T> {
            self.state.exit()
        }

        fn transition(&mut self, target: T) -> TransitionResult<T> {
            match self.substate.transition(target) {
                TransitionResult::Done => TransitionResult::Done,
                TransitionResult::MoveUp => {
                    if let Some(new_target) = self.substate.exit() {
                        TransitionResult::NewTransition(new_target)
                    } else if V::DECENDENTS.contains(target) {
                        if let Some(new_target) = self.substate.enter_substate_towards(target) {
                            TransitionResult::NewTransition(new_target)
                        } else {
                            self.substate.transition(target)
                        }
                    } else if V::STATE.contains(target) {
                        let init_transition = self.state.init();
                        if V::STATE.contains(init_transition) {
                            TransitionResult::Done
                        } else {
                            TransitionResult::NewTransition(init_transition)
                        }
                    } else {
                        TransitionResult::MoveUp
                    }
                }
                TransitionResult::NewTransition(new_target) => self.transition(new_target),
            }
        }
    }
}
