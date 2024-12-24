use crate::internal::*;
use crate::*;

use state_machine::{BlinkyMachineBuilder, BlinkyState, BlinkyStateMachine, BlinkyStateRefs};

struct Top;
impl<'a> TopState<BlinkyState> for Top {
    fn init(&mut self) -> Option<BlinkyState> {
        Some(BlinkyState::Enabled)
    }
}

struct Disabled;
impl<'a> State<'a, BlinkyState, BlinkyStateMachine<'a>> for Disabled {
    fn enter(state_refs: &BlinkyStateRefs<'a>) -> StateEntry<Self, BlinkyState> {
        StateEntry::Transition(BlinkyState::LedOn)
    }
}

struct Enabled;
impl<'a> State<'a, BlinkyState, BlinkyStateMachine<'a>> for Enabled {
    fn enter(state_refs: &BlinkyStateRefs) -> StateEntry<Self, BlinkyState> {
        StateEntry::State(Self {})
    }

    fn init(&mut self, state_refs: &BlinkyStateRefs) -> Option<BlinkyState> {
        Some(BlinkyState::LedOn)
    }
}

struct LedOn;
impl<'a> State<'a, BlinkyState, BlinkyStateMachine<'a>> for LedOn {
    fn enter(state_refs: &BlinkyStateRefs) -> StateEntry<Self, BlinkyState> {
        StateEntry::State(Self {})
    }

    fn top_down_update(&mut self, state_refs: &BlinkyStateRefs) -> Option<BlinkyState> {
        Some(BlinkyState::LedOff)
    }

    fn exit(self, state_refs: &BlinkyStateRefs) -> Option<BlinkyState> {
        Some(BlinkyState::Enabled)
    }
}

struct LedOff;
impl<'a> State<'a, BlinkyState, BlinkyStateMachine<'a>> for LedOff {
    fn enter(state_refs: &BlinkyStateRefs) -> StateEntry<Self, BlinkyState> {
        StateEntry::State(Self {})
    }

    fn update(&mut self, state_refs: &BlinkyStateRefs) -> Option<BlinkyState> {
        Some(BlinkyState::LedOn)
    }
}

// AUTOGEN
mod state_machine {
    use std::{cell::RefCell, marker::PhantomData, rc::Rc};

    use crate as moku;

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum BlinkyState {
        Top,
        Disabled,
        Enabled,
        LedOn,
        LedOff,
    }

    impl moku::StateEnum for BlinkyState {}

    pub struct BlinkyStateRefs<'a> {
        top: Option<&'a mut super::Top>,
        disabled: Option<&'a mut super::Disabled>,
        enabled: Option<&'a mut super::Enabled>,
        led_on: Option<&'a mut super::LedOn>,
        led_off: Option<&'a mut super::LedOff>,
    }

    impl<'a> moku::StateRef<'a, BlinkyState, BlinkyStateMachine<'a>, super::Top>
        for BlinkyStateRefs<'a>
    {
        fn get_ref(&self) -> Option<&super::Top> {
            self.top.as_deref()
        }

        fn get_ref_mut(&mut self) -> Option<&mut super::Top> {
            self.top
        }
    }

    type LedOffNode<'a> = moku::internal::Node<
        'a,
        BlinkyState,
        BlinkyStateMachine<'a>,
        super::LedOff,
        LedOffSubstate,
    >;

    enum LedOffSubstate {
        None,
    }

    impl<'a> moku::internal::SubstateEnum<'a, BlinkyState, BlinkyStateMachine<'a>> for LedOffSubstate {
        fn none_variant() -> Self {
            Self::None
        }

        fn this_state() -> BlinkyState {
            BlinkyState::LedOff
        }

        fn is_state(state: BlinkyState) -> bool {
            matches!(state, BlinkyState::LedOff)
        }
    }

    type LedOnNode<'a> =
        moku::internal::Node<'a, BlinkyState, BlinkyStateMachine<'a>, super::LedOn, LedOnSubstate>;

    enum LedOnSubstate {
        None,
    }

    impl<'a> moku::internal::SubstateEnum<'a, BlinkyState, BlinkyStateMachine<'a>> for LedOnSubstate {
        fn none_variant() -> Self {
            Self::None
        }

        fn this_state() -> BlinkyState {
            BlinkyState::LedOn
        }

        fn is_state(state: BlinkyState) -> bool {
            matches!(state, BlinkyState::LedOn)
        }
    }

    type EnabledNode<'a> = moku::internal::Node<
        'a,
        BlinkyState,
        BlinkyStateMachine<'a>,
        super::Enabled,
        EnabledSubstate<'a>,
    >;

    enum EnabledSubstate<'a> {
        None,
        LedOn(LedOnNode<'a>),
        LedOff(LedOffNode<'a>),
    }

    impl<'a> moku::internal::SubstateEnum<'a, BlinkyState, BlinkyStateMachine<'a>>
        for EnabledSubstate<'a>
    {
        fn none_variant() -> Self {
            Self::None
        }

        fn this_state() -> BlinkyState {
            BlinkyState::Enabled
        }

        fn is_state(state: BlinkyState) -> bool {
            matches!(state, BlinkyState::Enabled)
        }

        fn current_state(&self) -> BlinkyState {
            match self {
                Self::None => BlinkyState::Enabled,
                Self::LedOn(state) => state.current_state(),
                Self::LedOff(state) => state.current_state(),
            }
        }

        fn is_ancestor(state: BlinkyState) -> bool {
            matches!(state, BlinkyState::LedOn | BlinkyState::LedOff)
        }

        fn update(&mut self, state_refs: &BlinkyStateRefs<'a>) -> Option<BlinkyState> {
            match self {
                Self::None => None,
                Self::LedOn(node) => node.update(state_refs),
                Self::LedOff(node) => node.update(state_refs),
            }
        }

        fn top_down_update(&mut self, state_refs: &BlinkyStateRefs<'a>) -> Option<BlinkyState> {
            match self {
                Self::None => None,
                Self::LedOn(node) => node.top_down_update(state_refs),
                Self::LedOff(node) => node.top_down_update(state_refs),
            }
        }

        fn exit(&mut self, state_refs: &BlinkyStateRefs) -> Option<BlinkyState> {
            let old_state = std::mem::replace(self, Self::None);
            match old_state {
                Self::None => None,
                Self::LedOn(node) => node.exit(state_refs),
                Self::LedOff(node) => node.exit(state_refs),
            }
        }

        fn transition(
            &mut self,
            target: BlinkyState,
            state_refs: &BlinkyStateRefs<'a>,
        ) -> moku::internal::TransitionResult<BlinkyState> {
            match self {
                Self::None => moku::internal::TransitionResult::MoveUp,
                Self::LedOn(node) => node.transition(target, state_refs),
                Self::LedOff(node) => node.transition(target, state_refs),
            }
        }

        fn enter_substate_towards(
            &mut self,
            target: BlinkyState,
            state_refs: &BlinkyStateRefs<'a>,
        ) -> Option<BlinkyState> {
            match target {
                BlinkyState::LedOn => match LedOnNode::enter(state_refs) {
                    moku::internal::NodeEntry::Node(node) => {
                        *self = Self::LedOn(node);
                        None
                    }
                    moku::internal::NodeEntry::Transition(new_target, PhantomData) => {
                        Some(new_target)
                    }
                },
                BlinkyState::LedOff => match LedOffNode::enter(state_refs) {
                    moku::internal::NodeEntry::Node(node) => {
                        *self = Self::LedOff(node);
                        None
                    }
                    moku::internal::NodeEntry::Transition(new_target, PhantomData) => {
                        Some(new_target)
                    }
                },
                _ => unreachable!(),
            }
        }

        fn state_matches(&self, state: BlinkyState) -> bool {
            return Self::is_state(state)
                || match self {
                    Self::None => false,
                    Self::LedOn(node) => node.state_matches(state),
                    Self::LedOff(node) => node.state_matches(state),
                };
        }
    }

    type DisabledNode<'a> = moku::internal::Node<
        'a,
        BlinkyState,
        BlinkyStateMachine<'a>,
        super::Disabled,
        DisabledSubstate,
    >;

    enum DisabledSubstate {
        None,
    }

    impl<'a> moku::internal::SubstateEnum<'a, BlinkyState, BlinkyStateMachine<'a>>
        for DisabledSubstate
    {
        fn none_variant() -> Self {
            Self::None
        }

        fn this_state() -> BlinkyState {
            BlinkyState::Disabled
        }

        fn is_state(state: BlinkyState) -> bool {
            matches!(state, BlinkyState::Disabled)
        }
    }

    type TopNode<'a> =
        moku::internal::Node<'a, BlinkyState, BlinkyStateMachine<'a>, super::Top, TopSubstate<'a>>;

    enum TopSubstate<'a> {
        None,
        Enabled(EnabledNode<'a>),
        Disabled(DisabledNode<'a>),
    }

    impl<'a> moku::internal::SubstateEnum<'a, BlinkyState, BlinkyStateMachine<'a>> for TopSubstate<'a> {
        fn none_variant() -> Self {
            Self::None
        }

        fn this_state() -> BlinkyState {
            BlinkyState::Top
        }

        fn is_state(state: BlinkyState) -> bool {
            matches!(state, BlinkyState::Top)
        }

        fn current_state(&self) -> BlinkyState {
            match self {
                Self::None => BlinkyState::Top,
                Self::Enabled(state) => state.current_state(),
                Self::Disabled(state) => state.current_state(),
            }
        }

        fn is_ancestor(state: BlinkyState) -> bool {
            true
        }

        fn update(&mut self, state_refs: &BlinkyStateRefs<'a>) -> Option<BlinkyState> {
            match self {
                Self::None => None,
                Self::Enabled(node) => node.update(state_refs),
                Self::Disabled(node) => node.update(state_refs),
            }
        }

        fn top_down_update(&mut self, state_refs: &BlinkyStateRefs<'a>) -> Option<BlinkyState> {
            match self {
                Self::None => None,
                Self::Enabled(node) => node.top_down_update(state_refs),
                Self::Disabled(node) => node.top_down_update(state_refs),
            }
        }

        fn exit(&mut self, state_refs: &BlinkyStateRefs<'a>) -> Option<BlinkyState> {
            let old_state = std::mem::replace(self, Self::None);
            match old_state {
                Self::None => None,
                Self::Enabled(node) => node.exit(state_refs),
                Self::Disabled(node) => node.exit(state_refs),
            }
        }

        fn transition(
            &mut self,
            target: BlinkyState,
            state_refs: &BlinkyStateRefs<'a>,
        ) -> moku::internal::TransitionResult<BlinkyState> {
            match self {
                Self::None => moku::internal::TransitionResult::MoveUp,
                Self::Enabled(node) => node.transition(target, state_refs),
                Self::Disabled(node) => node.transition(target, state_refs),
            }
        }

        fn enter_substate_towards(
            &mut self,
            target: BlinkyState,
            state_refs: &BlinkyStateRefs<'a>,
        ) -> Option<BlinkyState> {
            match target {
                BlinkyState::Disabled => match DisabledNode::enter(state_refs) {
                    moku::internal::NodeEntry::Node(node) => {
                        *self = Self::Disabled(node);
                        None
                    }
                    moku::internal::NodeEntry::Transition(new_target, PhantomData) => {
                        Some(new_target)
                    }
                },
                BlinkyState::Enabled | BlinkyState::LedOn | BlinkyState::LedOff => {
                    match EnabledNode::enter(state_refs) {
                        moku::internal::NodeEntry::Node(node) => {
                            *self = Self::Enabled(node);
                            None
                        }
                        moku::internal::NodeEntry::Transition(new_target, PhantomData) => {
                            Some(new_target)
                        }
                    }
                }
                _ => unreachable!(),
            }
        }

        fn state_matches(&self, state: BlinkyState) -> bool {
            return Self::is_state(state)
                || match self {
                    Self::None => false,
                    Self::Enabled(node) => node.state_matches(state),
                    Self::Disabled(node) => node.state_matches(state),
                };
        }
    }

    pub struct BlinkyStateMachine<'a> {
        top_node: moku::internal::TopNode<
            'a,
            BlinkyState,
            BlinkyStateMachine<'a>,
            super::Top,
            TopSubstate<'a>,
        >,
    }

    impl<'a> BlinkyStateMachine<'a> {
        fn new(
            top_node: moku::internal::TopNode<
                'a,
                BlinkyState,
                BlinkyStateMachine<'a>,
                super::Top,
                TopSubstate<'a>,
            >,
        ) -> Self {
            let mut new = Self { top_node };
            new.top_node.init();
            new
        }
    }

    impl<'a> moku::StateMachine<'a, BlinkyState> for BlinkyStateMachine<'a> {
        type StateRefs = BlinkyStateRefs<'a>;

        fn update(&mut self) {
            self.top_node.update()
        }

        fn top_down_update(&mut self) {
            self.top_node.top_down_update()
        }

        fn transition(&mut self, target: BlinkyState) {
            self.top_node.transition(target);
        }

        fn state(&self) -> BlinkyState {
            self.top_node.state()
        }

        fn name(&self) -> &str {
            self.top_node.name()
        }

        fn set_name(&mut self, name: String) {
            self.top_node.set_name(name)
        }

        fn state_matches(&self, state: BlinkyState) -> bool {
            self.top_node.state_matches(state)
        }
    }

    pub struct BlinkyMachineBuilder {
        top_state: super::Top,
        name: Option<String>,
    }

    impl<'a> moku::StateMachineBuilder<'a, BlinkyState, BlinkyStateMachine<'a>, super::Top>
        for BlinkyMachineBuilder
    {
        fn new(top_state: super::Top) -> Self {
            Self {
                top_state,
                name: None,
            }
        }

        fn name(mut self, name: &str) -> Self {
            self.name = Some(name.to_owned());
            self
        }

        fn build(self) -> BlinkyStateMachine<'a> {
            BlinkyStateMachine::new(moku::internal::TopNode::new(
                self.top_state,
                self.name.unwrap_or_else(|| String::from("Blinky")),
            ))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_log::test;

    #[test]
    fn basic() {
        let mut machine = BlinkyMachineBuilder::new(Top {}).name("Blinko").build();
        assert_eq!(machine.state(), BlinkyState::LedOn);

        assert!(machine.state_matches(BlinkyState::Top));
        assert!(machine.state_matches(BlinkyState::Enabled));
        assert!(machine.state_matches(BlinkyState::LedOn));
        assert!(!machine.state_matches(BlinkyState::Disabled));
        assert!(!machine.state_matches(BlinkyState::LedOff));

        machine.top_down_update();
        //assert_eq!(machine.state(), BlinkyState::LedOn);

        machine.update();
        //assert_eq!(machine.state(), BlinkyState::LedOff);

        machine.update();
        //assert_eq!(machine.state(), BlinkyState::LedOn);

        machine.transition(BlinkyState::Disabled);
        //assert_eq!(machine.state(), BlinkyState::Disabled);

        machine.transition(BlinkyState::Enabled);
        //assert_eq!(machine.state(), BlinkyState::LedOn);
    }
}
