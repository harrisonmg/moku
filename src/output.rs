use crate::{State, SubState};

#[derive(Default)]
struct Blinky;
impl State<BlinkyState> for Blinky {}

#[derive(Default)]
struct Disabled;
impl State<BlinkyState> for Disabled {}

#[derive(Default)]
struct Enabled;
impl State<BlinkyState> for Enabled {}

#[derive(Default)]
struct LedOn;
impl State<BlinkyState> for LedOn {}

#[derive(Default)]
struct LedOff;
impl State<BlinkyState> for LedOff {}

enum BlinkyState {
    Disabled,
    Enabled,
    LedOn,
    LedOff,
}

enum BlinkySubstate {
    Disabled(Disabled),
    Enabled(Enabled, EnabledSubstate),
}

impl SubState<BlinkyState> for BlinkySubstate {
    fn update(&mut self) -> Option<BlinkyState> {
        match self {
            Self::Disabled(state) => State::update(state),
            Self::Enabled(state, sub_state) => {
                SubState::update(sub_state).or_else(|| State::update(state))
            }
        }
    }
}

enum EnabledSubstate {
    LedOn(LedOn),
    LedOff(LedOff),
}

impl SubState<BlinkyState> for EnabledSubstate {
    fn update(&mut self) -> Option<BlinkyState> {
        match self {
            Self::LedOn(state) => State::update(state),
            Self::LedOff(state) => State::update(state),
        }
    }
}

struct StateMachine {
    top_state: Blinky,
    sub_state: BlinkySubstate,
}

impl StateMachine {
    fn update(&mut self) {
        if let Some(new_state) = self.sub_state.update() {
            self.transition(new_state);
        } else if let Some(new_state) = self.top_state.update() {
            self.transition(new_state);
        }
    }

    fn transition(&mut self, new_state: BlinkyState) {
        self.sub_state.transition(new_state)
    }
}
