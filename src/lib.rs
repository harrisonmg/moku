#![allow(unused)]

mod output;

pub trait StateEnum: Copy {}

pub enum StateEntry<T: StateEnum, U: State<T>> {
    State(U),
    Transition(T),
}

pub trait State<T: StateEnum>: Sized {
    // TODO autogen
    fn enter() -> StateEntry<T, Self>;

    fn init(&mut self) -> Option<T> {
        None
    }

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

pub trait TopState<T: StateEnum>: Sized {
    fn init(&mut self) -> Option<T> {
        None
    }

    fn update(&mut self) -> Option<T> {
        None
    }

    fn top_down_update(&mut self) -> Option<T> {
        None
    }
}

impl<T: StateEnum, U: TopState<T>> State<T> for U {
    fn enter() -> StateEntry<T, Self> {
        unreachable!()
    }

    fn init(&mut self) -> Option<T> {
        TopState::init(self)
    }

    fn update(&mut self) -> Option<T> {
        TopState::update(self)
    }

    fn top_down_update(&mut self) -> Option<T> {
        TopState::top_down_update(self)
    }

    fn exit(self) -> Option<T> {
        unreachable!()
    }
}

pub mod internal {
    use std::marker::PhantomData;

    use super::*;

    pub enum TransitionResult<T> {
        Done,
        MoveUp,
        NewTransition(T),
    }

    pub trait SubstateEnum<T: StateEnum> {
        fn none_variant() -> Self;

        fn is_state(state: T) -> bool;

        fn is_ancestor(state: T) -> bool;

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

    pub enum NodeEntry<T: StateEnum, U: State<T>, V: SubstateEnum<T>> {
        Node(Node<T, U, V>),
        Transition(T),
    }

    pub struct Node<T: StateEnum, U: State<T>, V: SubstateEnum<T>> {
        state: U,
        substate: V,
        phantom: PhantomData<T>,
    }

    impl<T: StateEnum, U: State<T>, V: SubstateEnum<T>> Node<T, U, V> {
        pub fn from_state(state: U) -> Self {
            Self {
                state,
                substate: V::none_variant(),
                phantom: PhantomData,
            }
        }

        pub fn enter() -> NodeEntry<T, U, V> {
            match U::enter() {
                StateEntry::State(state) => NodeEntry::Node(Self {
                    state,
                    substate: V::none_variant(),
                    phantom: PhantomData,
                }),
                StateEntry::Transition(target) => NodeEntry::Transition(target),
            }
        }

        pub fn update(&mut self) -> Option<T> {
            match self.substate.update() {
                Some(target) => Some(target),
                None => self.state.update(),
            }
        }

        pub fn top_down_update(&mut self) -> Option<T> {
            match self.state.top_down_update() {
                Some(target) => Some(target),
                None => self.substate.top_down_update(),
            }
        }

        pub fn exit(self) -> Option<T> {
            self.state.exit()
        }

        pub fn transition(&mut self, target: T) -> TransitionResult<T> {
            // try to transition the current substate towards the target state
            match self.substate.transition(target) {
                // substate is the target state
                TransitionResult::Done => TransitionResult::Done,

                // substate is not the target state or an ancestor of it
                TransitionResult::MoveUp => {
                    if let Some(new_target) = self.substate.exit() {
                        // substate exit resulted in a short circuit transition
                        self.transition(new_target)
                    } else if V::is_ancestor(target) {
                        if let Some(new_target) = self.substate.enter_substate_towards(target) {
                            // substate transition resulted in a short circuit transition
                            TransitionResult::NewTransition(new_target)
                        } else {
                            // substate successfully moved towards target state,
                            // continue transitioning downwards
                            self.substate.transition(target)
                        }
                    } else if V::is_state(target) {
                        // this state is the target
                        match self.state.init() {
                            None => TransitionResult::Done,
                            Some(new_target) => TransitionResult::NewTransition(new_target),
                        }
                    } else {
                        // this state is not the target state or an ancestor of it
                        TransitionResult::MoveUp
                    }
                }

                // substate transition resulted in a short circuit transition
                TransitionResult::NewTransition(new_target) => self.transition(new_target),
            }
        }
    }

    pub trait StateMachine<T: StateEnum, U: TopState<T>> {
        fn from_top_state(top_state: U) -> Self;
        fn update(&mut self);
        fn top_down_update(&mut self);
        fn transition(&mut self, target: T);
    }
}
