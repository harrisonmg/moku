use std::rc::{Rc, Weak};

use quote::format_ident;
use syn::{Ident, ItemMod};

use crate::visitor::build_metadata;

#[derive(Clone)]
pub struct State {
    ident: Ident,
    node: Ident,
    substate_enum: Ident,
    superstates_enum: Ident,
    parent: Weak<State>,
    children: Vec<Rc<State>>,
    autogen_enter: bool,
}

impl From<&Ident> for State {
    fn from(ident: &Ident) -> Self {
        Self {
            ident: ident.clone(),
            node: format_ident!("{ident}Node"),
            substate_enum: format_ident!("{ident}Substate"),
            superstates_enum: format_ident!("{ident}Superstates"),
            parent: Weak::new(),
            children: Vec::new(),
            autogen_enter: false,
        }
    }
}

pub struct Metadata {
    pub name: Ident,
    pub top_state: State,
    pub states: Vec<State>,
    pub machine_mod: Ident,
}

impl Metadata {
    /// Create Metadata based off of a state_machine module and the state machine's name.
    pub fn new(name: Ident, module: ItemMod) -> Result<Self, syn::Error> {
        build_metadata(name, &module).and_then(Metadata::finalize)
    }

    /// Add a state machine state
    pub fn add_state(&mut self, ident: &Ident, autogen_enter: bool) {
        let mut state: State = ident.into();
        state.autogen_enter = autogen_enter;
        self.states.push(state);
    }

    pub fn add_relation(&mut self, parent: &Ident, child: &Ident) -> Result<(), syn::Error> {
        todo!()
    }

    /// Validate that there are no dead branches in the state machine.
    fn finalize(self) -> Result<Self, syn::Error> {
        todo!()
    }
}
