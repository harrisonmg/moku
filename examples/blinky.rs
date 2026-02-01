use blinky::{new_machine, State, STATE_CHART};
use moku::StateMachine;

#[moku::state_machine]
mod blinky {
    use std::time::{Duration, Instant};

    use moku::*;

    #[machine_module]
    mod machine {}
    use machine::{Builder, Machine};
    pub use machine::{State, STATE_CHART};

    pub fn new_machine() -> Machine {
        Builder::new(Top {
            // In the real world, `StateMachine::update` would be called at some regular interval,
            // and this period may be set > 0 to control the blinking frequency.
            blink_period: Duration::ZERO,
        })
        .build()
    }

    pub struct Top {
        blink_period: Duration,
    }

    impl TopState for Top {
        fn init(&mut self) -> impl Into<Next<Self::State>> {
            State::Enabled
        }
    }

    struct Disabled;

    impl Substate<Top> for Disabled {}

    struct Enabled;

    impl Substate<Top> for Enabled {
        fn init(&mut self, _ctx: &mut Self::Context<'_>) -> impl Into<Next<Self::State>> {
            State::LedOn
        }
    }

    struct LedOn {
        entry_time: Instant,
    }

    impl Substate<Enabled> for LedOn {
        fn enter(_ctx: &mut Self::Context<'_>) -> impl Into<Entry<Self::State, Self>> {
            Self {
                entry_time: Instant::now(),
            }
        }

        fn update(&mut self, ctx: &mut Self::Context<'_>) -> impl Into<Next<Self::State>> {
            if self.entry_time.elapsed() >= ctx.top.blink_period {
                Next::Target(State::LedOff)
            } else {
                Next::None
            }
        }
    }

    struct LedOff {
        entry_time: Instant,
    }

    impl Substate<Enabled> for LedOff {
        fn enter(_ctx: &mut Self::Context<'_>) -> impl Into<Entry<Self::State, Self>> {
            Self {
                entry_time: Instant::now(),
            }
        }

        fn update(&mut self, ctx: &mut Self::Context<'_>) -> impl Into<Next<Self::State>> {
            if self.entry_time.elapsed() >= ctx.top.blink_period {
                Next::Target(State::LedOn)
            } else {
                Next::None
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

    println!("\nBlinky state chart:\n\n{}\n", STATE_CHART);

    let mut machine = new_machine();
    println!();

    for line in std::io::stdin().lines() {
        println!();
        match line.unwrap().to_lowercase().as_str() {
            "" => machine.update(),
            "top" => machine.transition(State::Top),
            "disabled" => machine.transition(State::Disabled),
            "enabled" => machine.transition(State::Enabled),
            "ledon" => machine.transition(State::LedOn),
            "ledoff" => machine.transition(State::LedOff),
            _ => println!("unrecognized input, try again"),
        };
        println!();
    }
}
