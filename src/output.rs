use crate::internal::*;
use crate::*;

use state_machine::{BlinkyMachine, BlinkyMachineBuilder, BlinkyState};

struct Top;
impl TopState<BlinkyState, BlinkyMachine> for Top {
    fn init(&mut self, machine: &mut BlinkyMachine) -> Option<BlinkyState> {
        Some(BlinkyState::Enabled)
    }
}

struct Disabled;
impl State<BlinkyState, BlinkyMachine> for Disabled {
    fn enter(machine: &mut BlinkyMachine) -> StateEntry<Self, BlinkyState> {
        StateEntry::Transition(BlinkyState::LedOn)
    }
}

struct Enabled;
impl State<BlinkyState, BlinkyMachine> for Enabled {
    fn enter(machine: &mut BlinkyMachine) -> StateEntry<Self, BlinkyState> {
        StateEntry::State(Self {})
    }

    fn init(&mut self, machine: &mut BlinkyMachine) -> Option<BlinkyState> {
        Some(BlinkyState::LedOn)
    }
}

struct LedOn;
impl State<BlinkyState, BlinkyMachine> for LedOn {
    fn enter(machine: &mut BlinkyMachine) -> StateEntry<Self, BlinkyState> {
        StateEntry::State(Self {})
    }

    fn top_down_update(&mut self, machine: &mut BlinkyMachine) -> Option<BlinkyState> {
        Some(BlinkyState::LedOff)
    }

    fn exit(self, machine: &mut BlinkyMachine) -> Option<BlinkyState> {
        Some(BlinkyState::Enabled)
    }
}

struct LedOff;
impl State<BlinkyState, BlinkyMachine> for LedOff {
    fn enter(machine: &mut BlinkyMachine) -> StateEntry<Self, BlinkyState> {
        StateEntry::State(Self {})
    }

    fn update(&mut self, machine: &mut BlinkyMachine) -> Option<BlinkyState> {
        Some(BlinkyState::LedOn)
    }
}

// AUTOGEN
mod state_machine {
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

    type LedOffNode =
        moku::internal::Node<BlinkyState, BlinkyMachine, super::LedOff, LedOffSubstate>;

    enum LedOffSubstate {
        None,
    }

    impl moku::internal::SubstateEnum<BlinkyState, BlinkyMachine> for LedOffSubstate {
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

    type LedOnNode = moku::internal::Node<BlinkyState, BlinkyMachine, super::LedOn, LedOnSubstate>;

    enum LedOnSubstate {
        None,
    }

    impl moku::internal::SubstateEnum<BlinkyState, BlinkyMachine> for LedOnSubstate {
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

    type EnabledNode =
        moku::internal::Node<BlinkyState, BlinkyMachine, super::Enabled, EnabledSubstate>;

    enum EnabledSubstate {
        None,
        LedOn(LedOnNode),
        LedOff(LedOffNode),
    }

    impl moku::internal::SubstateEnum<BlinkyState, BlinkyMachine> for EnabledSubstate {
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

        fn update(&mut self) -> Option<BlinkyState> {
            match self {
                Self::None => None,
                Self::LedOn(node) => node.update(),
                Self::LedOff(node) => node.update(),
            }
        }

        fn top_down_update(&mut self) -> Option<BlinkyState> {
            match self {
                Self::None => None,
                Self::LedOn(node) => node.top_down_update(),
                Self::LedOff(node) => node.top_down_update(),
            }
        }

        fn exit(&mut self) -> Option<BlinkyState> {
            let old_state = std::mem::replace(self, Self::None);
            match old_state {
                Self::None => None,
                Self::LedOn(node) => node.exit(),
                Self::LedOff(node) => node.exit(),
            }
        }

        fn transition(
            &mut self,
            target: BlinkyState,
        ) -> moku::internal::TransitionResult<BlinkyState> {
            match self {
                Self::None => moku::internal::TransitionResult::MoveUp,
                Self::LedOn(node) => node.transition(target),
                Self::LedOff(node) => node.transition(target),
            }
        }

        fn enter_substate_towards(&mut self, target: BlinkyState) -> Option<BlinkyState> {
            match target {
                BlinkyState::LedOn => match LedOnNode::enter() {
                    moku::internal::NodeEntry::Node(node) => {
                        *self = Self::LedOn(node);
                        None
                    }
                    moku::internal::NodeEntry::Transition(new_target) => Some(new_target),
                },
                BlinkyState::LedOff => match LedOffNode::enter() {
                    moku::internal::NodeEntry::Node(node) => {
                        *self = Self::LedOff(node);
                        None
                    }
                    moku::internal::NodeEntry::Transition(new_target) => Some(new_target),
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

    type DisabledNode =
        moku::internal::Node<BlinkyState, BlinkyMachine, super::Disabled, DisabledSubstate>;

    enum DisabledSubstate {
        None,
    }

    impl moku::internal::SubstateEnum<BlinkyState, BlinkyMachine> for DisabledSubstate {
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

    type TopNode = moku::internal::Node<BlinkyState, BlinkyMachine, super::Top, TopSubstate>;

    enum TopSubstate {
        None,
        Enabled(EnabledNode),
        Disabled(DisabledNode),
    }

    impl moku::internal::SubstateEnum<BlinkyState, BlinkyMachine> for TopSubstate {
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

        fn update(&mut self) -> Option<BlinkyState> {
            match self {
                Self::None => None,
                Self::Enabled(node) => node.update(),
                Self::Disabled(node) => node.update(),
            }
        }

        fn top_down_update(&mut self) -> Option<BlinkyState> {
            match self {
                Self::None => None,
                Self::Enabled(node) => node.top_down_update(),
                Self::Disabled(node) => node.top_down_update(),
            }
        }

        fn exit(&mut self) -> Option<BlinkyState> {
            let old_state = std::mem::replace(self, Self::None);
            match old_state {
                Self::None => None,
                Self::Enabled(node) => node.exit(),
                Self::Disabled(node) => node.exit(),
            }
        }

        fn transition(
            &mut self,
            target: BlinkyState,
        ) -> moku::internal::TransitionResult<BlinkyState> {
            match self {
                Self::None => moku::internal::TransitionResult::MoveUp,
                Self::Enabled(node) => node.transition(target),
                Self::Disabled(node) => node.transition(target),
            }
        }

        fn enter_substate_towards(&mut self, target: BlinkyState) -> Option<BlinkyState> {
            match target {
                BlinkyState::Disabled => match DisabledNode::enter() {
                    moku::internal::NodeEntry::Node(node) => {
                        *self = Self::Disabled(node);
                        None
                    }
                    moku::internal::NodeEntry::Transition(new_target) => Some(new_target),
                },
                BlinkyState::Enabled | BlinkyState::LedOn | BlinkyState::LedOff => {
                    match EnabledNode::enter() {
                        moku::internal::NodeEntry::Node(node) => {
                            *self = Self::Enabled(node);
                            None
                        }
                        moku::internal::NodeEntry::Transition(new_target) => Some(new_target),
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

    pub struct BlinkyMachine {
        top_node: moku::internal::TopNode<BlinkyState, BlinkyMachine, super::Top, TopSubstate>,
    }

    impl BlinkyMachine {
        fn new(
            top_node: moku::internal::TopNode<BlinkyState, BlinkyMachine, super::Top, TopSubstate>,
        ) {
            let new = Self { top_node };
            new.top_node.init(&mut new);
            new
        }
    }

    impl moku::StateMachine<BlinkyState> for BlinkyMachine {
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

    impl moku::StateMachineBuilder<BlinkyState, BlinkyMachine, super::Top> for BlinkyMachineBuilder {
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

        fn build(self) -> BlinkyMachine {
            BlinkyMachine::new(moku::internal::TopNode::new(
                self.top_state,
                self.name.unwrap_or_else(|| String::from("Blinky")),
            ))
        }
    }

    impl<T: moku::State<BlinkyState>> moku::StateRef<BlinkyState, T> for BlinkyMachine {
        fn get_ref(&self) -> Option<&T> {
            self.top_node.get_ref()
        }

        fn get_ref_mut(&mut self) -> Option<&mut T> {
            self.top_node.get_ref_mut()
        }
    }

    impl<T: moku::State<BlinkyState>> moku::StateRef<BlinkyState, T>
        for moku::internal::TopNode<BlinkyState, super::Top, TopSubstate>
    {
        fn get_ref(&self) -> Option<&T> {
            todo!()
            //self.node.get_ref()
        }

        fn get_ref_mut(&mut self) -> Option<&mut T> {
            todo!()
            //self.node.get_ref_mut()
        }
    }

    impl moku::StateRef<BlinkyState, super::Top>
        for moku::internal::Node<BlinkyState, super::Top, TopSubstate>
    {
        fn get_ref(&self) -> Option<&super::Top> {
            Some(&self.state)
        }

        fn get_ref_mut(&mut self) -> Option<&mut super::Top> {
            Some(&mut self.state)
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
