use std::time::{Duration, Instant};

use crate::*;

use state_machine::{BlinkyMachine, BlinkyMachineBuilder, BlinkyState};

struct Top;
impl TopState<BlinkyState> for Top {
    //fn init(&mut self) -> Option<BlinkyState> {
    //    Some(BlinkyState::Disabled)
    //}
}

struct Disabled;
impl State<BlinkyState> for Disabled {
    type Superstates<'a> = state_machine::TopSuperstates<'a>;

    fn enter<'a>(superstates: &mut Self::Superstates<'a>) -> StateEntry<Self, BlinkyState> {
        StateEntry::State(Self {})
    }
}

struct Enabled {
    blink_period: Duration,
}

impl State<BlinkyState> for Enabled {
    type Superstates<'a> = state_machine::TopSuperstates<'a>;

    fn enter<'a>(superstates: &mut Self::Superstates<'a>) -> StateEntry<Self, BlinkyState> {
        StateEntry::State(Self {
            blink_period: Duration::from_secs(2),
        })
    }

    //fn init<'a>(&mut self, superstates: &mut Self::Superstates<'a>) -> Option<BlinkyState> {
    //    Some(BlinkyState::LedOn)
    //}
}

struct LedOn {
    entry_time: Instant,
}

impl State<BlinkyState> for LedOn {
    type Superstates<'a> = state_machine::EnabledSuperstates<'a>;

    fn enter<'a>(superstates: &mut Self::Superstates<'a>) -> StateEntry<Self, BlinkyState> {
        StateEntry::State(Self {
            entry_time: Instant::now(),
        })
    }

    fn update<'a>(&mut self, superstates: &mut Self::Superstates<'a>) -> Option<BlinkyState> {
        if (self.entry_time.elapsed() >= superstates.enabled.blink_period) {
            Some(BlinkyState::LedOff)
        } else {
            None
        }
    }
}

struct LedOff {
    entry_time: Instant,
}

impl State<BlinkyState> for LedOff {
    type Superstates<'a> = state_machine::EnabledSuperstates<'a>;

    fn enter<'a>(superstates: &mut Self::Superstates<'a>) -> StateEntry<Self, BlinkyState> {
        StateEntry::State(Self {
            entry_time: Instant::now(),
        })
    }

    fn update<'a>(&mut self, superstates: &mut Self::Superstates<'a>) -> Option<BlinkyState> {
        if (self.entry_time.elapsed() >= superstates.enabled.blink_period) {
            Some(BlinkyState::LedOn)
        } else {
            None
        }
    }
}

// AUTOGEN //

mod state_machine {
    use crate as moku;

    use moku::StateRef;

    pub const BLINKY_STATE_CHART: &str = "Top
\u{0251C}\u{02500} Disabled
\u{02514}\u{02500} Enabled
   \u{0251C}\u{02500} LedOn
   \u{02514}\u{02500} LedOff";

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum BlinkyState {
        Top,
        Disabled,
        Enabled,
        LedOn,
        LedOff,
    }

    impl moku::StateEnum for BlinkyState {}

    type LedOffNode = moku::internal::Node<BlinkyState, super::LedOff, LedOffSubstate>;

    impl moku::StateRef<BlinkyState, super::Top> for LedOffNode {
        fn state_ref(&self) -> Option<&super::Top> {
            None
        }

        fn state_mut(&mut self) -> Option<&mut super::Top> {
            None
        }
    }

    impl moku::StateRef<BlinkyState, super::Disabled> for LedOffNode {
        fn state_ref(&self) -> Option<&super::Disabled> {
            None
        }

        fn state_mut(&mut self) -> Option<&mut super::Disabled> {
            None
        }
    }

    impl moku::StateRef<BlinkyState, super::Enabled> for LedOffNode {
        fn state_ref(&self) -> Option<&super::Enabled> {
            None
        }

        fn state_mut(&mut self) -> Option<&mut super::Enabled> {
            None
        }
    }

    impl moku::StateRef<BlinkyState, super::LedOn> for LedOffNode {
        fn state_ref(&self) -> Option<&super::LedOn> {
            None
        }

        fn state_mut(&mut self) -> Option<&mut super::LedOn> {
            None
        }
    }

    impl moku::StateRef<BlinkyState, super::LedOff> for LedOffNode {
        fn state_ref(&self) -> Option<&super::LedOff> {
            Some(&self.state)
        }

        fn state_mut(&mut self) -> Option<&mut super::LedOff> {
            Some(&mut self.state)
        }
    }

    enum LedOffSubstate {
        None,
    }

    impl moku::internal::SubstateEnum<BlinkyState, super::LedOff> for LedOffSubstate {
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

    type LedOnNode = moku::internal::Node<BlinkyState, super::LedOn, LedOnSubstate>;

    impl moku::StateRef<BlinkyState, super::Top> for LedOnNode {
        fn state_ref(&self) -> Option<&super::Top> {
            None
        }

        fn state_mut(&mut self) -> Option<&mut super::Top> {
            None
        }
    }

    impl moku::StateRef<BlinkyState, super::Disabled> for LedOnNode {
        fn state_ref(&self) -> Option<&super::Disabled> {
            None
        }

        fn state_mut(&mut self) -> Option<&mut super::Disabled> {
            None
        }
    }

    impl moku::StateRef<BlinkyState, super::Enabled> for LedOnNode {
        fn state_ref(&self) -> Option<&super::Enabled> {
            None
        }

        fn state_mut(&mut self) -> Option<&mut super::Enabled> {
            None
        }
    }

    impl moku::StateRef<BlinkyState, super::LedOn> for LedOnNode {
        fn state_ref(&self) -> Option<&super::LedOn> {
            Some(&self.state)
        }

        fn state_mut(&mut self) -> Option<&mut super::LedOn> {
            Some(&mut self.state)
        }
    }

    impl moku::StateRef<BlinkyState, super::LedOff> for LedOnNode {
        fn state_ref(&self) -> Option<&super::LedOff> {
            None
        }

        fn state_mut(&mut self) -> Option<&mut super::LedOff> {
            None
        }
    }

    enum LedOnSubstate {
        None,
    }

    impl moku::internal::SubstateEnum<BlinkyState, super::LedOn> for LedOnSubstate {
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

    type EnabledNode = moku::internal::Node<BlinkyState, super::Enabled, EnabledSubstate>;

    impl moku::StateRef<BlinkyState, super::Top> for EnabledNode {
        fn state_ref(&self) -> Option<&super::Top> {
            None
        }

        fn state_mut(&mut self) -> Option<&mut super::Top> {
            None
        }
    }

    impl moku::StateRef<BlinkyState, super::Disabled> for EnabledNode {
        fn state_ref(&self) -> Option<&super::Disabled> {
            None
        }

        fn state_mut(&mut self) -> Option<&mut super::Disabled> {
            None
        }
    }

    impl moku::StateRef<BlinkyState, super::Enabled> for EnabledNode {
        fn state_ref(&self) -> Option<&super::Enabled> {
            Some(&self.state)
        }

        fn state_mut(&mut self) -> Option<&mut super::Enabled> {
            Some(&mut self.state)
        }
    }

    impl moku::StateRef<BlinkyState, super::LedOn> for EnabledNode {
        fn state_ref(&self) -> Option<&super::LedOn> {
            match &self.substate {
                EnabledSubstate::None => None,
                EnabledSubstate::LedOff(node) => node.state_ref(),
                EnabledSubstate::LedOn(node) => node.state_ref(),
            }
        }

        fn state_mut(&mut self) -> Option<&mut super::LedOn> {
            match &mut self.substate {
                EnabledSubstate::None => None,
                EnabledSubstate::LedOff(node) => node.state_mut(),
                EnabledSubstate::LedOn(node) => node.state_mut(),
            }
        }
    }

    impl moku::StateRef<BlinkyState, super::LedOff> for EnabledNode {
        fn state_ref(&self) -> Option<&super::LedOff> {
            match &self.substate {
                EnabledSubstate::None => None,
                EnabledSubstate::LedOff(node) => node.state_ref(),
                EnabledSubstate::LedOn(node) => node.state_ref(),
            }
        }

        fn state_mut(&mut self) -> Option<&mut super::LedOff> {
            match &mut self.substate {
                EnabledSubstate::None => None,
                EnabledSubstate::LedOff(node) => node.state_mut(),
                EnabledSubstate::LedOn(node) => node.state_mut(),
            }
        }
    }

    enum EnabledSubstate {
        None,
        LedOn(LedOnNode),
        LedOff(LedOffNode),
    }

    pub struct EnabledSuperstates<'a> {
        pub top: &'a mut super::Top,
        pub enabled: &'a mut super::Enabled,
    }

    impl<'a> EnabledSuperstates<'a> {
        pub fn new(state: &'a mut super::Enabled, superstates: &'a mut TopSuperstates) -> Self {
            Self {
                top: superstates.top,
                enabled: state,
            }
        }
    }

    impl moku::internal::SubstateEnum<BlinkyState, super::Enabled> for EnabledSubstate {
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

        fn update<'a>(
            &mut self,
            state: &mut super::Enabled,
            superstates: &mut <super::Enabled as super::State<BlinkyState>>::Superstates<'a>,
        ) -> Option<BlinkyState> {
            match self {
                Self::None => None,
                Self::LedOn(node) => node.update(&mut EnabledSuperstates::new(state, superstates)),
                Self::LedOff(node) => node.update(&mut EnabledSuperstates::new(state, superstates)),
            }
        }

        fn top_down_update<'a>(
            &mut self,
            state: &mut super::Enabled,
            superstates: &mut <super::Enabled as super::State<BlinkyState>>::Superstates<'a>,
        ) -> Option<BlinkyState> {
            match self {
                Self::None => None,
                Self::LedOn(node) => {
                    node.top_down_update(&mut EnabledSuperstates::new(state, superstates))
                }
                Self::LedOff(node) => {
                    node.top_down_update(&mut EnabledSuperstates::new(state, superstates))
                }
            }
        }

        fn exit<'a>(
            &mut self,
            state: &mut super::Enabled,
            superstates: &mut <super::Enabled as super::State<BlinkyState>>::Superstates<'a>,
        ) -> Option<BlinkyState> {
            let old_state = std::mem::replace(self, Self::None);
            match old_state {
                Self::None => None,
                Self::LedOn(node) => node.exit(&mut EnabledSuperstates::new(state, superstates)),
                Self::LedOff(node) => node.exit(&mut EnabledSuperstates::new(state, superstates)),
            }
        }

        fn transition<'a>(
            &mut self,
            target: BlinkyState,
            state: &mut super::Enabled,
            superstates: &mut <super::Enabled as super::State<BlinkyState>>::Superstates<'a>,
        ) -> super::TransitionResult<BlinkyState> {
            match self {
                Self::None => moku::internal::TransitionResult::MoveUp,
                Self::LedOn(node) => {
                    node.transition(target, &mut EnabledSuperstates::new(state, superstates))
                }
                Self::LedOff(node) => {
                    node.transition(target, &mut EnabledSuperstates::new(state, superstates))
                }
            }
        }

        fn enter_substate_towards<'a>(
            &mut self,
            target: BlinkyState,
            state: &mut super::Enabled,
            superstates: &mut <super::Enabled as super::State<BlinkyState>>::Superstates<'a>,
        ) -> Option<BlinkyState> {
            match target {
                BlinkyState::LedOn => {
                    match LedOnNode::enter(&mut EnabledSuperstates::new(state, superstates)) {
                        moku::internal::NodeEntry::Node(node) => {
                            *self = Self::LedOn(node);
                            None
                        }
                        moku::internal::NodeEntry::Transition(new_target) => Some(new_target),
                    }
                }
                BlinkyState::LedOff => {
                    match LedOffNode::enter(&mut EnabledSuperstates::new(state, superstates)) {
                        moku::internal::NodeEntry::Node(node) => {
                            *self = Self::LedOff(node);
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
                    Self::LedOn(node) => node.state_matches(state),
                    Self::LedOff(node) => node.state_matches(state),
                };
        }
    }

    type DisabledNode = moku::internal::Node<BlinkyState, super::Disabled, DisabledSubstate>;

    impl moku::StateRef<BlinkyState, super::Top> for DisabledNode {
        fn state_ref(&self) -> Option<&super::Top> {
            None
        }

        fn state_mut(&mut self) -> Option<&mut super::Top> {
            None
        }
    }

    impl moku::StateRef<BlinkyState, super::Disabled> for DisabledNode {
        fn state_ref(&self) -> Option<&super::Disabled> {
            Some(&self.state)
        }

        fn state_mut(&mut self) -> Option<&mut super::Disabled> {
            Some(&mut self.state)
        }
    }

    impl moku::StateRef<BlinkyState, super::Enabled> for DisabledNode {
        fn state_ref(&self) -> Option<&super::Enabled> {
            None
        }

        fn state_mut(&mut self) -> Option<&mut super::Enabled> {
            None
        }
    }

    impl moku::StateRef<BlinkyState, super::LedOn> for DisabledNode {
        fn state_ref(&self) -> Option<&super::LedOn> {
            None
        }

        fn state_mut(&mut self) -> Option<&mut super::LedOn> {
            None
        }
    }

    impl moku::StateRef<BlinkyState, super::LedOff> for DisabledNode {
        fn state_ref(&self) -> Option<&super::LedOff> {
            None
        }

        fn state_mut(&mut self) -> Option<&mut super::LedOff> {
            None
        }
    }

    enum DisabledSubstate {
        None,
    }

    impl moku::internal::SubstateEnum<BlinkyState, super::Disabled> for DisabledSubstate {
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

    type TopNode = moku::internal::Node<BlinkyState, super::Top, TopSubstate>;

    impl moku::StateRef<BlinkyState, super::Top> for TopNode {
        fn state_ref(&self) -> Option<&super::Top> {
            Some(&self.state)
        }

        fn state_mut(&mut self) -> Option<&mut super::Top> {
            Some(&mut self.state)
        }
    }

    impl moku::StateRef<BlinkyState, super::Disabled> for TopNode {
        fn state_ref(&self) -> Option<&super::Disabled> {
            match &self.substate {
                TopSubstate::None => None,
                TopSubstate::Enabled(node) => node.state_ref(),
                TopSubstate::Disabled(node) => node.state_ref(),
            }
        }

        fn state_mut(&mut self) -> Option<&mut super::Disabled> {
            match &mut self.substate {
                TopSubstate::None => None,
                TopSubstate::Enabled(node) => node.state_mut(),
                TopSubstate::Disabled(node) => node.state_mut(),
            }
        }
    }

    impl moku::StateRef<BlinkyState, super::Enabled> for TopNode {
        fn state_ref(&self) -> Option<&super::Enabled> {
            match &self.substate {
                TopSubstate::None => None,
                TopSubstate::Enabled(node) => node.state_ref(),
                TopSubstate::Disabled(node) => node.state_ref(),
            }
        }

        fn state_mut(&mut self) -> Option<&mut super::Enabled> {
            match &mut self.substate {
                TopSubstate::None => None,
                TopSubstate::Enabled(node) => node.state_mut(),
                TopSubstate::Disabled(node) => node.state_mut(),
            }
        }
    }

    impl moku::StateRef<BlinkyState, super::LedOn> for TopNode {
        fn state_ref(&self) -> Option<&super::LedOn> {
            match &self.substate {
                TopSubstate::None => None,
                TopSubstate::Enabled(node) => node.state_ref(),
                TopSubstate::Disabled(node) => node.state_ref(),
            }
        }

        fn state_mut(&mut self) -> Option<&mut super::LedOn> {
            match &mut self.substate {
                TopSubstate::None => None,
                TopSubstate::Enabled(node) => node.state_mut(),
                TopSubstate::Disabled(node) => node.state_mut(),
            }
        }
    }

    impl moku::StateRef<BlinkyState, super::LedOff> for TopNode {
        fn state_ref(&self) -> Option<&super::LedOff> {
            match &self.substate {
                TopSubstate::None => None,
                TopSubstate::Enabled(node) => node.state_ref(),
                TopSubstate::Disabled(node) => node.state_ref(),
            }
        }

        fn state_mut(&mut self) -> Option<&mut super::LedOff> {
            match &mut self.substate {
                TopSubstate::None => None,
                TopSubstate::Enabled(node) => node.state_mut(),
                TopSubstate::Disabled(node) => node.state_mut(),
            }
        }
    }

    enum TopSubstate {
        None,
        Enabled(EnabledNode),
        Disabled(DisabledNode),
    }

    pub struct TopSuperstates<'a> {
        top: &'a mut super::Top,
    }

    impl<'a> TopSuperstates<'a> {
        pub fn new(state: &'a mut super::Top, superstates: &'a mut moku::NoSuperstates) -> Self {
            Self { top: state }
        }
    }

    impl moku::internal::SubstateEnum<BlinkyState, super::Top> for TopSubstate {
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
            !matches!(state, BlinkyState::Top)
        }

        fn update<'a>(
            &mut self,
            state: &mut super::Top,
            superstates: &mut <super::Top as super::State<BlinkyState>>::Superstates<'a>,
        ) -> Option<BlinkyState> {
            match self {
                Self::None => None,
                Self::Enabled(node) => node.update(&mut TopSuperstates::new(state, superstates)),
                Self::Disabled(node) => node.update(&mut TopSuperstates::new(state, superstates)),
            }
        }

        fn top_down_update<'a>(
            &mut self,
            state: &mut super::Top,
            superstates: &mut <super::Top as super::State<BlinkyState>>::Superstates<'a>,
        ) -> Option<BlinkyState> {
            match self {
                Self::None => None,
                Self::Enabled(node) => {
                    node.top_down_update(&mut TopSuperstates::new(state, superstates))
                }
                Self::Disabled(node) => {
                    node.top_down_update(&mut TopSuperstates::new(state, superstates))
                }
            }
        }

        fn exit<'a>(
            &mut self,
            state: &mut super::Top,
            superstates: &mut <super::Top as super::State<BlinkyState>>::Superstates<'a>,
        ) -> Option<BlinkyState> {
            let old_state = std::mem::replace(self, Self::None);
            match old_state {
                Self::None => None,
                Self::Enabled(node) => node.exit(&mut TopSuperstates::new(state, superstates)),
                Self::Disabled(node) => node.exit(&mut TopSuperstates::new(state, superstates)),
            }
        }

        fn transition<'a>(
            &mut self,
            target: BlinkyState,
            state: &mut super::Top,
            superstates: &mut <super::Top as super::State<BlinkyState>>::Superstates<'a>,
        ) -> super::TransitionResult<BlinkyState> {
            match self {
                Self::None => moku::internal::TransitionResult::MoveUp,
                Self::Enabled(node) => {
                    node.transition(target, &mut TopSuperstates::new(state, superstates))
                }
                Self::Disabled(node) => {
                    node.transition(target, &mut TopSuperstates::new(state, superstates))
                }
            }
        }

        fn enter_substate_towards<'a>(
            &mut self,
            target: BlinkyState,
            state: &mut super::Top,
            superstates: &mut <super::Top as super::State<BlinkyState>>::Superstates<'a>,
        ) -> Option<BlinkyState> {
            match target {
                BlinkyState::Disabled => {
                    match DisabledNode::enter(&mut TopSuperstates::new(state, superstates)) {
                        moku::internal::NodeEntry::Node(node) => {
                            *self = Self::Disabled(node);
                            None
                        }
                        moku::internal::NodeEntry::Transition(new_target) => Some(new_target),
                    }
                }
                BlinkyState::Enabled | BlinkyState::LedOn | BlinkyState::LedOff => {
                    match EnabledNode::enter(&mut TopSuperstates::new(state, superstates)) {
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
        top_node: moku::internal::TopNode<BlinkyState, super::Top, TopSubstate>,
    }

    impl BlinkyMachine {
        fn new(top_node: moku::internal::TopNode<BlinkyState, super::Top, TopSubstate>) -> Self {
            let mut new = Self { top_node };
            new.top_node.init();
            new
        }
    }

    impl moku::StateMachine<BlinkyState, super::Top> for BlinkyMachine {
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

        fn top_ref(&self) -> &super::Top {
            &self.top_node.node.state
        }

        fn top_mut(&mut self) -> &mut super::Top {
            &mut self.top_node.node.state
        }
    }

    impl moku::StateRef<BlinkyState, super::Top> for BlinkyMachine {
        fn state_ref(&self) -> Option<&super::Top> {
            self.top_node.node.state_ref()
        }

        fn state_mut(&mut self) -> Option<&mut super::Top> {
            self.top_node.node.state_mut()
        }
    }

    impl moku::StateRef<BlinkyState, super::Disabled> for BlinkyMachine {
        fn state_ref(&self) -> Option<&super::Disabled> {
            self.top_node.node.state_ref()
        }

        fn state_mut(&mut self) -> Option<&mut super::Disabled> {
            self.top_node.node.state_mut()
        }
    }

    impl moku::StateRef<BlinkyState, super::Enabled> for BlinkyMachine {
        fn state_ref(&self) -> Option<&super::Enabled> {
            self.top_node.node.state_ref()
        }

        fn state_mut(&mut self) -> Option<&mut super::Enabled> {
            self.top_node.node.state_mut()
        }
    }

    impl moku::StateRef<BlinkyState, super::LedOn> for BlinkyMachine {
        fn state_ref(&self) -> Option<&super::LedOn> {
            self.top_node.node.state_ref()
        }

        fn state_mut(&mut self) -> Option<&mut super::LedOn> {
            self.top_node.node.state_mut()
        }
    }

    impl moku::StateRef<BlinkyState, super::LedOff> for BlinkyMachine {
        fn state_ref(&self) -> Option<&super::LedOff> {
            self.top_node.node.state_ref()
        }

        fn state_mut(&mut self) -> Option<&mut super::LedOff> {
            self.top_node.node.state_mut()
        }
    }

    pub struct BlinkyMachineBuilder {
        top_state: super::Top,
        name: Option<String>,
    }

    impl moku::StateMachineBuilder<BlinkyState, super::Top, BlinkyMachine> for BlinkyMachineBuilder {
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
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_log::test;

    #[test]
    fn basic() {
        println!("{}", state_machine::BLINKY_STATE_CHART);
        let mut machine = BlinkyMachineBuilder::new(Top {}).build();

        let top: &Top = machine.top_ref();

        let state: Option<&Top> = machine.state_ref();
        assert!(state.is_some());
        let state: Option<&Disabled> = machine.state_ref();
        assert!(state.is_none());
        let state: Option<&Enabled> = machine.state_ref();
        assert!(state.is_none());
        let state: Option<&LedOn> = machine.state_ref();
        assert!(state.is_none());
        let state: Option<&LedOff> = machine.state_ref();
        assert!(state.is_none());

        machine.transition(BlinkyState::Disabled);
        let state: Option<&Top> = machine.state_ref();
        assert!(state.is_some());
        let state: Option<&Disabled> = machine.state_ref();
        assert!(state.is_some());
        let state: Option<&Enabled> = machine.state_ref();
        assert!(state.is_none());
        let state: Option<&LedOn> = machine.state_ref();
        assert!(state.is_none());
        let state: Option<&LedOff> = machine.state_ref();
        assert!(state.is_none());

        machine.transition(BlinkyState::Enabled);
        let state: Option<&Top> = machine.state_ref();
        assert!(state.is_some());
        let state: Option<&Disabled> = machine.state_ref();
        assert!(state.is_none());
        let state: Option<&Enabled> = machine.state_ref();
        assert!(state.is_some());
        let state: Option<&LedOn> = machine.state_ref();
        assert!(state.is_none());
        let state: Option<&LedOff> = machine.state_ref();
        assert!(state.is_none());

        machine.transition(BlinkyState::LedOn);
        let state: Option<&Top> = machine.state_ref();
        assert!(state.is_some());
        let state: Option<&Disabled> = machine.state_ref();
        assert!(state.is_none());
        let state: Option<&Enabled> = machine.state_ref();
        assert!(state.is_some());
        let state: Option<&LedOn> = machine.state_ref();
        assert!(state.is_some());
        let state: Option<&LedOff> = machine.state_ref();
        assert!(state.is_none());

        machine.transition(BlinkyState::LedOff);
        let state: Option<&Top> = machine.state_ref();
        assert!(state.is_some());
        let state: Option<&Disabled> = machine.state_ref();
        assert!(state.is_none());
        let state: Option<&Enabled> = machine.state_ref();
        assert!(state.is_some());
        let state: Option<&LedOn> = machine.state_ref();
        assert!(state.is_none());
        let state: Option<&LedOff> = machine.state_ref();
        assert!(state.is_some());
    }
}
