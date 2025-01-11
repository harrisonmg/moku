//#[crate::state_machine]
//mod blinky {
//    use std::time::{Duration, Instant};

//    use crate::*;

//    #[machine_module]
//    mod state_machine {}

//    struct Top;

//    impl TopState<BlinkyState> for Top {}

//    struct Disabled;

//    #[superstate(Top)]
//    impl State<BlinkyState> for Disabled {}

//    struct Enabled {
//        blink_period: Duration,
//    }

//    #[superstate(Top)]
//    impl State<BlinkyState> for Enabled {
//        fn enter<'a>(superstates: &mut Self::Superstates<'a>) -> StateEntry<Self, BlinkyState> {
//            StateEntry::State(Self {
//                blink_period: Duration::from_secs(2),
//            })
//        }
//    }

//    struct LedOn {
//        entry_time: Instant,
//    }

//    #[superstate(Enabled)]
//    impl State<BlinkyState> for LedOn {
//        fn enter<'a>(superstates: &mut Self::Superstates<'a>) -> StateEntry<Self, BlinkyState> {
//            StateEntry::State(Self {
//                entry_time: Instant::now(),
//            })
//        }

//        fn update<'a>(&mut self, superstates: &mut Self::Superstates<'a>) -> Option<BlinkyState> {
//            if (self.entry_time.elapsed() >= superstates.enabled.blink_period) {
//                Some(BlinkyState::LedOff)
//            } else {
//                None
//            }
//        }
//    }

//    struct LedOff {
//        entry_time: Instant,
//    }

//    #[superstate(Enabled)]
//    impl State<BlinkyState> for LedOff {
//        fn enter<'a>(superstates: &mut Self::Superstates<'a>) -> StateEntry<Self, BlinkyState> {
//            StateEntry::State(Self {
//                entry_time: Instant::now(),
//            })
//        }

//        fn update<'a>(&mut self, superstates: &mut Self::Superstates<'a>) -> Option<BlinkyState> {
//            if (self.entry_time.elapsed() >= superstates.enabled.blink_period) {
//                Some(BlinkyState::LedOn)
//            } else {
//                None
//            }
//        }
//    }
//}
