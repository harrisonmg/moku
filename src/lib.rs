#![allow(unused)]

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
    V: TopState<T, U>,
{
    fn new(top_state: V) -> Self;
    fn name(self, name: &str) -> Self;
    fn build(self) -> U;
}

pub trait StateRef<T, U, V>
where
    T: StateEnum,
    U: StateMachine<T>,
    V: State<T, U>,
{
    fn get_ref(&self) -> Option<&U>;
    fn get_ref_mut(&mut self) -> Option<&mut U>;
}

pub enum StateEntry<T, U: StateEnum> {
    State(T),
    Transition(U),
}

pub trait State<T, U>: Sized
where
    T: StateEnum,
    U: StateMachine<T>,
{
    // TODO autogen
    fn enter(machine: &mut U) -> StateEntry<Self, T>;

    fn init(&mut self, machine: &mut U) -> Option<T> {
        None
    }

    fn update(&mut self, machine: &mut U) -> Option<T> {
        None
    }

    fn top_down_update(&mut self, machine: &mut U) -> Option<T> {
        None
    }

    fn exit(self, machine: &mut U) -> Option<T> {
        None
    }
}

pub trait TopState<T, U>: Sized
where
    T: StateEnum,
    U: StateMachine<T>,
{
    fn init(&mut self, machine: &mut U) -> Option<T> {
        None
    }

    fn update(&mut self, machine: &mut U) -> Option<T> {
        None
    }

    fn top_down_update(&mut self, machine: &mut U) -> Option<T> {
        None
    }
}

impl<T, U, V> State<T, U> for V
where
    T: StateEnum,
    U: StateMachine<T>,
    V: TopState<T, U>,
{
    fn enter() -> StateEntry<T, Self> {
        unreachable!()
    }

    fn init(&mut self, machine: &mut U) -> Option<T> {
        TopState::init(self)
    }

    fn update(&mut self, machine: &mut U) -> Option<T> {
        TopState::update(self, machine)
    }

    fn top_down_update(&mut self, machine: &mut U) -> Option<T> {
        TopState::top_down_update(self, machine)
    }

    fn exit(self, machine: &mut U) -> Option<T> {
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

    pub trait SubstateEnum<T: StateEnum, U: StateMachine<T>> {
        fn none_variant() -> Self;

        fn this_state() -> T;

        fn is_state(state: T) -> bool;

        fn current_state(&self) -> T {
            Self::this_state()
        }

        fn is_ancestor(state: T) -> bool {
            false
        }

        fn update(&mut self, machine: &mut U) -> Option<T> {
            None
        }

        fn top_down_update(&mut self, machine: &mut U) -> Option<T> {
            None
        }

        fn exit(&mut self, machine: &mut U) -> Option<T> {
            None
        }

        fn transition(&mut self, target: T, machine: &mut U) -> TransitionResult<T> {
            TransitionResult::MoveUp
        }

        fn enter_substate_towards(&mut self, target: T, machine: &mut U) -> Option<T> {
            unreachable!()
        }

        fn state_matches(&self, state: T) -> bool {
            Self::is_state(state)
        }
    }

    pub enum NodeEntry<T, U, V, W>
    where
        T: StateEnum,
        U: StateMachine<T>,
        V: State<T, U>,
        W: SubstateEnum<T, U>,
    {
        Node(Node<T, U, V, W>),
        Transition(T),
    }

    pub struct Node<T, U, V, W>
    where
        T: StateEnum,
        U: StateMachine<T>,
        V: State<T, U>,
        W: SubstateEnum<T, U>,
    {
        pub state: V,
        pub substate: W,
        phantom_t: PhantomData<T>,
        phantom_u: PhantomData<U>,
    }

    impl<T, U, V, W> Node<T, U, V, W>
    where
        T: StateEnum,
        U: StateMachine<T>,
        V: State<T, U>,
        W: SubstateEnum<T, U>,
    {
        pub fn from_state(state: V) -> Self {
            Self {
                state,
                substate: W::none_variant(),
                phantom_t: PhantomData,
                phantom_u: PhantomData,
            }
        }

        pub fn enter(machine: &mut U) -> NodeEntry<T, U, V, W> {
            info!("\u{02502}Entering {:?}", W::this_state());
            match V::enter(machine) {
                StateEntry::State(state) => NodeEntry::Node(Self {
                    state,
                    substate: W::none_variant(),
                    phantom_t: PhantomData,
                    phantom_u: PhantomData,
                }),
                StateEntry::Transition(target) => {
                    info!("\u{02502}Short circuit transition to {target:?}");
                    NodeEntry::Transition(target)
                }
            }
        }

        pub fn update(&mut self, machine: &mut U) -> Option<T> {
            match self.substate.update(machine) {
                Some(target) => Some(target),
                None => {
                    info!("\u{02502}Updating {:?}", W::this_state());
                    self.state.update(machine)
                }
            }
        }

        pub fn top_down_update(&mut self, machine: &mut U) -> Option<T> {
            info!("\u{02502}Top-down updating {:?}", W::this_state());
            match self.state.top_down_update(machine) {
                Some(target) => Some(target),
                None => self.substate.top_down_update(machine),
            }
        }

        pub fn exit(self, machine: &mut U) -> Option<T> {
            info!("\u{02502}Exiting {:?}", W::this_state());
            self.state.exit(machine).inspect(|target| {
                info!("\u{02502}Short circuit transition to {target:?}");
            })
        }

        pub fn transition(&mut self, target: T, machine: &mut U) -> TransitionResult<T> {
            // try to transition the current substate towards the target state
            match self.substate.transition(target, machine) {
                // substate is the target state
                TransitionResult::Done => TransitionResult::Done,

                // substate is not the target state or an ancestor of it
                TransitionResult::MoveUp => {
                    if let Some(new_target) = self.substate.exit(machine) {
                        // substate exit resulted in a short circuit transition
                        self.transition(new_target, machine)
                    } else if W::is_ancestor(target) {
                        if let Some(new_target) =
                            self.substate.enter_substate_towards(target, machine)
                        {
                            // substate transition resulted in a short circuit transition
                            TransitionResult::NewTransition(new_target)
                        } else {
                            // substate successfully moved towards target state,
                            // continue transitioning downwards
                            self.substate.transition(target, machine)
                        }
                    } else if W::is_state(target) {
                        // this state is the target
                        match self.state.init(machine) {
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

    pub struct TopNode<T, U, V, W>
    where
        T: StateEnum,
        U: StateMachine<T>,
        V: TopState<T, U>,
        W: SubstateEnum<T, U>,
    {
        pub node: Node<T, U, V, W>,
        name: String,
    }

    impl<T, U, V, W> TopNode<T, U, V, W>
    where
        T: StateEnum,
        U: StateMachine<T>,
        V: TopState<T, U>,
        W: SubstateEnum<T, U>,
    {
        pub fn new(mut top_state: V, name: String) -> Self {
            Self {
                node: Node::from_state(top_state),
                name,
            }
        }

        pub fn init(&mut self, machine: &mut U) {
            if let Some(target) = TopState::init(&mut self.node.state, machine) {
                info!("{}: Initial transition to {target:?}", self.name);
                self.transition_quiet(target, machine);
                info!("\u{02502}Transition complete");
            }
        }

        pub fn update(&mut self, machine: &mut U) {
            info!("{}: Updating", self.name);
            if let Some(target) = self.node.update(machine) {
                self.transition(target, machine);
            }
        }

        pub fn top_down_update(&mut self, machine: &mut U) {
            info!("{}: Top-down updating", self.name);
            if let Some(target) = self.node.top_down_update(machine) {
                self.transition(target, machine);
            }
        }

        pub fn transition_quiet(&mut self, target: T, machine: &mut U) {
            match self.node.transition(target, machine) {
                TransitionResult::Done => return,
                TransitionResult::MoveUp => unreachable!(),
                TransitionResult::NewTransition(new_target) => {
                    self.transition_quiet(new_target, machine)
                }
            }
        }

        pub fn transition(&mut self, target: T, machine: &mut U) {
            info!(
                "{}: Transitioning from {:?} to {target:?}",
                self.name(),
                self.state()
            );
            self.transition_quiet(target, machine);
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
