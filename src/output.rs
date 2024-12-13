use crate::internal::*;
use crate::*;

struct Top;
impl TopState<BlinkyState> for Top {
    fn init() -> StateEntry<BlinkyState, Self> {
        StateEntry::State(Self {})
    }

    fn init(&mut self) -> BlinkyState {
        BlinkyState::Enabled
    }
}

struct Disabled;
impl State<BlinkyState> for Disabled {
    fn enter() -> StateEntry<BlinkyState, Self> {
        StateEntry::State(Self {})
    }

    fn init(&mut self) -> BlinkyState {
        BlinkyState::Disabled
    }
}

struct Enabled;
impl State<BlinkyState> for Enabled {
    fn enter() -> StateEntry<BlinkyState, Self> {
        StateEntry::State(Self {})
    }

    fn init(&mut self) -> BlinkyState {
        BlinkyState::Enabled
    }
}

struct LedOn;
impl State<BlinkyState> for LedOn {
    fn enter() -> StateEntry<BlinkyState, Self> {
        StateEntry::State(Self {})
    }

    fn init(&mut self) -> BlinkyState {
        BlinkyState::LedOn
    }
}

struct LedOff;
impl State<BlinkyState> for LedOff {
    fn enter() -> StateEntry<BlinkyState, Self> {
        StateEntry::State(Self {})
    }

    fn init(&mut self) -> BlinkyState {
        BlinkyState::LedOff
    }
}

// AUTOGEN

#[derive(Debug, EnumSetType)]
enum BlinkyState {
    Top,
    Disabled,
    Enabled,
    LedOn,
    LedOff,
}

impl StateEnum for BlinkyState {
    fn get_decendents(&mut self) -> EnumSet<Self> {
        todo!()
    }
}

mod state_machine {
    use super::{BlinkyState as SE, Enabled};
    use crate::internal::*;
    use enumset::{enum_set, EnumSet};

    type LedOffNode = Node<SE, super::LedOff, LedOffSubstate>;

    enum LedOffSubstate {
        None,
    }

    impl SubstateEnum<SE, super::LedOff> for LedOffSubstate {
        const STATE: EnumSet<SE> = enum_set!(SE::LedOff);

        fn none_variant() -> Self {
            Self::None
        }
    }

    type LedOnNode = Node<SE, super::LedOn, LedOnSubstate>;

    enum LedOnSubstate {
        None,
    }

    impl SubstateEnum<SE, super::LedOn> for LedOnSubstate {
        const STATE: EnumSet<SE> = enum_set!(SE::LedOn);

        fn none_variant() -> Self {
            Self::None
        }
    }

    type EnabledNode = Node<SE, super::Enabled, EnabledSubstate>;

    enum EnabledSubstate {
        None,
        LedOn(LedOnNode),
        LedOff(LedOffNode),
    }

    impl SubstateEnum<SE, super::Enabled> for EnabledSubstate {
        const STATE: EnumSet<SE> = enum_set!(SE::Enabled);
        const DECENDENTS: EnumSet<SE> = enum_set!(SE::LedOn | SE::LedOff);

        fn none_variant() -> Self {
            Self::None
        }
    }

    type DisabledNode = Node<SE, super::Disabled, DisabledSubstate>;

    enum DisabledSubstate {
        None,
    }

    impl SubstateEnum<SE, super::Disabled> for DisabledSubstate {
        const STATE: EnumSet<SE> = enum_set!(SE::Disabled);

        fn none_variant() -> Self {
            Self::None
        }
    }

    type TopNode = Node<SE, super::Top, TopSubstate>;

    enum TopSubstate {
        None,
        Enabled(EnabledNode),
        Disabled(DisabledNode),
    }

    impl SubstateEnum<SE, super::Top> for TopSubstate {
        const STATE: EnumSet<SE> = enum_set!(SE::Top);
        const DECENDENTS: EnumSet<SE> =
            enum_set!(SE::Enabled | SE::LedOn | SE::LedOff | SE::Disabled);

        fn none_variant() -> Self {
            Self::None
        }

        fn update(&mut self) -> Option<SE> {
            match self {
                Self::None => None,
                Self::Enabled(node) => node.update(),
                Self::Disabled(node) => node.update(),
            }
        }

        fn top_down_update(&mut self) -> Option<SE> {
            match self {
                Self::None => None,
                Self::Enabled(node) => node.top_down_update(),
                Self::Disabled(node) => node.top_down_update(),
            }
        }

        fn exit(&mut self) -> Option<SE> {
            let old_state = std::mem::replace(self, Self::None);
            match old_state {
                Self::None => None,
                Self::Enabled(node) => node.exit(),
                Self::Disabled(node) => node.exit(),
            }
        }

        fn transition(&mut self, target: SE) -> TransitionResult<SE> {
            match self {
                Self::None => TransitionResult::Done,
                Self::Enabled(node) => node.transition(target),
                Self::Disabled(node) => node.transition(target),
            }
        }

        fn enter_substate_towards(&mut self, target: SE) -> Option<SE> {
            match target {
                SE::Disabled => match DisabledNode::enter() {
                    NodeEntry::Node(node) => {
                        *self = Self::Disabled(node);
                        None
                    }
                    NodeEntry::Transition(new_target) => Some(new_target),
                },
                SE::Enabled | SE::LedOn | SE::LedOn => match EnabledNode::enter() {
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

    pub struct StateMachine {
        top_node: TopNode,
    }

    impl StateMachine {
        pub fn init() -> Self {
            let top_node = match TopNode::enter() {
                NodeEntry::Node(node) => node,
                NodeEntry::Transition(target)
            };

            Self { top_node }
        }

        pub fn update(&mut self) -> {

        }
    }
}
