#![allow(unused)]

use std::cell::Ref;

mod output;

pub trait StateEnum: std::fmt::Debug + Copy {}

pub trait StateMachine<'a, T: StateEnum> {
    type StateRefs;
    fn update(&mut self);
    fn top_down_update(&mut self);
    fn transition(&mut self, target: T);
    fn state(&self) -> T;
    fn name(&self) -> &str;
    fn set_name(&mut self, name: String);
    fn state_matches(&self, state: T) -> bool;
}

pub trait StateMachineBuilder<'a, T, U, V>
where
    T: StateEnum,
    U: StateMachine<'a, T>,
    V: TopState<T>,
{
    fn new(top_state: V) -> Self;
    fn name(self, name: &str) -> Self;
    fn build(self) -> U;
}

pub trait StateRef<'a, T, U, V>
where
    T: StateEnum,
    U: StateMachine<'a, T>,
    V: State<'a, T, U>,
{
    fn get_ref(&self) -> Option<&V>;
    fn get_ref_mut(&mut self) -> Option<&mut V>;
}

pub enum StateEntry<T, U: StateEnum> {
    State(T),
    Transition(U),
}

pub trait State<'a, T, U>: Sized
where
    T: StateEnum,
    U: StateMachine<'a, T>,
{
    // TODO autogen
    fn enter(state_refs: &U::StateRefs) -> StateEntry<Self, T>;

    fn init(&mut self, state_refs: &U::StateRefs) -> Option<T> {
        None
    }

    fn update(&mut self, state_refs: &U::StateRefs) -> Option<T> {
        None
    }

    fn top_down_update(&mut self, state_refs: &U::StateRefs) -> Option<T> {
        None
    }

    fn exit(self, state_refs: &U::StateRefs) -> Option<T> {
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

impl<'a, T, U, V> State<'a, T, U> for V
where
    T: StateEnum,
    U: StateMachine<'a, T>,
    V: TopState<T>,
{
    fn enter(state_refs: &U::StateRefs) -> StateEntry<Self, T> {
        unreachable!()
    }

    fn init(&mut self, state_refs: &U::StateRefs) -> Option<T> {
        TopState::init(self)
    }

    fn update(&mut self, state_refs: &U::StateRefs) -> Option<T> {
        TopState::update(self)
    }

    fn top_down_update(&mut self, state_refs: &U::StateRefs) -> Option<T> {
        TopState::top_down_update(self)
    }

    fn exit(self, state_refs: &U::StateRefs) -> Option<T> {
        unreachable!()
    }
}

pub mod internal {
    use std::marker::PhantomData;

    use log::info;

    use super::*;

    pub enum TransitionResult<T> {
        Done,
        MoveUp,
        NewTransition(T),
    }

    pub trait SubstateEnum<'a, T: StateEnum, U: StateMachine<'a, T>> {
        fn none_variant() -> Self;

        fn this_state() -> T;

        fn is_state(state: T) -> bool;

        fn current_state(&self) -> T {
            Self::this_state()
        }

        fn is_ancestor(state: T) -> bool {
            false
        }

        fn update(&mut self, state_refs: &U::StateRefs) -> Option<T> {
            None
        }

        fn top_down_update(&mut self, state_refs: &U::StateRefs) -> Option<T> {
            None
        }

        fn exit(&mut self, state_refs: &U::StateRefs) -> Option<T> {
            None
        }

        fn transition(&mut self, target: T, state_refs: &U::StateRefs) -> TransitionResult<T> {
            TransitionResult::MoveUp
        }

        fn enter_substate_towards(&mut self, target: T, state_refs: &U::StateRefs) -> Option<T> {
            unreachable!()
        }

        fn state_matches(&self, state: T) -> bool {
            Self::is_state(state)
        }
    }

    pub enum NodeEntry<'a, T, U, V, W>
    where
        T: StateEnum,
        U: StateMachine<'a, T>,
        V: State<'a, T, U>,
        W: SubstateEnum<'a, T, U>,
    {
        Node(Node<'a, T, U, V, W>),
        Transition(T, PhantomData<&'a ()>),
    }

    pub struct Node<'a, T, U, V, W>
    where
        T: StateEnum,
        U: StateMachine<'a, T>,
        V: State<'a, T, U>,
        W: SubstateEnum<'a, T, U>,
    {
        pub state: V,
        pub substate: W,
        phantom_a: PhantomData<&'a ()>,
        phantom_t: PhantomData<T>,
        phantom_u: PhantomData<U>,
    }

    impl<'a, T, U, V, W> Node<'a, T, U, V, W>
    where
        T: StateEnum,
        U: StateMachine<'a, T>,
        V: State<'a, T, U>,
        W: SubstateEnum<'a, T, U>,
    {
        pub fn from_state(state: V) -> Self {
            Self {
                state,
                substate: W::none_variant(),
                phantom_a: PhantomData,
                phantom_t: PhantomData,
                phantom_u: PhantomData,
            }
        }

        pub fn enter(state_refs: &U::StateRefs) -> NodeEntry<'a, T, U, V, W> {
            info!("\u{02502}Entering {:?}", W::this_state());
            match V::enter(state_refs) {
                StateEntry::State(state) => NodeEntry::Node(Self {
                    state,
                    substate: W::none_variant(),
                    phantom_a: PhantomData,
                    phantom_t: PhantomData,
                    phantom_u: PhantomData,
                }),
                StateEntry::Transition(target) => {
                    info!("\u{02502}Short circuit transition to {target:?}");
                    NodeEntry::Transition(target, PhantomData)
                }
            }
        }

        pub fn update(&mut self, state_refs: &U::StateRefs) -> Option<T> {
            match self.substate.update(&state_refs) {
                Some(target) => Some(target),
                None => {
                    info!("\u{02502}Updating {:?}", W::this_state());
                    self.state.update(&state_refs)
                }
            }
        }

        pub fn top_down_update(&mut self, state_refs: &U::StateRefs) -> Option<T> {
            info!("\u{02502}Top-down updating {:?}", W::this_state());
            match self.state.top_down_update(&state_refs) {
                Some(target) => Some(target),
                None => self.substate.top_down_update(&state_refs),
            }
        }

        pub fn exit(self, state_refs: &U::StateRefs) -> Option<T> {
            info!("\u{02502}Exiting {:?}", W::this_state());
            self.state.exit(&state_refs).inspect(|target| {
                info!("\u{02502}Short circuit transition to {target:?}");
            })
        }

        pub fn transition(&mut self, target: T, state_refs: &U::StateRefs) -> TransitionResult<T> {
            // try to transition the current substate towards the target state
            match self.substate.transition(target, &state_refs) {
                // substate is the target state
                TransitionResult::Done => TransitionResult::Done,

                // substate is not the target state or an ancestor of it
                TransitionResult::MoveUp => {
                    if let Some(new_target) = self.substate.exit(&state_refs) {
                        // substate exit resulted in a short circuit transition
                        self.transition(new_target, &state_refs)
                    } else if W::is_ancestor(target) {
                        if let Some(new_target) =
                            self.substate.enter_substate_towards(target, &state_refs)
                        {
                            // substate transition resulted in a short circuit transition
                            TransitionResult::NewTransition(new_target)
                        } else {
                            // substate successfully moved towards target state,
                            // continue transitioning downwards
                            self.substate.transition(target, &state_refs)
                        }
                    } else if W::is_state(target) {
                        // this state is the target
                        match self.state.init(&state_refs) {
                            None => TransitionResult::Done,
                            Some(new_target) => {
                                info!("\u{02502}Initial transition to {new_target:?}");
                                TransitionResult::NewTransition(new_target)
                            }
                        }
                    } else {
                        // this state is not the target state or an ancestor of it
                        TransitionResult::MoveUp
                    }
                }

                // substate transition resulted in a short circuit transition
                // bubble back up to top
                TransitionResult::NewTransition(new_target) => {
                    TransitionResult::NewTransition(new_target)
                }
            }
        }

        pub fn current_state(&self) -> T {
            self.substate.current_state()
        }

        pub fn state_matches(&self, state: T) -> bool {
            self.substate.state_matches(state)
        }
    }

    pub struct TopNode<'a, T, U, V, W>
    where
        T: StateEnum,
        U: StateMachine<'a, T>,
        V: TopState<T>,
        W: SubstateEnum<'a, T, U>,
    {
        pub node: Node<'a, T, U, V, W>,
        name: String,
        phantom: PhantomData<&'a ()>,
    }

    impl<'a, T, U, V, W> TopNode<'a, T, U, V, W>
    where
        T: StateEnum,
        U: StateMachine<'a, T>,
        V: TopState<T>,
        W: SubstateEnum<'a, T, U>,
    {
        pub fn new(mut top_state: V, name: String) -> Self {
            Self {
                node: Node::from_state(top_state),
                name,
                phantom: PhantomData,
            }
        }

        pub fn init(&mut self) {
            if let Some(target) = TopState::init(&mut self.node.state) {
                info!("{}: Initial transition to {target:?}", self.name);
                self.transition_quiet(target);
                info!("\u{02502}Transition complete");
            }
        }

        pub fn update(&mut self) {
            info!("{}: Updating", self.name);
            //if let Some(target) = self.node.update() {
            //    self.transition(target);
            //}
        }

        pub fn top_down_update(&mut self) {
            info!("{}: Top-down updating", self.name);
            //if let Some(target) = self.node.top_down_update() {
            //    self.transition(target);
            //}
        }

        pub fn transition_quiet(&mut self, target: T) {
            //match self.node.transition(target) {
            //    TransitionResult::Done => return,
            //    TransitionResult::MoveUp => unreachable!(),
            //    TransitionResult::NewTransition(new_target) => self.transition_quiet(new_target),
            //}
        }

        pub fn transition(&mut self, target: T) {
            info!(
                "{}: Transitioning from {:?} to {target:?}",
                self.name(),
                self.state()
            );
            self.transition_quiet(target);
            info!("\u{02502}Transition complete");
        }

        pub fn state(&self) -> T {
            self.node.current_state()
        }

        pub fn name(&self) -> &str {
            &self.name
        }

        pub fn set_name(&mut self, name: String) {
            self.name = name;
        }

        pub fn state_matches(&self, state: T) -> bool {
            self.node.state_matches(state)
        }
    }
}
