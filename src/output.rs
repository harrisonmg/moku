use crate::internal::*;
use crate::*;

struct Top;
impl State<BlinkyState> for Top {
    fn enter() -> StateEntry<BlinkyState, Self> {
        StateEntry::State(Self {})
    }
}
impl BranchState<BlinkyState> for Top {
    fn init(&mut self) -> BlinkyState {
        BlinkyState::Enabled
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
}
impl BranchState<BlinkyState> for Enabled {
    fn init(&mut self) -> BlinkyState {
        BlinkyState::LedOn
    }
}

struct LedOn;
impl State<BlinkyState> for LedOn {
    fn enter() -> StateEntry<BlinkyState, Self> {
        StateEntry::State(Self {})
    }
}

struct LedOff;
impl State<BlinkyState> for LedOff {
    fn enter() -> StateEntry<BlinkyState, Self> {
        StateEntry::State(Self {})
    }
}

/// AUTOGEN

#[derive(Debug, EnumSetType)]
enum BlinkyState {
    Top,
    Disabled,
    Enabled,
    LedOn,
    LedOff,
}
impl StateList for BlinkyState {}

mod moku_machine {
    use crate::internal::*;
    use enumset::{enum_set, EnumSet};

    use super::{BlinkyState as SL, State};

    struct LedOn {
        state: super::LedOn,
    }

    impl Leaf<SL, super::LedOn> for LedOn {
        const STATE: EnumSet<SL> = enum_set!(SL::LedOn);

        fn get_state(&mut self) -> &mut super::LedOn {
            &mut self.state
        }

        fn take_state(self) -> super::LedOn {
            self.state
        }
    }

    struct LedOff {
        state: super::LedOff,
    }

    impl Leaf<SL, super::LedOff> for LedOff {
        const STATE: EnumSet<SL> = enum_set!(SL::LedOff);

        fn get_state(&mut self) -> &mut super::LedOff {
            &mut self.state
        }

        fn take_state(self) -> super::LedOff {
            self.state
        }
    }

    struct Enabled {
        state: super::Enabled,
        child: EnabledSubstate,
    }

    enum EnabledSubstate {
        None,
        LedOn(LedOn),
        LedOff(LedOff),
    }

    impl Branch<SL, super::Enabled> for Enabled {
        const STATE: EnumSet<SL> = enum_set!(SL::Enabled);
        const CHILDREN: EnumSet<SL> = enum_set!(SL::LedOn | SL::LedOff);

        fn get_state(&mut self) -> &mut super::Enabled {
            &mut self.state
        }

        fn take_state(self) -> super::Enabled {
            self.state
        }

        fn transition_child(&mut self, target: SL) -> TransitionResult<SL> {
            todo!();
        }
    }

    struct Disabled {
        state: super::Disabled,
    }

    impl Leaf<SL, super::Disabled> for Disabled {
        const STATE: EnumSet<SL> = enum_set!(SL::Disabled);

        fn get_state(&mut self) -> &mut super::Disabled {
            &mut self.state
        }

        fn take_state(self) -> super::Disabled {
            self.state
        }
    }

    struct Top {
        state: super::Top,
        child: TopSubstate,
    }

    enum TopSubstate {
        None,
        Disabled(Disabled),
        Enabled(Enabled),
    }

    impl Branch<SL, super::Top> for Top {
        const STATE: EnumSet<SL> = enum_set!(SL::Top);
        const CHILDREN: EnumSet<SL> = enum_set!(SL::Disabled | SL::Enabled);

        fn get_state(&mut self) -> &mut super::Top {
            &mut self.state
        }

        fn take_state(self) -> super::Top {
            self.state
        }

        fn transition_child(&mut self, target: SL) -> TransitionResult<SL> {
            if !matches!(self.child, TopSubstate::None) {
                let transition_res = match self.child {
                    TopSubstate::Disabled(ref mut substate) => substate.transition(target),
                    TopSubstate::Enabled(ref mut substate) => substate.transition(target),
                    TopSubstate::None => unreachable!(),
                };

                match transition_res {
                    TransitionResult::Done => return TransitionResult::Done,
                    TransitionResult::NewTransition(new_target) => {
                        return self.transition(new_target)
                    }
                    TransitionResult::MoveUp => {
                        let child = std::mem::replace(&mut self.child, TopSubstate::None);
                        let exit_res = match child {
                            TopSubstate::Disabled(substate) => substate.take_state().exit(),
                            TopSubstate::Enabled(substate) => substate.take_state().exit(),
                            TopSubstate::None => unreachable!(),
                        };

                        match exit_res {
                            Some(new_target) => self.transition(new_target),
                            None => TransitionResult::MoveUp,
                        }
                    }
                }
            } else {
                match target {
                    SL::
                }
            }
        }
    }
}
