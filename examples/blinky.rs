use blinky::{new_machine, BlinkyState, BLINKY_STATE_CHART};
use moku::StateMachine;

#[moku::state_machine]
mod blinky {
    use std::time::{Duration, Instant};

    use moku::*;

    #[machine_module]
    mod state_machine {}
    use state_machine::{BlinkyMachine, BlinkyMachineBuilder};
    pub use state_machine::{BlinkyState, BLINKY_STATE_CHART};

    pub fn new_machine() -> BlinkyMachine {
        BlinkyMachineBuilder::new(Top {}).build()
    }

    pub struct Top;

    impl TopState<BlinkyState> for Top {
        fn init(&mut self) -> Option<BlinkyState> {
            Some(BlinkyState::Disabled)
        }
    }

    struct Disabled;

    #[superstate(Top)]
    impl State<BlinkyState> for Disabled {}

    struct Enabled {
        blink_period: Duration,
    }

    #[superstate(Top)]
    impl State<BlinkyState> for Enabled {
        fn enter(_superstates: &mut Self::Superstates<'_>) -> StateEntry<Self, BlinkyState> {
            StateEntry::State(Self {
                // In the real world, `StateMachine::update` would be called at some regular interval,
                // and this period may be set > 0 to control the blinking frequency.
                blink_period: Duration::from_secs(0),
            })
        }

        fn init(&mut self, _superstates: &mut Self::Superstates<'_>) -> Option<BlinkyState> {
            Some(BlinkyState::LedOn)
        }
    }

    struct LedOn {
        entry_time: Instant,
    }

    #[superstate(Enabled)]
    impl State<BlinkyState> for LedOn {
        fn enter(_superstates: &mut Self::Superstates<'_>) -> StateEntry<Self, BlinkyState> {
            StateEntry::State(Self {
                entry_time: Instant::now(),
            })
        }

        fn update(&mut self, superstates: &mut Self::Superstates<'_>) -> Option<BlinkyState> {
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
        fn enter(_superstates: &mut Self::Superstates<'_>) -> StateEntry<Self, BlinkyState> {
            StateEntry::State(Self {
                entry_time: Instant::now(),
            })
        }

        fn update(&mut self, superstates: &mut Self::Superstates<'_>) -> Option<BlinkyState> {
            if self.entry_time.elapsed() >= superstates.enabled.blink_period {
                Some(BlinkyState::LedOn)
            } else {
                None
            }
        }
    }
}

fn main() {
    env_logger::builder()
        .filter_level(log::LevelFilter::Info)
        .init();

    println!("\nWelcome to the blinky state machine example!");
    println!("If enabled, blinky will switch between the LedOn and LedOff states when updated.\n");

    println!("- Type nothing and press enter to run a single update of the state machine");
    println!("- Type a state name and press enter to manually transition into that state");

    println!("\nBlinky state chart:\n\n{}\n", BLINKY_STATE_CHART);

    let mut machine = new_machine();
    println!("");

    for line in std::io::stdin().lines() {
        println!("");
        match line.unwrap().to_lowercase().as_str() {
            "" => machine.update(),
            "disabled" => machine.transition(BlinkyState::Disabled),
            "enabled" => machine.transition(BlinkyState::Enabled),
            "ledon" => machine.transition(BlinkyState::LedOn),
            "ledoff" => machine.transition(BlinkyState::LedOff),
            _ => println!("unrecognized input, try again"),
        };
        println!("");
    }
}
