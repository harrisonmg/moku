# moku
[![Rust](https://github.com/harrisonmg/moku/workflows/Rust/badge.svg)](https://github.com/harrisonmg/moku/actions)
[![Latest version](https://img.shields.io/crates/v/moku.svg)](https://crates.io/crates/moku)
[![Documentation](https://docs.rs/moku/badge.svg)](https://docs.rs/moku)
![License](https://img.shields.io/crates/l/moku.svg)

Moku is a Rust library for creating hierarchical state machines.

While it's also useful for creating flat state machines, nested states are a first-class feature.

Though it may be _impure_ to store data inside of states, in practice it is often useful for states to hold some data (e.g. file handles, perf counters, etc) and to allow substates and external code to access that data. This is also a core feature of moku.

## Features
- Autogeneration of boilerplate, including
    * A full state list
    * A state tree diagram
    * A state machine type
    * A state machine builder type
- Mutable access to active states from both outside and within the state machine
- Proc macros that emit useful compiler errors
- No dynamic memory allocation
- Minimal stack memory usage
- Logging of state machine actions through the Rust `log` API
- `no_std` support

## Shortcomings
Because moku generates a tree of sum types to represent the state machine, states must be `Sized` and do not support generic parameters (though generics can be used behind type aliases).

## What is an HSM?
A hierarchical state machine (HSM) is a type of finite state machine where states can be nested inside of other states. Common functionalities between substates, such as state entry and exit actions, can be grouped by implementing them for the superstate. Beyond the convenient programming implications of HSMs, they often provide a more logical way of modeling systems.

A classic HSM example is blinky, a state machine that - when enabled - blinks some LED on and off:
```text
┌─Enabled─────────────────┐    ┌─Disabled───┐
│                         ├───►│            │
│       ┌─LedOn───┐       │    │            │
│    ┌─►│         ├──┐    │◄───┤            │
│    │  └─────────┘  │    │    └────────────┘
│    │               │    │
│    │  ┌─LedOff──┐  │    │
│    └──┤         │◄─┘    │
│       └─────────┘       │
│                         │
└─────────────────────────┘
```

Blinky has two superstates: `Enabled` and `Disabled`. When in the `Enabled` state, it cycles between two substates, `LedOn` and `LedOff`, which results in a blinking LED.

## Usage
The simplest possible moku state machine can be defined as follows:
```rust
// The `state_machine` attribute indicates to moku that the `blinky` module contains
// state definitions and an empty module for it to generate the state machine within.
#[moku::state_machine]
mod blinky {
    use moku::*;

    // The `machine_module` attribute marks the module moku will use for autogeneration.
    #[machine_module]
    mod machine {}

    // Moku has generated `State`, an enum of all states.
    use machine::State;

    // Every moku state machine must have a single `TopState`, which acts as a
    // superstate to all other states. The state machine never leaves this state.
    struct Top;

    // The top state is indicated by implementing the `TopState` trait for a struct.
    impl TopState for Top {}
}
```

Moku will generate the following public items inside of the `machine` module:
- The enum `State` that implements [`StateEnum`]
- The struct `Machine` that implements [`StateMachine`] and [`StateRef`] for every state
- The struct `Builder` that implements [`StateMachineBuilder`]
- The `const` `&str` `STATE_CHART`

Let's add some more states inside of the `blinky` module:
```rust
# // NOTE: The lines prefixed with `#` below should be hidden with rustdoc.
# //       Visit https://docs.rs/moku instead for your viewing pleasure.
# #[moku::state_machine]
# mod blinky {
#     use moku::*;
#     #[machine_module]
#     mod machine {}
#     use machine::State;
#     struct Top;
#     impl TopState for Top {}
    // ...

    struct Disabled;

    // Every substate must implement `Substate` with its superstate as the generic parameter.
    impl Substate<Top> for Disabled {}

    struct Enabled;

    impl Substate<Top> for Enabled {}

    struct LedOn;

    impl Substate<Enabled> for LedOn {}

    struct LedOff;

    impl Substate<Enabled> for LedOff {}

    // ...
# }
```

At this point, `STATE_CHART` will look like:
```txt
Top
├─ Disabled
└─ Enabled
   ├─ LedOn
   └─ LedOff
```

and `State` will look like:
```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum State {
    Top,
    Disabled,
    Enabled,
    LedOn,
    LedOff,
}
```

Let's add some functionality to our states:
```rust
# // NOTE: The lines prefixed with `#` below should be hidden with rustdoc.
# //       Visit https://docs.rs/moku instead for your viewing pleasure.
# #[moku::state_machine]
# mod blinky {
#     use moku::*;
#     #[machine_module]
#     mod machine {}
#     use machine::State;
    // ...

    struct Top {
        // Store the blink time setting in the top state; we may want to configure it on a
        // per-instance basis.
        //
        // Because the `TopState` is always active, this value will persist for the lifetime
        // of the state machine.
        blink_time: std::time::Duration,
    }

    impl TopState for Top {
        // By implementing the `init` method, we can define the initial transition taken
        // after transitioning into a state.
        //
        // Like most other methods in the `TopState` and `Substate` traits, the return value
        // indicates a state to transition to, where `Next::None` indicates no transition.
        fn init(&mut self) -> impl Into<Next<Self::State>> {
            // When we transition into the `Top` state (or initialize the state machine),
            // transition into the `Enabled` state.
            State::Enabled
        }
    }

    // ...
#     struct Disabled;
#     impl Substate<Top> for Disabled {}
#     struct Enabled;

    impl Substate<Top> for Enabled {
        fn init(
            &mut self,
            _ctx: &mut Self::Context<'_>,
        ) -> impl Into<Next<Self::State>> {
            // When we transition into the `Enabled` state, transition into the `LedOn` state.
            State::LedOn
        }
    }

    struct LedOn {
        entry_time: std::time::Instant,
    }

    impl Substate<Enabled> for LedOn {
        // The `enter` method acts as a constructor for the state when it becomes active.
        // States do not persist when they are inactive.
        //
        // If unimplemented, moku will autogenerate this method for states with no fields.
        //
        // The `Entry` return type also allows for a transition away instead of
        // entering the state - for instance towards a fault state if some aspect of
        // state construction fails.
        fn enter(
            _ctx: &mut Self::Context<'_>,
        ) -> impl Into<Entry<Self::State, Self>> {
            // dummy code to turn the LED on
            // led_gpio.set_high()

            Self {
                entry_time: std::time::Instant::now(),
            }
        }

        // Moku automatically defines the `Context` associated type for each state.
        // This type will contain a mutable reference to each active superstate.
        fn update(
            &mut self,
            ctx: &mut Self::Context<'_>,
        ) -> impl Into<Next<Self::State>> {
            // We can use `ctx` to access the `blink_time` field of the `Top` state.
            if self.entry_time.elapsed() >= ctx.top.blink_time {
                // If we've met or exceeded the blink time, transition to the `LedOff` state.
                Next::Target(State::LedOff)
            } else {
                // Otherwise, don't transition away from this state.
                Next::None
            }
        }
    }

    struct LedOff {
        entry_time: std::time::Instant,
    }

    impl Substate<Enabled> for LedOff {
        fn enter(
            _ctx: &mut Self::Context<'_>,
        ) -> impl Into<Entry<Self::State, Self>> {
            // dummy code to turn the LED off
            // led_gpio.set_low()

            Self {
                entry_time: std::time::Instant::now(),
            }
        }

        fn update(
            &mut self,
            context: &mut Self::Context<'_>,
        ) -> impl Into<Next<Self::State>> {
            if self.entry_time.elapsed() >= context.top.blink_time {
                // If we've met or exceeded the blink time, transition to the `LedOn` state.
                Next::Target(State::LedOn)
            } else {
                // Otherwise, don't transition away from this state.
                Next::None
            }
        }
    }

    // ...
# }
```

Finally, let's use our state machine!
```rust
# // NOTE: The lines prefixed with `#` below should be hidden with rustdoc.
# //       Visit https://docs.rs/moku instead for your viewing pleasure.
# #[moku::state_machine]
# mod blinky {
#     use moku::*;
#     #[machine_module]
#     pub mod machine {}
#     use machine::State;
#    pub struct Top { pub blink_time: std::time::Duration }
#    impl TopState for Top {
#        fn init(&mut self) -> impl Into<Next<Self::State>> {
#            State::Enabled
#        }
#    }
#     struct Disabled;
#     impl Substate<Top> for Disabled {}
#     struct Enabled;
#     impl Substate<Top> for Enabled {
#         fn init(
#             &mut self,
#             _ctx: &mut Self::Context<'_>,
#         ) -> impl Into<Next<Self::State>> {
#             State::LedOn
#         }
#     }
#     struct LedOn { entry_time: std::time::Instant }
#     impl Substate<Enabled> for LedOn {
#         fn enter(
#             _ctx: &mut Self::Context<'_>,
#         ) -> impl Into<Entry<Self::State, Self>> {
#             Self {
#                 entry_time: std::time::Instant::now(),
#             }
#         }
#         fn update(
#             &mut self,
#             context: &mut Self::Context<'_>,
#         ) -> impl Into<Next<Self::State>> {
#             if self.entry_time.elapsed() >= context.top.blink_time {
#                 Next::Target(State::LedOff)
#             } else {
#                 Next::None
#             }
#         }
#     }
#     pub struct LedOff { pub entry_time: std::time::Instant }
#     impl Substate<Enabled> for LedOff {
#         fn enter(
#             _ctx: &mut Self::Context<'_>,
#         ) -> impl Into<Entry<Self::State, Self>> {
#             Self {
#                 entry_time: std::time::Instant::now(),
#             }
#         }
#         fn update(
#             &mut self,
#             context: &mut Self::Context<'_>,
#         ) -> impl Into<Next<Self::State>> {
#             if self.entry_time.elapsed() >= context.top.blink_time {
#                 Next::Target(State::LedOn)
#             } else {
#                 Next::None
#             }
#         }
#     }
# }
// ...

use moku::{StateMachine, StateMachineBuilder};
use blinky::{machine::{Builder, State}, Top};

let top_state = Top {
    blink_time: std::time::Duration::ZERO,
};

// The builder type let's us make a new state machine from a top state.
// The state machine is initialized upon building.
let mut machine = Builder::new(top_state).build();

// log output:
// ----------
// Blinky: Initial transition to Enabled
// │Entering Enabled
// │Initial transition to LedOn
// │Entering LedOn
// └Transition complete

// `state_matches(...)` will match with any active state or superstate.
assert!(machine.state_matches(State::Enabled));
assert!(machine.state_matches(State::LedOn));

// `state()` returns the exact current state.
assert!(matches!(machine.state(), State::LedOn));

// `update()` calls each state's `update()` method, starting from the deepest state.
machine.update();

// log output:
// ----------
// Blinky: Updating
// │Updating LedOn
// │Transitioning from LedOn to LedOff
// ││Exiting LedOn
// ││Entering LedOff
// │└Transition complete
// │Updating Enabled
// │Updating Top
// └Update complete

// `top_down_update()` calls each state's `top_down_update()` method,
// starting from the `TopState`.
machine.top_down_update();

// log output:
// ----------
// Blinky: Top-down updating
// │Top-down updating Top
// │Top-down updating Enabled
// │Top-down updating LedOff
// └Top-down update complete

// We have access to the `TopState` at all times.
dbg!(machine.top_ref().blink_time);
machine.top_mut().blink_time = std::time::Duration::from_secs(1);

// We can access currently active states through the `StateRef` trait.
use moku::StateRef;

let led_off: &blinky::LedOff = machine.state_ref().unwrap();
dbg!(led_off.entry_time);

let mut led_off: &mut blinky::LedOff = machine.state_mut().unwrap();
led_off.entry_time = std::time::Instant::now();

// We can manually induce transitions.
machine.transition(State::Disabled);

// log output:
// ----------
// Blinky: Transitioning from LedOff to Disabled
// │Exiting LedOff
// │Exiting Enabled
// │Entering Disabled
// └Transition complete

```
If a transition occurs during an update or top-down update, the update will continue from the nearest common ancestor between the previous state and the new state. See [`StateMachine::update`] and [`StateMachine::top_down_update`] for more details.

An interactive example of blinky can be found in the [`examples/`](https://github.com/harrisonmg/moku/tree/main/examples) directory. Try it out with:
```text
cargo run --example blinky
```

## Events
Moku state machines can optionally handle events of a user-specified type. Events are handled by each active state, starting from the deepest state.
```rust
#[moku::state_machine]
mod example {
    use moku::*;

    #[machine_module]
    mod machine {}
    use machine::*;

    enum Event {
        A,
        B,
        C,
    }

    // `StateMachineEvent` must be implemented on the event type.
    impl StateMachineEvent for Event {}

    struct Top;

    // The proc macro auto-detects the Event type from the StateMachineEvent impl.
    impl TopState for Top {
        fn handle_event(&mut self, event: &Self::Event) -> impl Into<Next<Self::State>> {
            match event {
                Event::A => Next::Target(State::Foo), // Transition to the Foo state.
                Event::B => Next::Target(State::Bar), // Transition to the Bar state.
                Event::C => Next::None, // Do nothing.
            }
        }
    }

    struct Foo;

    impl Substate<Top> for Foo {
        // The default implementation of `handle_event` simply defers all
        // events to the next highest state.
    }

    struct Bar;

    impl Substate<Top> for Bar {
        fn handle_event(
            &mut self,
            _ctx: &mut Self::Context<'_>,
            event: &Self::Event,
        ) -> impl Into<Response<Self::State>> {
            match event {
                Event::A => {
                    // Do nothing with this event and pass handling to the next highest state.
                    Response::Next(Next::None)
                }
                Event::B => {
                    // Do nothing and stop event handling immediately.
                    Response::Drop
                }
                Event::C => {
                    // Transitition to the Foo state and stop event handling.
                    // Response implements `From` for `StateEnum`, `Option<StateEnum>`,
                    // and `Next<StateEnum>` for convenience.
                    Response::Next(Next::Target(State::Foo))
                }
            }
        }
    }
}
```

## Warning
Moku exposes the [`internal`] module, the contents of which are intended to be used only by the code that is generated by moku. This, in addition to the methods defined in the [`TopState`] and [`Substate`] traits, are not intended to be called by users.

## Macro expansion
Should you wish to view the fully expanded code generated by moku, the [`cargo-expand`](https://crates.io/crates/cargo-expand) crate may prove useful.
