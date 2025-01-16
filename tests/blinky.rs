#[moku::state_machine]
mod blinky {
    use std::time::{Duration, Instant};

    use moku::*;

    #[machine_module]
    mod state_machine {}
    use state_machine::BlinkyState;

    struct Top;

    impl TopState<BlinkyState> for Top {}

    struct Disabled;

    #[superstate(Top)]
    impl State<BlinkyState> for Disabled {}

    struct Enabled {
        blink_period: Duration,
    }

    #[superstate(Top)]
    impl State<BlinkyState> for Enabled {
        fn enter<'a>(_superstates: &mut Self::Superstates<'a>) -> StateEntry<Self, BlinkyState> {
            StateEntry::State(Self {
                blink_period: Duration::from_secs(0),
            })
        }
    }

    struct LedOn {
        entry_time: Instant,
    }

    #[superstate(Enabled)]
    impl State<BlinkyState> for LedOn {
        fn enter<'a>(_superstates: &mut Self::Superstates<'a>) -> StateEntry<Self, BlinkyState> {
            StateEntry::State(Self {
                entry_time: Instant::now(),
            })
        }

        fn update<'a>(&mut self, superstates: &mut Self::Superstates<'a>) -> Option<BlinkyState> {
            if self.entry_time.elapsed() >= superstates.enabled.blink_period {
                Some(BlinkyState::LedOff)
            } else {
                None
            }
        }
    }

    struct LedOff {
        entry_time: Instant,
    }

    #[superstate(Enabled)]
    impl State<BlinkyState> for LedOff {
        fn enter<'a>(_superstates: &mut Self::Superstates<'a>) -> StateEntry<Self, BlinkyState> {
            StateEntry::State(Self {
                entry_time: Instant::now(),
            })
        }

        fn update<'a>(&mut self, superstates: &mut Self::Superstates<'a>) -> Option<BlinkyState> {
            if self.entry_time.elapsed() >= superstates.enabled.blink_period {
                Some(BlinkyState::LedOn)
            } else {
                None
            }
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use state_machine::BlinkyMachineBuilder;
        use test_log::test;

        #[test]
        fn basic() {
            println!("{}", state_machine::BLINKY_STATE_CHART);
            let mut machine = BlinkyMachineBuilder::new(Top {}).build();

            let _top: &Top = machine.top_ref();

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
}
