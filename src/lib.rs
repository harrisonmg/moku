#![allow(unused)]

use std::marker::PhantomData;

mod output;

pub trait StateEnum: std::fmt::Debug + Copy {}

pub trait StateMachine<T: StateEnum> {
    fn update(&mut self);
    fn top_down_update(&mut self);
    fn transition(&mut self, target: T);
    fn state(&self) -> T;
    fn name(&self) -> &str;
    fn set_name(&mut self, name: String);
    fn state_matches(&self, state: T) -> bool;
}

pub trait StateMachineBuilder<T, U, V>
where
    T: StateEnum,
    U: StateMachine<T>,
    V: TopState<T>,
{
    fn new(top_state: V) -> Self;
    fn name(self, name: &str) -> Self;
    fn build(self) -> U;
}

pub enum StateEntry<T, U: StateEnum> {
    State(T),
    Transition(U),
}

pub trait State<T: StateEnum>: Sized {
    // TODO autogen
    type Superstates<'a>;

    // TODO autogen
    fn enter<'a>(superstates: &mut Self::Superstates<'a>) -> StateEntry<Self, T>;

    fn init<'a>(&mut self, superstates: &mut Self::Superstates<'a>) -> Option<T> {
        None
    }

    fn update<'a>(&mut self, superstates: &mut Self::Superstates<'a>) -> Option<T> {
        None
    }

    fn top_down_update<'a>(&mut self, superstates: &mut Self::Superstates<'a>) -> Option<T> {
        None
    }

    fn exit<'a>(self, superstates: &mut Self::Superstates<'a>) -> Option<T> {
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

pub struct NoSuperstates<'a>(PhantomData<&'a ()>);

impl<T: StateEnum, U: TopState<T>> State<T> for U {
    type Superstates<'a> = NoSuperstates<'a>;

    fn enter<'a>(superstates: &mut Self::Superstates<'a>) -> StateEntry<Self, T> {
        unreachable!()
    }

    fn init<'a>(&mut self, superstates: &mut Self::Superstates<'a>) -> Option<T> {
        TopState::init(self)
    }

    fn update<'a>(&mut self, superstates: &mut Self::Superstates<'a>) -> Option<T> {
        TopState::update(self)
    }

    fn top_down_update<'a>(&mut self, superstates: &mut Self::Superstates<'a>) -> Option<T> {
        TopState::top_down_update(self)
    }

    fn exit<'a>(self, superstates: &mut Self::Superstates<'a>) -> Option<T> {
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

    pub trait SubstateEnum<T: StateEnum, U: State<T>> {
        fn none_variant() -> Self;

        fn this_state() -> T;

        fn is_state(state: T) -> bool;

        fn current_state(&self) -> T {
            Self::this_state()
        }

        fn is_ancestor(state: T) -> bool {
            false
        }

        fn update<'a>(&mut self, state: &mut U, superstates: &mut U::Superstates<'a>) -> Option<T> {
            None
        }

        fn top_down_update<'a>(
            &mut self,
            state: &mut U,
            superstates: &mut U::Superstates<'a>,
        ) -> Option<T> {
            None
        }

        fn exit<'a>(&mut self, state: &mut U, superstates: &mut U::Superstates<'a>) -> Option<T> {
            None
        }

        fn transition<'a>(
            &mut self,
            target: T,
            state: &mut U,
            superstates: &mut U::Superstates<'a>,
        ) -> TransitionResult<T> {
            TransitionResult::MoveUp
        }

        fn enter_substate_towards<'a>(
            &mut self,
            target: T,
            state: &mut U,
            superstates: &mut U::Superstates<'a>,
        ) -> Option<T> {
            unreachable!()
        }

        fn state_matches(&self, state: T) -> bool {
            Self::is_state(state)
        }
    }

    pub enum NodeEntry<T, U, V>
    where
        T: StateEnum,
        U: State<T>,
        V: SubstateEnum<T, U>,
    {
        Node(Node<T, U, V>),
        Transition(T),
    }

    pub struct Node<T, U, V>
    where
        T: StateEnum,
        U: State<T>,
        V: SubstateEnum<T, U>,
    {
        pub state: U,
        pub substate: V,
        phantom: PhantomData<T>,
    }

    impl<T, U, V> Node<T, U, V>
    where
        T: StateEnum,
        U: State<T>,
        V: SubstateEnum<T, U>,
    {
        pub fn from_state(state: U) -> Self {
            Self {
                state,
                substate: V::none_variant(),
                phantom: PhantomData,
            }
        }

        pub fn enter<'a>(superstates: &mut U::Superstates<'a>) -> NodeEntry<T, U, V> {
            info!("\u{02502}Entering {:?}", V::this_state());
            match U::enter(superstates) {
                StateEntry::State(state) => NodeEntry::Node(Self {
                    state,
                    substate: V::none_variant(),
                    phantom: PhantomData,
                }),
                StateEntry::Transition(target) => {
                    info!("\u{02502}Short circuit transition to {target:?}");
                    NodeEntry::Transition(target)
                }
            }
        }

        pub fn update<'a>(&mut self, superstates: &mut U::Superstates<'a>) -> Option<T> {
            match self.substate.update(&mut self.state, superstates) {
                Some(target) => Some(target),
                None => {
                    info!("\u{02502}Updating {:?}", V::this_state());
                    self.state.update(superstates)
                }
            }
        }

        pub fn top_down_update<'a>(&mut self, superstates: &mut U::Superstates<'a>) -> Option<T> {
            info!("\u{02502}Top-down updating {:?}", V::this_state());
            match self.state.top_down_update(superstates) {
                Some(target) => Some(target),
                None => self.substate.top_down_update(&mut self.state, superstates),
            }
        }

        pub fn exit<'a>(self, superstates: &mut U::Superstates<'a>) -> Option<T> {
            info!("\u{02502}Exiting {:?}", V::this_state());
            self.state.exit(superstates).inspect(|target| {
                info!("\u{02502}Short circuit transition to {target:?}");
            })
        }

        pub fn transition<'a>(
            &mut self,
            target: T,
            superstates: &mut U::Superstates<'a>,
        ) -> TransitionResult<T> {
            // try to transition the current substate towards the target state
            match self
                .substate
                .transition(target, &mut self.state, superstates)
            {
                // substate is the target state
                TransitionResult::Done => TransitionResult::Done,

                // substate is not the target state or an ancestor of it
                TransitionResult::MoveUp => {
                    if let Some(new_target) = self.substate.exit(&mut self.state, superstates) {
                        // substate exit resulted in a short circuit transition
                        self.transition(new_target, superstates)
                    } else if V::is_ancestor(target) {
                        if let Some(new_target) = self.substate.enter_substate_towards(
                            target,
                            &mut self.state,
                            superstates,
                        ) {
                            // substate transition resulted in a short circuit transition
                            TransitionResult::NewTransition(new_target)
                        } else {
                            // substate successfully moved towards target state,
                            // continue transitioning downwards
                            self.substate
                                .transition(target, &mut self.state, superstates)
                        }
                    } else if V::is_state(target) {
                        // this state is the target
                        match self.state.init(superstates) {
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

    pub struct TopNode<T, U, V>
    where
        T: StateEnum,
        U: TopState<T>,
        V: SubstateEnum<T, U>,
    {
        pub node: Node<T, U, V>,
        name: String,
    }

    impl<T, U, V> TopNode<T, U, V>
    where
        T: StateEnum,
        U: TopState<T>,
        V: SubstateEnum<T, U>,
    {
        pub fn new(mut top_state: U, name: String) -> Self {
            Self {
                node: Node::from_state(top_state),
                name,
            }
        }

        pub fn init(&mut self) {
            if let Some(target) = TopState::init(&mut self.node.state) {
                info!("{}: Initial transition to {target:?}", self.name);
                self.transition_quiet(target);
                info!("\u{02502}Transition complete");
            }
        }

        pub fn update<'a>(&mut self) {
            info!("{}: Updating", self.name);
            if let Some(target) = self.node.update(&mut NoSuperstates(PhantomData)) {
                self.transition(target);
            }
        }

        pub fn top_down_update(&mut self) {
            info!("{}: Top-down updating", self.name);
            if let Some(target) = self.node.top_down_update(&mut NoSuperstates(PhantomData)) {
                self.transition(target);
            }
        }

        pub fn transition_quiet(&mut self, target: T) {
            match self
                .node
                .transition(target, &mut NoSuperstates(PhantomData))
            {
                TransitionResult::Done => return,
                TransitionResult::MoveUp => unreachable!(),
                TransitionResult::NewTransition(new_target) => self.transition_quiet(new_target),
            }
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
