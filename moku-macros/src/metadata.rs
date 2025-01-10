use syn::Ident;

pub struct State {
    ident: Ident,
    node: Ident,
    substate_enum: Ident,
    superstates_enum: Ident,
    ancestors: Vec<State>,
    children: Vec<State>,
}

pub struct Metadata {
    states: Vec<State>,
    top_state: State,
}
