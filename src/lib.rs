#![allow(unused)]

mod output;

pub trait StateEnum: std::fmt::Debug + Copy {}

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

pub trait StateMachine<T: StateEnum, U: TopState<T>> {
    fn from_top_state(top_state: U) -> Self;
    fn from_top_state_with_name(top_state: U, name: &str) -> Self;
    fn update(&mut self);
    fn top_down_update(&mut self);
    fn transition(&mut self, target: T);
    fn state(&self) -> T;
    fn name(&self) -> &str;
    fn set_name(&mut self, name: String);
    fn state_matches(&self, state: T) -> bool;
    //fn state_ref<V>(&self, state: T) -> Option<&V>;
    //fn state_ref_mut<V>(&mut self, state: T) -> Option<&mut V>;
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

    pub trait SubstateEnum<T: StateEnum> {
        fn none_variant() -> Self;

        fn this_state() -> T;

        fn is_state(state: T) -> bool;

        fn current_state(&self) -> T {
            Self::this_state()
        }

        fn is_ancestor(state: T) -> bool {
            false
        }

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

        fn state_matches(&self, state: T) -> bool {
            Self::is_state(state)
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
            info!("\u{02502}Entering {:?}", V::this_state());
            match U::enter() {
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

        pub fn update(&mut self) -> Option<T> {
            match self.substate.update() {
                Some(target) => Some(target),
                None => {
                    info!("\u{02502}Updating {:?}", V::this_state());
                    self.state.update()
                }
            }
        }

        pub fn top_down_update(&mut self) -> Option<T> {
            info!("\u{02502}Top-down updating {:?}", V::this_state());
            match self.state.top_down_update() {
                Some(target) => Some(target),
                None => self.substate.top_down_update(),
            }
        }

        pub fn exit(self) -> Option<T> {
            info!("\u{02502}Exiting {:?}", V::this_state());
            self.state.exit().inspect(|target| {
                info!("\u{02502}Short circuit transition to {target:?}");
            })
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

    pub struct TopNode<T: StateEnum, U: TopState<T>, V: SubstateEnum<T>> {
        node: Node<T, U, V>,
        name: String,
    }

    impl<T: StateEnum, U: TopState<T>, V: SubstateEnum<T>> TopNode<T, U, V> {
        pub fn from_top_state_with_name(mut top_state: U, name: &str) -> Self {
            let initial_transition = TopState::init(&mut top_state);

            let mut new = Self {
                node: Node::from_state(top_state),
                name: name.to_owned(),
            };

            if let Some(target) = initial_transition {
                info!("{}: Initial transition to {target:?}", new.name);
                new.transition_quiet(target);
                info!("\u{02502}Transition complete");
            }

            new
        }

        pub fn update(&mut self) {
            info!("{}: Updating", self.name);
            if let Some(target) = self.node.update() {
                self.transition(target);
            }
        }

        pub fn top_down_update(&mut self) {
            info!("{}: Top-down updating", self.name);
            if let Some(target) = self.node.top_down_update() {
                self.transition(target);
            }
        }

        pub fn transition_quiet(&mut self, target: T) {
            match self.node.transition(target) {
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
