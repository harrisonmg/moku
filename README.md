# moku
[![Rust](https://github.com/moku/moku/workflows/Rust/badge.svg)](https://github.com/harrisonmg/moku/actions)
[![Latest version](https://img.shields.io/crates/v/moku.svg)](https://crates.io/crates/moku)
[![Documentation](https://docs.rs/moku/badge.svg)](https://docs.rs/moku)
![License](https://img.shields.io/crates/l/moku.svg)

Moku is a Rust library for creating Hierarchical State Machines. While it's also useful for creating flat state machines, state nesting is a first-class feature of moku.

## What is a hierarchical state machine?
A hierarchical state machine (HSM) is a state machine where common functionalities between states, such as state entry and exit actions, can be grouped by modeling them as a superstate with nested substates.

## When would I use moku?
- When you want to avoid writing and testing the boilerplate required to define and run an HSM
- When you need more flexibility
