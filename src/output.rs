use crate::internal::*;
use crate::*;

struct Top;
impl State<BlinkyState> for Top {
    fn enter() -> StateEntry<BlinkyState, Self> {
        StateEntry::State(Self {})
    }

    // TODO autogen
    fn init(&mut self) -> BlinkyState {
        BlinkyState::Enabled
    }
}

struct Disabled;
impl State<BlinkyState> for Disabled {
    fn enter() -> StateEntry<BlinkyState, Self> {
        StateEntry::State(Self {})
    }

    // TODO autogen
    fn init(&mut self) -> BlinkyState {
        BlinkyState::Disabled
    }
}

struct Enabled;
impl State<BlinkyState> for Enabled {
    fn enter() -> StateEntry<BlinkyState, Self> {
        StateEntry::State(Self {})
    }

    // TODO autogen
    fn init(&mut self) -> BlinkyState {
        BlinkyState::Enabled
    }
}

struct LedOn;
impl State<BlinkyState> for LedOn {
    fn enter() -> StateEntry<BlinkyState, Self> {
        StateEntry::State(Self {})
    }

    // TODO autogen
    fn init(&mut self) -> BlinkyState {
        BlinkyState::LedOn
    }
}

struct LedOff;
impl State<BlinkyState> for LedOff {
    fn enter() -> StateEntry<BlinkyState, Self> {
        StateEntry::State(Self {})
    }

    // TODO autogen
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

mod moku_machine {
    use super::BlinkyState as SE;
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

        fn none_variant() -> Self {
            Self::None
        }

        fn
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

        fn none_variant() -> Self {
            Self::None
        }
    }

    type StateMachine =
}
