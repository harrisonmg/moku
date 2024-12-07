//use crate::internal::*;
//use crate::*;

//struct Top;
//impl State<BlinkyState> for Top {
//    fn enter() -> StateEntry<BlinkyState, Self> {
//        StateEntry::State(Self {})
//    }
//}
//impl BranchState<BlinkyState> for Top {
//    fn init(&mut self) -> BlinkyState {
//        BlinkyState::Enabled
//    }
//}

//struct Disabled;
//impl State<BlinkyState> for Disabled {
//    fn enter() -> StateEntry<BlinkyState, Self> {
//        StateEntry::State(Self {})
//    }
//}

//struct Enabled;
//impl State<BlinkyState> for Enabled {
//    fn enter() -> StateEntry<BlinkyState, Self> {
//        StateEntry::State(Self {})
//    }
//}
//impl BranchState<BlinkyState> for Enabled {
//    fn init(&mut self) -> BlinkyState {
//        BlinkyState::LedOn
//    }
//}

//struct LedOn;
//impl State<BlinkyState> for LedOn {
//    fn enter() -> StateEntry<BlinkyState, Self> {
//        StateEntry::State(Self {})
//    }
//}

//struct LedOff;
//impl State<BlinkyState> for LedOff {
//    fn enter() -> StateEntry<BlinkyState, Self> {
//        StateEntry::State(Self {})
//    }
//}

///// AUTOGEN

//#[derive(Debug, EnumSetType)]
//enum BlinkyState {
//    Top,
//    Disabled,
//    Enabled,
//    LedOn,
//    LedOff,
//}
//impl StateEnum for BlinkyState {}

//mod moku_machine {
//    use crate::internal::*;
//    use enumset::{enum_set, EnumSet};

//    use super::{BlinkyState as SE, State};

//    struct LedOn {
//        state: super::LedOn,
//    }

//    impl Leaf<SE, super::LedOn> for LedOn {
//        const STATE: EnumSet<SE> = enum_set!(SE::LedOn);

//        fn get_state(&mut self) -> &mut super::LedOn {
//            &mut self.state
//        }

//        fn take_state(self) -> super::LedOn {
//            self.state
//        }
//    }

//    struct LedOff {
//        state: super::LedOff,
//    }

//    impl Leaf<SE, super::LedOff> for LedOff {
//        const STATE: EnumSet<SE> = enum_set!(SE::LedOff);

//        fn get_state(&mut self) -> &mut super::LedOff {
//            &mut self.state
//        }

//        fn take_state(self) -> super::LedOff {
//            self.state
//        }
//    }

//    struct Enabled {
//        state: super::Enabled,
//        child: EnabledSubstate,
//    }

//    enum EnabledSubstate {
//        None,
//        LedOn(LedOn),
//        LedOff(LedOff),
//    }

//    impl Branch<SE, super::Enabled> for Enabled {
//        const STATE: EnumSet<SE> = enum_set!(SE::Enabled);
//        const CHILDREN: [EnumSet<SE>; 2] = [enum_set!(SE::LedOn), enum_set!(SE::LedOff)];

//        fn get_state(&mut self) -> &mut super::Enabled {
//            &mut self.state
//        }

//        fn take_state(self) -> super::Enabled {
//            self.state
//        }

//        fn transition_child(&mut self, target: SE) -> TransitionResult<SE> {
//            todo!();
//        }
//    }

//    struct Disabled {
//        state: super::Disabled,
//    }

//    impl Leaf<SE, super::Disabled> for Disabled {
//        const STATE: EnumSet<SE> = enum_set!(SE::Disabled);

//        fn get_state(&mut self) -> &mut super::Disabled {
//            &mut self.state
//        }

//        fn take_state(self) -> super::Disabled {
//            self.state
//        }
//    }

//    struct Top {
//        state: super::Top,
//        child: TopSubstate,
//    }

//    enum TopSubstate {
//        None,
//        Disabled(Disabled),
//        Enabled(Enabled),
//    }

//    impl Branch<SE, super::Top> for Top {
//        const STATE: EnumSet<SE> = enum_set!(SE::Top);
//        const CHILDREN: EnumSet<SE> = enum_set!(SE::Disabled | SE::Enabled);

//        fn get_state(&mut self) -> &mut super::Top {
//            &mut self.state
//        }

//        fn take_state(self) -> super::Top {
//            self.state
//        }

//        fn transition_child(&mut self, target: SE) -> TransitionResult<SE> {
//            if !matches!(self.child, TopSubstate::None) {
//                let transition_res = match self.child {
//                    TopSubstate::Disabled(ref mut substate) => substate.transition(target),
//                    TopSubstate::Enabled(ref mut substate) => substate.transition(target),
//                    TopSubstate::None => unreachable!(),
//                };

//                match transition_res {
//                    TransitionResult::Done => return TransitionResult::Done,
//                    TransitionResult::NewTransition(new_target) => self.transition(new_target),
//                    TransitionResult::MoveUp => {
//                        let child = std::mem::replace(&mut self.child, TopSubstate::None);
//                        let exit_res = match child {
//                            TopSubstate::Disabled(substate) => substate.take_state().exit(),
//                            TopSubstate::Enabled(substate) => substate.take_state().exit(),
//                            TopSubstate::None => unreachable!(),
//                        };

//                        match exit_res {
//                            Some(new_target) => self.transition(new_target),
//                            None => TransitionResult::MoveUp,
//                        }
//                    }
//                }
//            } else {
//                todo!()
//                //match target {
//                //    SE::
//                //}
//            }
//        }
//    }
//}
