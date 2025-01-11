use std::collections::HashMap;

use quote::format_ident;
use syn::{Ident, ItemMod};

use crate::visitor::build_metadata;

#[derive(Debug, Clone)]
pub struct State {
    ident: Ident,
    node: Ident,
    substate_enum: Ident,
    superstates_enum: Ident,
    children: Vec<State>,
    autogen_enter: bool,
}

impl From<&Ident> for State {
    fn from(ident: &Ident) -> Self {
        Self {
            ident: ident.clone(),
            node: format_ident!("{ident}Node"),
            substate_enum: format_ident!("{ident}Substate"),
            superstates_enum: format_ident!("{ident}Superstates"),
            children: Vec::new(),
            autogen_enter: false,
        }
    }
}

impl State {
    /// Add a child to this State or one of its descendents. Returns the child if no parent is
    /// found.
    fn add_child(&mut self, mut child: State, parent: &Ident) -> Option<State> {
        if self.ident == *parent {
            self.children.push(child);
            None
        } else {
            for state in &mut self.children {
                if let Some(reject) = state.add_child(child, parent) {
                    child = reject;
                } else {
                    return None;
                }
            }

            Some(child)
        }
    }

    /// Generate a simple text state chart of this State and its children.
    fn state_chart(&self) -> String {
        format!("{}", self.ident) + &self.state_chart_children(0)
    }

    /// Helper function for formatting the children of a `state_chart`.
    fn state_chart_children(&self, level: usize) -> String {
        if let Some((last, firsts)) = self.children.split_last() {
            let mut acc = String::new();
            for child in firsts {
                acc += &child.state_chart_acc(level + 1, false);
            }
            acc + &last.state_chart_acc(level + 1, true)
        } else {
            String::new()
        }
    }

    /// Helper function for recursively formatting `state_chart`.
    fn state_chart_acc(&self, level: usize, last: bool) -> String {
        let pad = " ".repeat(level * 3);
        let vert = if last { '\u{02514}' } else { '\u{0251C}' };
        format!("\n{pad}{vert}\u{02500} {}", self.ident) + &self.state_chart_children(level)
    }
}

#[derive(Debug)]
pub struct Metadata {
    pub name: Ident,
    pub top_state: State,
    pub states: HashMap<Ident, State>,
    pub machine_mod: Ident,
}

impl Metadata {
    /// Add a state machine state
    pub fn add_state(&mut self, ident: &Ident, autogen_enter: bool) {
        let mut state: State = ident.into();
        state.autogen_enter = autogen_enter;
        self.states.insert(ident.clone(), state);
    }

    /// Add a parent-child relation to the state graph, while detecting state graph cycles.
    pub fn add_relation(&mut self, parent: &Ident, child: &Ident) -> Result<(), syn::Error> {
        let mut child = self.states.remove(child).unwrap();

        for state in self.states.values_mut().chain([&mut self.top_state]) {
            if let Some(reject) = state.add_child(child, parent) {
                child = reject;
            } else {
                return Ok(());
            }
        }

        // we didn't find the parent, this is a cycle
        Err(syn::Error::new(
            parent.span(),
            format!(
                "state graph cycle detected in children of {parent}:\n{parent}{}",
                child.state_chart_acc(0, true),
            ),
        ))
    }
}
