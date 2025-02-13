//#![warn(missing_docs)]
#![doc = include_str!("../README.md")]
#![cfg_attr(not(feature = "std"), no_std)]

use core::marker::PhantomData;

/// Designates a module for state machine generation.
///
/// Moku expects the following items directly inside of the attributed module:
/// - an empty module attributed with [machine_module]
/// - exactly one implementation of [TopState]
/// - any number of implementations of [State]
///
/// ## Example
/// ```
/// #[moku::state_machine]
/// mod blinky {
///     use moku::*;
///
///     #[machine_module]
///     mod machine {}
///
///     use machine::BlinkyState;
///
///     struct Top;
///     impl TopState<BlinkyState> for Top {}
/// }
/// ```
///
/// ## Arguments
/// A single argument may be used to manually set the prefix of the public items generated inside
/// of the [machine_module]:
/// ```
/// #[moku::state_machine(Foo)]
/// mod blinky {
///     use moku::*;
///
///     #[machine_module]
///     mod machine {}
///
///     use machine::FooState;
///
///     struct Top;
///     impl TopState<FooState> for Top {}
/// }
/// ```
pub use moku_macros::state_machine;

/// Designates a module to be populated with the autogenerated code of a state machine.
///
/// The module must be empty and reside directly inside a module attributed with [state_machine].
///
/// ## Example
/// ```
/// #[moku::state_machine]
/// mod blinky {
///     use moku::*;
///
///     #[machine_module]
///     mod machine {}
///
///     use machine::BlinkyState;
///
///     struct Top;
///     impl TopState<BlinkyState> for Top {}
/// }
/// ```
pub use moku_macros::machine_module;

/// Designates implementations of [State] with an argument indicating their superstate.
///
/// The implementations must reside directly inside a module attributed with [state_machine].
///
/// The argument must be the name of an implementor of either [TopState] or [State].
///
/// ## Example
/// ```
/// #[moku::state_machine]
/// mod example {
///     use moku::*;
///
///     #[machine_module]
///     mod machine {}
///
///     use machine::ExampleState;
///
///     struct Top;
///     impl TopState<ExampleState> for Top {}
///
///     struct Foo;
///
///     #[superstate(Top)]
///     impl State<ExampleState> for Foo {}
///
///     struct Bar;
///
///     #[superstate(Foo)]
///     impl State<ExampleState> for Bar {}
/// }
/// ```
pub use moku_macros::superstate;

/// A flat list of all states in a state machine.
pub trait StateEnum: core::fmt::Debug + Copy {}

/// A state machine.
pub trait StateMachine<T: StateEnum, U: TopState<T>> {
    /// Update the state machine.
    ///
    /// Starting with the deepest state, calls [State::update] (or [TopState::update]).
    ///
    /// If any state returns `Some(state)` from its update method, that transition will be
    /// completed and the state machine will continue updating states starting from the nearest
    /// common ancestor of the previous state and the new state after transition.
    ///
    /// TODO: Actually implement that.
    ///
    /// # Example
    /// For some machine:
    /// ```text
    /// Top
    /// ├─ Foo
    /// │  └─ Bar
    /// └─ Fizz
    ///    └─ Buzz
    /// ```
    /// If the [State::update] method of the `Bar` state will return `Some(ExampleState::Buzz)`,
    /// then:
    /// ```
    /// # #[moku::state_machine]
    /// # mod example {
    /// #     use moku::*;
    /// #
    /// #     #[machine_module]
    /// #     pub mod machine {}
    /// #
    /// #     pub use machine::ExampleState;
    /// #
    /// #     pub struct Top;
    /// #     impl TopState<ExampleState> for Top {}
    /// #
    /// #     struct Foo;
    /// #     #[superstate(Top)]
    /// #     impl State<ExampleState> for Foo {}
    /// #
    /// #     struct Bar;
    /// #     #[superstate(Foo)]
    /// #     impl State<ExampleState> for Bar {}
    /// #
    /// #     struct Fizz;
    /// #     #[superstate(Top)]
    /// #     impl State<ExampleState> for Fizz {}
    /// #
    /// #     struct Buzz;
    /// #     #[superstate(Fizz)]
    /// #     impl State<ExampleState> for Buzz {}
    /// # }
    /// # use moku::{StateMachine, StateMachineBuilder};
    /// # use example::ExampleState;
    /// # let mut machine = example::machine::ExampleMachineBuilder::new(example::Top).build();
    /// # machine.transition(ExampleState::Bar);
    /// assert!(matches!(machine.state(), ExampleState::Bar));
    /// machine.update();
    /// ```
    /// Will have the log output of:
    /// ```text
    /// Example: Updating
    /// │Updating Bar
    /// Example: Transitioning from Bar to Buzz
    /// │Exiting Bar
    /// │Exiting Foo
    /// │Entering Fizz
    /// │Entering Buzz
    /// └Transition complete
    /// │Updating Top
    /// └Update complete
    /// ```
    /// `Top` being the nearest common ancestor of the starting state, `Bar`, and the new state,
    /// `Buzz`, so the update continues from `Top`.
    fn update(&mut self);

    /// Top-down update the state machine.
    ///
    /// Starting with the [TopState], calls [State::update] (or [TopState::update]).
    ///
    /// If any state returns `Some(state)` from its update method, that transition will be
    /// completed and the state machine will continue updating states starting from the first
    /// active descendent of the nearest common ancestor of the previous state and the new state
    /// after transition.
    ///
    /// TODO: Actually implement that.
    ///
    /// # Example
    /// For some machine:
    /// ```text
    /// Top
    /// ├─ Foo
    /// │  └─ Bar
    /// └─ Fizz
    ///    └─ Buzz
    /// ```
    /// If the [State::top_down_update] method of the `Foo` state will return `Some(ExampleState::Fizz)`,
    /// then:
    /// ```
    /// # #[moku::state_machine]
    /// # mod example {
    /// #     use moku::*;
    /// #
    /// #     #[machine_module]
    /// #     pub mod machine {}
    /// #
    /// #     pub use machine::ExampleState;
    /// #
    /// #     pub struct Top;
    /// #     impl TopState<ExampleState> for Top {}
    /// #
    /// #     struct Foo;
    /// #     #[superstate(Top)]
    /// #     impl State<ExampleState> for Foo {}
    /// #
    /// #     struct Bar;
    /// #     #[superstate(Foo)]
    /// #     impl State<ExampleState> for Bar {}
    /// #
    /// #     struct Fizz;
    /// #     #[superstate(Top)]
    /// #     impl State<ExampleState> for Fizz {}
    /// #
    /// #     struct Buzz;
    /// #     #[superstate(Fizz)]
    /// #     impl State<ExampleState> for Buzz {}
    /// # }
    /// # use moku::{StateMachine, StateMachineBuilder};
    /// # use example::ExampleState;
    /// # let mut machine = example::machine::ExampleMachineBuilder::new(example::Top).build();
    /// # machine.transition(ExampleState::Foo);
    /// assert!(matches!(machine.state(), ExampleState::Foo));
    /// machine.top_down_update();
    /// ```
    /// Will have the log output of:
    /// ```text
    /// Example: Top-down updating
    /// │Updating Top
    /// │Updating Foo
    /// Example: Transitioning from Foo to Fizz
    /// │Exiting Foo
    /// │Entering Fizz
    /// └Transition complete
    /// │Updating Fizz
    /// └Update complete
    /// ```
    /// `Top` being the nearest common ancestor of the starting state, `Foo`, and the new state,
    /// `Fizz`, so the top-down update continues from the first acitve descendent of `Top`: `Fizz`.
    fn top_down_update(&mut self);

    /// Attempt to transition the [StateMachine] to the target state.
    ///
    /// Subject to short-circuit transtions (from [State::enter] or [State::exit]) and initial
    /// transitions (from [State::init] or [TopState::init]).
    fn transition(&mut self, target: T);

    /// Get the current state of the [StateMachine].
    ///
    /// Returns the deepest active state.
    /// # Example
    /// For some machine:
    /// ```text
    /// Top
    /// └─ Foo
    ///    └─ Bar
    /// ```
    /// ```
    /// # #[moku::state_machine]
    /// # mod example {
    /// #     use moku::*;
    /// #
    /// #     #[machine_module]
    /// #     pub mod machine {}
    /// #
    /// #     pub use machine::ExampleState;
    /// #
    /// #     pub struct Top;
    /// #     impl TopState<ExampleState> for Top {}
    /// #
    /// #     struct Foo;
    /// #     #[superstate(Top)]
    /// #     impl State<ExampleState> for Foo {}
    /// #
    /// #     struct Bar;
    /// #     #[superstate(Foo)]
    /// #     impl State<ExampleState> for Bar {}
    /// # }
    /// # use moku::{StateMachine, StateMachineBuilder};
    /// # use example::ExampleState;
    /// # let mut machine = example::machine::ExampleMachineBuilder::new(example::Top).build();
    /// machine.transition(ExampleState::Bar);
    /// assert!(matches!(machine.state(), ExampleState::Bar));
    /// ```
    fn state(&self) -> T;

    /// Check if a given state matches the current state of this [StateMachine] or any active
    /// superstate.
    ///
    /// # Example
    /// For some machine:
    /// ```text
    /// Top
    /// ├─ Foo
    /// │  └─ Bar
    /// └─ Fizz
    /// ```
    /// ```
    /// # #[moku::state_machine]
    /// # mod example {
    /// #     use moku::*;
    /// #
    /// #     #[machine_module]
    /// #     pub mod machine {}
    /// #
    /// #     pub use machine::ExampleState;
    /// #
    /// #     pub struct Top;
    /// #     impl TopState<ExampleState> for Top {}
    /// #
    /// #     struct Foo;
    /// #     #[superstate(Top)]
    /// #     impl State<ExampleState> for Foo {}
    /// #
    /// #     struct Bar;
    /// #     #[superstate(Foo)]
    /// #     impl State<ExampleState> for Bar {}
    /// #
    /// #     struct Fizz;
    /// #     #[superstate(Top)]
    /// #     impl State<ExampleState> for Fizz {}
    /// # }
    /// # use moku::{StateMachine, StateMachineBuilder};
    /// # use example::ExampleState;
    /// # let mut machine = example::machine::ExampleMachineBuilder::new(example::Top).build();
    /// machine.transition(ExampleState::Bar);
    /// assert!(machine.state_matches(ExampleState::Top));
    /// assert!(machine.state_matches(ExampleState::Foo));
    /// assert!(machine.state_matches(ExampleState::Bar));
    /// assert!(!machine.state_matches(ExampleState::Fizz));
    /// ```
    fn state_matches(&self, state: T) -> bool;

    fn top_ref(&self) -> &U;

    fn top_mut(&mut self) -> &mut U;

    /// Get the name of this [StateMachine] instance.
    ///
    /// This name is used in moku log messages.
    fn name(&self) -> &str;

    /// Set the name of this [StateMachine] instance.
    ///
    /// This name is used in moku log messages.
    #[cfg(feature = "std")]
    fn set_name(&mut self, name: String);
}

pub trait StateRef<T: StateEnum, U: State<T>> {
    fn state_ref(&self) -> Option<&U>;
    fn state_mut(&mut self) -> Option<&mut U>;
}

pub trait StateMachineBuilder<T, U, V>
where
    T: StateEnum,
    U: TopState<T>,
    V: StateMachine<T, U>,
{
    fn new(top_state: U) -> Self;

    #[cfg(feature = "std")]
    fn name(self, name: String) -> Self;

    fn build(self) -> V;
}

pub enum StateEntry<T, U: StateEnum> {
    State(T),
    Transition(U),
}

pub trait State<T: StateEnum>: Sized {
    type Superstates<'a>;

    fn enter(_superstates: &mut Self::Superstates<'_>) -> StateEntry<Self, T>;

    fn init(&mut self, _superstates: &mut Self::Superstates<'_>) -> Option<T> {
        None
    }

    fn update(&mut self, _superstates: &mut Self::Superstates<'_>) -> Option<T> {
        None
    }

    fn top_down_update(&mut self, _superstates: &mut Self::Superstates<'_>) -> Option<T> {
        None
    }

    fn exit(self, _superstates: &mut Self::Superstates<'_>) -> Option<T> {
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

    fn enter(_superstates: &mut Self::Superstates<'_>) -> StateEntry<Self, T> {
        unreachable!()
    }

    fn init(&mut self, _superstates: &mut Self::Superstates<'_>) -> Option<T> {
        TopState::init(self)
    }

    fn update(&mut self, _superstates: &mut Self::Superstates<'_>) -> Option<T> {
        TopState::update(self)
    }

    fn top_down_update(&mut self, _superstates: &mut Self::Superstates<'_>) -> Option<T> {
        TopState::top_down_update(self)
    }

    fn exit(self, _superstates: &mut Self::Superstates<'_>) -> Option<T> {
        unreachable!()
    }
}

/// Types and traits for autogenerated state machine code.
///
/// The contents of this module is intended to be used only by the code that is generated by moku.
pub mod internal {
    use core::marker::PhantomData;

    use log::info;

    use super::*;

    /// The result of the attempted transition on a branch of the state tree.
    pub enum TransitionResult<T> {
        /// The target state has been reached and the transition is done.
        Done,

        /// The target state does not exist in the branch and the machine must move up the tree
        /// towards the root to find it.
        MoveUp,

        /// A short circuit transition occurred during the transition; this new target should
        /// replace the previous target state.
        NewTransition(T),
    }

    /// The substate of a [State].
    ///
    /// Also aggregates some functionality that would be attributed to the [State].
    pub trait SubstateEnum<T: StateEnum, U: State<T>> {
        /// The variant that represents no substate, i.e. being in exactly this state.
        fn none_variant() -> Self;

        /// The [StateEnum] variant that this [SubstateEnum] represents.
        fn this_state() -> T;

        /// Does this state match exactly a given [StateEnum] variant?
        fn is_state(state: T) -> bool;

        /// The current leaf state of this branch.
        fn current_state(&self) -> T {
            Self::this_state()
        }

        /// Is this state an ancesstor the the given state?
        #[allow(unused_variables)]
        fn is_ancestor(state: T) -> bool {
            false
        }

        /// Update this state and its active descendents.
        #[allow(unused_variables)]
        fn update(&mut self, state: &mut U, superstates: &mut U::Superstates<'_>) -> Option<T> {
            None
        }

        /// Top-down update this state and its active descendents.
        #[allow(unused_variables)]
        fn top_down_update(
            &mut self,
            state: &mut U,
            superstates: &mut U::Superstates<'_>,
        ) -> Option<T> {
            None
        }

        /// Exit this state and its active descendents.
        #[allow(unused_variables)]
        fn exit(&mut self, state: &mut U, superstates: &mut U::Superstates<'_>) -> Option<T> {
            None
        }

        /// Transition this state and its active descendents.
        #[allow(unused_variables)]
        fn transition(
            &mut self,
            target: T,
            state: &mut U,
            superstates: &mut U::Superstates<'_>,
        ) -> TransitionResult<T> {
            TransitionResult::MoveUp
        }

        /// Enter the substate that moves towards a target state.
        ///
        /// Panics if called when the target state is not a descendent of this state.
        #[allow(unused_variables)]
        fn enter_substate_towards(
            &mut self,
            target: T,
            state: &mut U,
            superstates: &mut U::Superstates<'_>,
        ) -> Option<T> {
            unreachable!()
        }

        /// Does this state or any active descendents match a given state?
        fn state_matches(&self, state: T) -> bool {
            Self::is_state(state)
        }
    }

    /// The result of trying to enter a [Node].
    pub enum NodeEntry<T, U, V>
    where
        T: StateEnum,
        U: State<T>,
        V: SubstateEnum<T, U>,
    {
        /// Entry was successful, here is the new [Node].
        Node(Node<T, U, V>),

        /// Entry resulted in a short-circuit transition.
        Transition(T),
    }

    /// A node in the state tree.
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
        /// Make a new [Node] from a [State].
        pub fn from_state(state: U) -> Self {
            Self {
                state,
                substate: V::none_variant(),
                phantom: PhantomData,
            }
        }

        /// Enter this node.
        pub fn enter(superstates: &mut U::Superstates<'_>) -> NodeEntry<T, U, V> {
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

        /// Update this node and its active descendents.
        pub fn update(&mut self, superstates: &mut U::Superstates<'_>) -> Option<T> {
            match self.substate.update(&mut self.state, superstates) {
                Some(target) => Some(target),
                None => {
                    info!("\u{02502}Updating {:?}", V::this_state());
                    self.state.update(superstates)
                }
            }
        }

        /// Top-down update this node and its active descendents.
        pub fn top_down_update(&mut self, superstates: &mut U::Superstates<'_>) -> Option<T> {
            info!("\u{02502}Top-down updating {:?}", V::this_state());
            match self.state.top_down_update(superstates) {
                Some(target) => Some(target),
                None => self.substate.top_down_update(&mut self.state, superstates),
            }
        }

        /// Exit this node and its active descendents.
        pub fn exit(self, superstates: &mut U::Superstates<'_>) -> Option<T> {
            info!("\u{02502}Exiting {:?}", V::this_state());
            self.state.exit(superstates).inspect(|target| {
                info!("\u{02502}Short circuit transition to {target:?}");
            })
        }

        /// Transition this node and its active descendents.
        pub fn transition(
            &mut self,
            target: T,
            superstates: &mut U::Superstates<'_>,
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

        /// Get the current leaf state of this branch.
        pub fn current_state(&self) -> T {
            self.substate.current_state()
        }

        /// Does this node or any active descendents match a given state?
        pub fn state_matches(&self, state: T) -> bool {
            self.substate.state_matches(state)
        }
    }

    /// The root node of a state tree.
    pub struct TopNode<T, U, V>
    where
        T: StateEnum,
        U: TopState<T>,
        V: SubstateEnum<T, U>,
    {
        pub node: Node<T, U, V>,

        #[cfg(feature = "std")]
        name: String,

        #[cfg(not(feature = "std"))]
        name: &'static str,
    }

    impl<T, U, V> TopNode<T, U, V>
    where
        T: StateEnum,
        U: TopState<T>,
        V: SubstateEnum<T, U>,
    {
        /// Make a new [TopNode] from a [TopState] and a machine instance name.
        #[cfg(feature = "std")]
        pub fn new(top_state: U, name: String) -> Self {
            Self {
                node: Node::from_state(top_state),
                name,
            }
        }

        /// Make a new [TopNode] from a [TopState] and a machine instance name.
        #[cfg(not(feature = "std"))]
        pub fn new(top_state: U, name: &'static str) -> Self {
            Self {
                node: Node::from_state(top_state),
                name,
            }
        }

        /// Perform the initial transition of this node.
        pub fn init(&mut self) {
            if let Some(target) = TopState::init(&mut self.node.state) {
                info!("{}: Initial transition to {target:?}", self.name());
                self.transition_quiet(target);
                info!("\u{02514}Transition complete");
            }
        }

        /// Update this node and its active descendents.
        pub fn update(&mut self) {
            info!("{}: Updating", self.name());
            if let Some(target) = self.node.update(&mut NoSuperstates(PhantomData)) {
                self.transition(target);
            }
            info!("\u{02514}Update complete");
        }

        /// Top-down update this node and its active descendents.
        pub fn top_down_update(&mut self) {
            info!("{}: Top-down updating", self.name());
            if let Some(target) = self.node.top_down_update(&mut NoSuperstates(PhantomData)) {
                self.transition(target);
            }
            info!("\u{02514}Top-down update complete");
        }

        /// Transition this node and its active descendents without logging the start and end of
        /// the transition.
        pub fn transition_quiet(&mut self, target: T) {
            match self
                .node
                .transition(target, &mut NoSuperstates(PhantomData))
            {
                TransitionResult::Done => (),
                TransitionResult::MoveUp => unreachable!(),
                TransitionResult::NewTransition(new_target) => self.transition_quiet(new_target),
            }
        }

        /// Transition this node and its active descendents.
        pub fn transition(&mut self, target: T) {
            info!(
                "{}: Transitioning from {:?} to {target:?}",
                self.name(),
                self.state()
            );
            self.transition_quiet(target);
            info!("\u{02514}Transition complete");
        }

        /// Get the current leaf state of this state tree.
        pub fn state(&self) -> T {
            self.node.current_state()
        }

        /// Get the name of this machine instance.
        pub fn name(&self) -> &str {
            #[cfg(feature = "std")]
            return &self.name;

            #[cfg(not(feature = "std"))]
            return self.name;
        }

        /// Set the name of this machine instance.
        #[cfg(feature = "std")]
        pub fn set_name(&mut self, name: String) {
            self.name = name;
        }

        /// Does this node or any active descendents match a given state?
        pub fn state_matches(&self, state: T) -> bool {
            self.node.state_matches(state)
        }
    }
}
