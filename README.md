# moku
[![Rust](https://github.com/harrisonmg/moku/workflows/Rust/badge.svg)](https://github.com/harrisonmg/moku/actions)
[![Latest version](https://img.shields.io/crates/v/moku.svg)](https://crates.io/crates/moku)
[![Documentation](https://docs.rs/moku/badge.svg)](https://docs.rs/moku)
![License](https://img.shields.io/crates/l/moku.svg)

Moku is a Rust library for creating hierarchical state machines. While it's also useful for creating flat state machines, nested states are a first-class feature of moku.

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

## What is a hierarchical state machine?
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

Blinky has two superstates: `Enabled` and `Disabled`. When in the `Enabled` state, it cycles between two substates, `LedOn` and `LedOff`, which results in a blinking LED when enabled.

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

    // Moku has generated `BlinkyState`, an enum of all states.
    use machine::BlinkyState;

    // Every moku state machine must have a single `TopState`, which acts as a
    // superstate to all other states. The state machine never leaves this state.
    struct Top;

    // The top state is indicated by implementing the `TopState` trait for a struct.
    impl TopState<BlinkyState> for Top {}
}
```

Moku will generate the following public items inside of the `machine` module:
- The enum `BlinkyState` that implements [StateEnum]
- The struct `BlinkyMachine` that implements [StateMachine] and [StateRef] for every state
- The struct `BlinkyMachineBuilder` that implements [StateMachineBuilder]
- The `const` `&str` `BLINKY_STATE_CHART`

The `Blinky` name that prepends each of these items is automatically chosen based on the parent module, but can be manually specified as an argument to the [state_machine] attribute.

Let's add some more states inside the `blinky` module:
```rust
# // NOTE: The lines prefixed with `#` below should be hidden with rustdoc.
# //       Visit https://docs.rs/moku instead for your viewing pleasure.
# #[moku::state_machine]
# mod blinky {
#     use moku::*;
#     #[machine_module]
#     mod machine {}
#     use machine::BlinkyState;
#     struct Top;
#     impl TopState<BlinkyState> for Top {}
    // ...

    struct Disabled;

    // Every `State` must use the `superstate` attribute to indicate what state
    // it is a substate of.
    #[superstate(Top)]
    impl State<BlinkyState> for Disabled {}

    struct Enabled;

    #[superstate(Top)]
    impl State<BlinkyState> for Enabled {}

    struct LedOn;

    #[superstate(Enabled)]
    impl State<BlinkyState> for LedOn {}

    struct LedOff;

    #[superstate(Enabled)]
    impl State<BlinkyState> for LedOff {}

    // ...
# }
```

At this point, `BLINKY_STATE_CHART` will look like:
```txt
Top
├─ Disabled
└─ Enabled
   ├─ LedOn
   └─ LedOff
```

and `BlinkyState` will look like:
```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BlinkyState {
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
#     use machine::BlinkyState;
    // ...

    struct Top {
        // Store the blink time setting in the top state, since we may want to configure
        // it for a given instance of the state machine.
        //
        // Since the `TopState` is always active, this value will persist for the lifetime
        // of the state machine.
        blink_time: std::time::Duration,
    }

    impl TopState<BlinkyState> for Top {
        // By implementing the `init` method, we can define the initial transition taken
        // after transitioning into a state.
        //
        // Like most other methods in the `TopState` and `State` traits, the return value
        // indicates a state to transition to, where `None` indicates no transition.
        fn init(&mut self) -> Option<BlinkyState> {
            // When we transition into the `Top` state (or initialize the state machine),
            // transition into the `Enabled` state.
            Some(BlinkyState::Enabled)
        }
    }

    // ...
#     struct Disabled;
#     #[superstate(Top)]
#     impl State<BlinkyState> for Disabled {}
#     struct Enabled;

    #[superstate(Top)]
    impl State<BlinkyState> for Enabled {
        fn init(
            &mut self,
            _superstates: &mut Self::Superstates<'_>,
        ) -> Option<BlinkyState> {
            // When we transition into the `Enabled` state, transition into the `LedOn` state.
            Some(BlinkyState::LedOn)
        }
    }

    struct LedOn {
        entry_time: std::time::Instant,
    }

    #[superstate(Enabled)]
    impl State<BlinkyState> for LedOn {
        // The `enter` method acts as a constructor for the state when it becomes active.
        // States do not persist when they are inactive.
        //
        // If unimplemented, moku will autogenerate this method for states with no fields.
        //
        // The `StateEntry` return type also allows for a transition away instead of
        // entering the state - for instance towards a fault state if some aspect of
        // state construction fails.
        fn enter(
            _superstates: &mut Self::Superstates<'_>,
        ) -> StateEntry<Self, BlinkyState> {
            // pseudocode to turn the LED on
            // led_gpio.set_high()

            StateEntry::State(Self {
                entry_time: std::time::Instant::now(),
            })
        }

        // Moku automatically defines the `Superstates` associated type for each state.
        // This type will contain a mutable reference to each active superstate.
        fn update(
            &mut self,
            superstates: &mut Self::Superstates<'_>,
        ) -> Option<BlinkyState> {
            // We can use `superstates` to access the `blink_time` field of the `Top` state.
            if self.entry_time.elapsed() >= superstates.top.blink_time {
                // If we've met or exceeded the blink time, transition to the `LedOff` state.
                Some(BlinkyState::LedOff)
            } else {
                // Otherwise, don't transition away from this state.
                None
            }
        }
    }

    struct LedOff {
        entry_time: std::time::Instant,
    }

    #[superstate(Enabled)]
    impl State<BlinkyState> for LedOff {
        fn enter(
            _superstates: &mut Self::Superstates<'_>,
        ) -> StateEntry<Self, BlinkyState> {
            // pseudocode to turn the LED off
            // led_gpio.set_low()

            StateEntry::State(Self {
                entry_time: std::time::Instant::now(),
            })
        }

        fn update(
            &mut self,
            superstates: &mut Self::Superstates<'_>,
        ) -> Option<BlinkyState> {
            if self.entry_time.elapsed() >= superstates.top.blink_time {
                // If we've met or exceeded the blink time, transition to the `LedOn` state.
                Some(BlinkyState::LedOn)
            } else {
                // Otherwise, don't transition away from this state.
                None
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
#     use machine::BlinkyState;
#    pub struct Top { pub blink_time: std::time::Duration }
#    impl TopState<BlinkyState> for Top {
#        fn init(&mut self) -> Option<BlinkyState> {
#            Some(BlinkyState::Enabled)
#        }
#    }
#     struct Disabled;
#     #[superstate(Top)]
#     impl State<BlinkyState> for Disabled {}
#     struct Enabled;
#     #[superstate(Top)]
#     impl State<BlinkyState> for Enabled {
#         fn init(
#             &mut self,
#             _superstates: &mut Self::Superstates<'_>,
#         ) -> Option<BlinkyState> {
#             Some(BlinkyState::LedOn)
#         }
#     }
#     struct LedOn { entry_time: std::time::Instant }
#     #[superstate(Enabled)]
#     impl State<BlinkyState> for LedOn {
#         fn enter(
#             _superstates: &mut Self::Superstates<'_>,
#         ) -> StateEntry<Self, BlinkyState> {
#             StateEntry::State(Self {
#                 entry_time: std::time::Instant::now(),
#             })
#         }
#         fn update(
#             &mut self,
#             superstates: &mut Self::Superstates<'_>,
#         ) -> Option<BlinkyState> {
#             if self.entry_time.elapsed() >= superstates.top.blink_time {
#                 Some(BlinkyState::LedOff)
#             } else {
#                 None
#             }
#         }
#     }
#     pub struct LedOff { pub entry_time: std::time::Instant }
#     #[superstate(Enabled)]
#     impl State<BlinkyState> for LedOff {
#         fn enter(
#             _superstates: &mut Self::Superstates<'_>,
#         ) -> StateEntry<Self, BlinkyState> {
#             StateEntry::State(Self {
#                 entry_time: std::time::Instant::now(),
#             })
#         }
#         fn update(
#             &mut self,
#             superstates: &mut Self::Superstates<'_>,
#         ) -> Option<BlinkyState> {
#             if self.entry_time.elapsed() >= superstates.top.blink_time {
#                 Some(BlinkyState::LedOn)
#             } else {
#                 None
#             }
#         }
#     }
# }
// ...

use moku::{StateMachine, StateMachineBuilder};
use blinky::{machine::{BlinkyMachineBuilder, BlinkyState}, Top};

let top_state = Top {
    blink_time: std::time::Duration::ZERO,
};

let mut machine = BlinkyMachineBuilder::new(top_state).build();

// log output:
// ----------
// Blinky: Initial transition to Enabled
// │Entering Enabled
// │Initial transition to LedOn
// │Entering LedOn
// └Transition complete

// `state_matches(...)` will match with any active state or superstate.
assert!(machine.state_matches(BlinkyState::Enabled));
assert!(machine.state_matches(BlinkyState::LedOn));

// `state()` returns the exact current state.
assert!(matches!(machine.state(), BlinkyState::LedOn));

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
machine.transition(BlinkyState::Disabled);

// log output:
// ----------
// Blinky: Transitioning from LedOff to Disabled
// │Exiting LedOff
// │Exiting Enabled
// │Entering Disabled
// └Transition complete

```
If a transition occurs during an update or top-down update, the update will continue from the nearest common ancestor between the previous state and the new state. See [StateMachine::update] and [StateMachine::top_down_update] for more details.

## Event handling
It is common to implement state machines alongside an event type, where each active state handles events as they are generated. Often state machine transitions are defined via a centralized table of states and events. Moku focuses on the autogeneration of state machine boilerplate, leaving event queues and handling for users to implement at their discretion.

Some basic examples of event-based state machines are included in the `examples/` directory.

## Warning
Moku exposes the [internal] module, the contents of which are intended to be used only by the code that is generated by moku. This, in addition to the methods defined in the [TopState] and [State] traits, are not intended to be called by users.

## Macro expansion
Should you wish to view the fully expanded code generated by moku, the [cargo-expand](https://crates.io/crates/cargo-expand) crate may prove useful.
