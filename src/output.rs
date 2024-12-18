use crate::internal::*;
use crate::*;

struct Top;
impl TopState<BlinkyState> for Top {
    fn init(&mut self) -> Option<BlinkyState> {
        Some(BlinkyState::Enabled)
    }
}

struct Disabled;
impl State<BlinkyState> for Disabled {
    fn enter() -> StateEntry<BlinkyState, Self> {
        StateEntry::State(Self {})
    }
}

struct Enabled;
impl State<BlinkyState> for Enabled {
    fn enter() -> StateEntry<BlinkyState, Self> {
        StateEntry::State(Self {})
    }

    fn init(&mut self) -> Option<BlinkyState> {
        Some(BlinkyState::LedOn)
    }
}

struct LedOn;
impl State<BlinkyState> for LedOn {
    fn enter() -> StateEntry<BlinkyState, Self> {
        StateEntry::State(Self {})
    }

    fn update(&mut self) -> Option<BlinkyState> {
        Some(BlinkyState::LedOff)
    }
}

struct LedOff;
impl State<BlinkyState> for LedOff {
    fn enter() -> StateEntry<BlinkyState, Self> {
        StateEntry::State(Self {})
    }

    fn update(&mut self) -> Option<BlinkyState> {
        Some(BlinkyState::LedOn)
    }
}

// AUTOGEN

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum BlinkyState {
    Top,
    Disabled,
    Enabled,
    LedOn,
    LedOff,
}

impl StateEnum for BlinkyState {}

use state_machine::Machine;

mod state_machine {
    use crate::internal::*;

    type LedOffNode = Node<super::BlinkyState, super::LedOff, LedOffSubstate>;

    enum LedOffSubstate {
        None,
    }

    impl SubstateEnum<super::BlinkyState> for LedOffSubstate {
        fn none_variant() -> Self {
            Self::None
        }

        fn this_state() -> super::BlinkyState {
            super::BlinkyState::LedOff
        }

        fn current_state(&self) -> super::BlinkyState {
            super::BlinkyState::LedOff
        }

        fn is_state(state: super::BlinkyState) -> bool {
            matches!(state, super::BlinkyState::LedOff)
        }
    }

    type LedOnNode = Node<super::BlinkyState, super::LedOn, LedOnSubstate>;

    enum LedOnSubstate {
        None,
    }

    impl SubstateEnum<super::BlinkyState> for LedOnSubstate {
        fn none_variant() -> Self {
            Self::None
        }

        fn this_state() -> super::BlinkyState {
            super::BlinkyState::LedOn
        }

        fn current_state(&self) -> super::BlinkyState {
            super::BlinkyState::LedOn
        }

        fn is_state(state: super::BlinkyState) -> bool {
            matches!(state, super::BlinkyState::LedOn)
        }
    }

    type EnabledNode = Node<super::BlinkyState, super::Enabled, EnabledSubstate>;

    enum EnabledSubstate {
        None,
        LedOn(LedOnNode),
        LedOff(LedOffNode),
    }

    impl SubstateEnum<super::BlinkyState> for EnabledSubstate {
        fn none_variant() -> Self {
            Self::None
        }

        fn this_state() -> super::BlinkyState {
            super::BlinkyState::Enabled
        }

        fn current_state(&self) -> super::BlinkyState {
            match self {
                Self::None => super::BlinkyState::Enabled,
                Self::LedOn(state) => state.current_state(),
                Self::LedOff(state) => state.current_state(),
            }
        }

        fn is_state(state: super::BlinkyState) -> bool {
            matches!(state, super::BlinkyState::Enabled)
        }

        fn is_ancestor(state: super::BlinkyState) -> bool {
            matches!(
                state,
                super::BlinkyState::LedOn | super::BlinkyState::LedOff
            )
        }

        fn update(&mut self) -> Option<super::BlinkyState> {
            match self {
                Self::None => None,
                Self::LedOn(node) => node.update(),
                Self::LedOff(node) => node.update(),
            }
        }

        fn top_down_update(&mut self) -> Option<super::BlinkyState> {
            match self {
                Self::None => None,
                Self::LedOn(node) => node.top_down_update(),
                Self::LedOff(node) => node.top_down_update(),
            }
        }

        fn exit(&mut self) -> Option<super::BlinkyState> {
            let old_state = std::mem::replace(self, Self::None);
            match old_state {
                Self::None => None,
                Self::LedOn(node) => node.exit(),
                Self::LedOff(node) => node.exit(),
            }
        }

        fn transition(
            &mut self,
            target: super::BlinkyState,
        ) -> TransitionResult<super::BlinkyState> {
            match self {
                Self::None => TransitionResult::MoveUp,
                Self::LedOn(node) => node.transition(target),
                Self::LedOff(node) => node.transition(target),
            }
        }

        fn enter_substate_towards(
            &mut self,
            target: super::BlinkyState,
        ) -> Option<super::BlinkyState> {
            match target {
                super::BlinkyState::LedOn => match LedOnNode::enter() {
                    NodeEntry::Node(node) => {
                        *self = Self::LedOn(node);
                        None
                    }
                    NodeEntry::Transition(new_target) => Some(new_target),
                },
                super::BlinkyState::LedOff => match LedOffNode::enter() {
                    NodeEntry::Node(node) => {
                        *self = Self::LedOff(node);
                        None
                    }
                    NodeEntry::Transition(new_target) => Some(new_target),
                },
                _ => unreachable!(),
            }
        }
    }

    type DisabledNode = Node<super::BlinkyState, super::Disabled, DisabledSubstate>;

    enum DisabledSubstate {
        None,
    }

    impl SubstateEnum<super::BlinkyState> for DisabledSubstate {
        fn none_variant() -> Self {
            Self::None
        }

        fn this_state() -> super::BlinkyState {
            super::BlinkyState::Disabled
        }

        fn current_state(&self) -> super::BlinkyState {
            super::BlinkyState::Disabled
        }

        fn is_state(state: super::BlinkyState) -> bool {
            matches!(state, super::BlinkyState::Disabled)
        }

        fn is_ancestor(state: super::BlinkyState) -> bool {
            false
        }
    }

    type TopNode = Node<super::BlinkyState, super::Top, TopSubstate>;

    enum TopSubstate {
        None,
        Enabled(EnabledNode),
        Disabled(DisabledNode),
    }

    impl SubstateEnum<super::BlinkyState> for TopSubstate {
        fn none_variant() -> Self {
            Self::None
        }

        fn this_state() -> super::BlinkyState {
            super::BlinkyState::Top
        }

        fn current_state(&self) -> super::BlinkyState {
            match self {
                Self::None => super::BlinkyState::Top,
                Self::Enabled(state) => state.current_state(),
                Self::Disabled(state) => state.current_state(),
            }
        }

        fn is_state(state: super::BlinkyState) -> bool {
            matches!(state, super::BlinkyState::Enabled)
        }

        fn is_ancestor(state: super::BlinkyState) -> bool {
            true
        }

        fn update(&mut self) -> Option<super::BlinkyState> {
            match self {
                Self::None => None,
                Self::Enabled(node) => node.update(),
                Self::Disabled(node) => node.update(),
            }
        }

        fn top_down_update(&mut self) -> Option<super::BlinkyState> {
            match self {
                Self::None => None,
                Self::Enabled(node) => node.top_down_update(),
                Self::Disabled(node) => node.top_down_update(),
            }
        }

        fn exit(&mut self) -> Option<super::BlinkyState> {
            let old_state = std::mem::replace(self, Self::None);
            match old_state {
                Self::None => None,
                Self::Enabled(node) => node.exit(),
                Self::Disabled(node) => node.exit(),
            }
        }

        fn transition(
            &mut self,
            target: super::BlinkyState,
        ) -> TransitionResult<super::BlinkyState> {
            match self {
                Self::None => TransitionResult::MoveUp,
                Self::Enabled(node) => node.transition(target),
                Self::Disabled(node) => node.transition(target),
            }
        }

        fn enter_substate_towards(
            &mut self,
            target: super::BlinkyState,
        ) -> Option<super::BlinkyState> {
            match target {
                super::BlinkyState::Disabled => match DisabledNode::enter() {
                    NodeEntry::Node(node) => {
                        *self = Self::Disabled(node);
                        None
                    }
                    NodeEntry::Transition(new_target) => Some(new_target),
                },
                super::BlinkyState::Enabled
                | super::BlinkyState::LedOn
                | super::BlinkyState::LedOff => match EnabledNode::enter() {
                    NodeEntry::Node(node) => {
                        *self = Self::Enabled(node);
                        None
                    }
                    NodeEntry::Transition(new_target) => Some(new_target),
                },
                _ => unreachable!(),
            }
        }
    }

    pub struct Machine {
        node: Node<super::BlinkyState, super::Top, TopSubstate>,
    }

    impl StateMachine<super::BlinkyState, super::Top> for Machine {
        fn from_top_state(mut top_state: super::Top) -> Self {
            let initial_transition = crate::TopState::init(&mut top_state);

            let mut new = Self {
                node: Node::from_state(top_state),
            };

            if let Some(target) = initial_transition {
                new.transition(target);
            }

            new
        }

        fn update(&mut self) {
            if let Some(target) = self.node.update() {
                self.transition(target);
            }
        }

        fn top_down_update(&mut self) {
            if let Some(target) = self.node.top_down_update() {
                self.transition(target);
            }
        }

        fn transition(&mut self, target: super::BlinkyState) {
            match self.node.transition(target) {
                TransitionResult::Done => return,
                TransitionResult::MoveUp => unreachable!(),
                TransitionResult::NewTransition(new_target) => self.transition(new_target),
            }
        }

        fn get_state(&self) -> super::BlinkyState {
            self.node.current_state()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_log::test;

    #[test]
    fn state_machine_init() {
        let machine = Machine::from_top_state(Top {});
        assert_eq!(machine.get_state(), BlinkyState::Enabled)
    }
}
