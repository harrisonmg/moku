// NOTE(harrisonmg): These tests were generated with Claude. After reviewing manually, I've decided that I might as well keep them in the test suite. At the time of writing, all functionality is covered with hand-written tests and I intend to continue doing so, as it's part of my development process.

//! Comprehensive test suite for moku hierarchical state machine library.
//! Tests edge cases, complex hierarchies, and various transition scenarios.

#![allow(clippy::upper_case_acronyms)]

use moku::*;

// =============================================================================
// Test 1: Deep Nesting (5 levels)
// =============================================================================

#[state_machine]
mod deep {
    use moku::*;

    #[machine_module]
    pub mod machine {}
    use machine::DeepState;

    pub struct Top {
        pub depth_visited: Vec<u8>,
    }

    impl TopState<DeepState> for Top {
        fn update(&mut self) -> impl Into<Next<DeepState>> {
            self.depth_visited.push(0);
            None
        }
    }

    pub struct L1 {
        pub entered: bool,
    }

    #[superstate(Top)]
    impl State<DeepState> for L1 {
        fn enter(superstates: &mut Self::Superstates<'_>) -> StateEntry<DeepState, Self> {
            superstates.top.depth_visited.push(1);
            Self { entered: true }.into()
        }

        fn update(
            &mut self,
            superstates: &mut Self::Superstates<'_>,
        ) -> impl Into<Next<DeepState>> {
            superstates.top.depth_visited.push(1);
            None
        }
    }

    pub struct L2;

    #[superstate(L1)]
    impl State<DeepState> for L2 {
        fn enter(superstates: &mut Self::Superstates<'_>) -> StateEntry<DeepState, Self> {
            superstates.top.depth_visited.push(2);
            Self.into()
        }

        fn update(
            &mut self,
            superstates: &mut Self::Superstates<'_>,
        ) -> impl Into<Next<DeepState>> {
            superstates.top.depth_visited.push(2);
            None
        }
    }

    pub struct L3;

    #[superstate(L2)]
    impl State<DeepState> for L3 {
        fn enter(superstates: &mut Self::Superstates<'_>) -> StateEntry<DeepState, Self> {
            superstates.top.depth_visited.push(3);
            Self.into()
        }

        fn update(
            &mut self,
            superstates: &mut Self::Superstates<'_>,
        ) -> impl Into<Next<DeepState>> {
            superstates.top.depth_visited.push(3);
            None
        }
    }

    pub struct L4;

    #[superstate(L3)]
    impl State<DeepState> for L4 {
        fn enter(superstates: &mut Self::Superstates<'_>) -> StateEntry<DeepState, Self> {
            superstates.top.depth_visited.push(4);
            Self.into()
        }

        fn update(
            &mut self,
            superstates: &mut Self::Superstates<'_>,
        ) -> impl Into<Next<DeepState>> {
            superstates.top.depth_visited.push(4);
            None
        }
    }

    pub struct L5;

    #[superstate(L4)]
    impl State<DeepState> for L5 {
        fn enter(superstates: &mut Self::Superstates<'_>) -> StateEntry<DeepState, Self> {
            superstates.top.depth_visited.push(5);
            Self.into()
        }

        fn update(
            &mut self,
            superstates: &mut Self::Superstates<'_>,
        ) -> impl Into<Next<DeepState>> {
            superstates.top.depth_visited.push(5);
            None
        }
    }
}

mod deep_nesting_tests {
    use super::*;
    use deep::{machine::*, *};

    #[test]
    fn state_chart() {
        assert_eq!(
            DEEP_STATE_CHART,
            "Top
└─ L1
   └─ L2
      └─ L3
         └─ L4
            └─ L5"
        );
    }

    #[test]
    fn deep_transition_enters_all_levels() {
        let mut machine = DeepMachineBuilder::new(Top {
            depth_visited: vec![],
        })
        .build();

        machine.transition(DeepState::L5);
        assert!(matches!(machine.state(), DeepState::L5));

        // Should have entered L1, L2, L3, L4, L5 in order
        assert_eq!(machine.top_ref().depth_visited, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn deep_update_order() {
        let mut machine = DeepMachineBuilder::new(Top {
            depth_visited: vec![],
        })
        .build();

        machine.transition(DeepState::L5);
        machine.top_mut().depth_visited.clear();

        machine.update();

        // Bottom-up: L5, L4, L3, L2, L1, Top
        assert_eq!(machine.top_ref().depth_visited, vec![5, 4, 3, 2, 1, 0]);
    }

    #[test]
    fn deep_state_matches() {
        let mut machine = DeepMachineBuilder::new(Top {
            depth_visited: vec![],
        })
        .build();

        machine.transition(DeepState::L5);

        assert!(machine.state_matches(DeepState::Top));
        assert!(machine.state_matches(DeepState::L1));
        assert!(machine.state_matches(DeepState::L2));
        assert!(machine.state_matches(DeepState::L3));
        assert!(machine.state_matches(DeepState::L4));
        assert!(machine.state_matches(DeepState::L5));
    }

    #[test]
    fn deep_state_refs() {
        let mut machine = DeepMachineBuilder::new(Top {
            depth_visited: vec![],
        })
        .build();

        machine.transition(DeepState::L5);

        let l1: &L1 = machine.state_ref().unwrap();
        assert!(l1.entered);

        let _l2: &L2 = machine.state_ref().unwrap();
        let _l3: &L3 = machine.state_ref().unwrap();
        let _l4: &L4 = machine.state_ref().unwrap();
        let _l5: &L5 = machine.state_ref().unwrap();
    }
}

// =============================================================================
// Test 2: Complex Sibling/Cousin Transitions
// =============================================================================

#[state_machine]
mod complex {
    use moku::*;

    #[machine_module]
    pub mod machine {}
    use machine::ComplexState;

    pub struct Top {
        pub transition_log: Vec<String>,
    }

    impl TopState<ComplexState> for Top {}

    // Branch A
    pub struct A;

    #[superstate(Top)]
    impl State<ComplexState> for A {
        fn enter(superstates: &mut Self::Superstates<'_>) -> StateEntry<ComplexState, Self> {
            superstates.top.transition_log.push("enter A".to_string());
            Self.into()
        }

        fn exit(self, superstates: &mut Self::Superstates<'_>) -> impl Into<Next<ComplexState>> {
            superstates.top.transition_log.push("exit A".to_string());
            None
        }
    }

    pub struct A1;

    #[superstate(A)]
    impl State<ComplexState> for A1 {
        fn enter(superstates: &mut Self::Superstates<'_>) -> StateEntry<ComplexState, Self> {
            superstates.top.transition_log.push("enter A1".to_string());
            Self.into()
        }

        fn exit(self, superstates: &mut Self::Superstates<'_>) -> impl Into<Next<ComplexState>> {
            superstates.top.transition_log.push("exit A1".to_string());
            None
        }
    }

    pub struct A2;

    #[superstate(A)]
    impl State<ComplexState> for A2 {
        fn enter(superstates: &mut Self::Superstates<'_>) -> StateEntry<ComplexState, Self> {
            superstates.top.transition_log.push("enter A2".to_string());
            Self.into()
        }

        fn exit(self, superstates: &mut Self::Superstates<'_>) -> impl Into<Next<ComplexState>> {
            superstates.top.transition_log.push("exit A2".to_string());
            None
        }
    }

    // Branch B
    pub struct B;

    #[superstate(Top)]
    impl State<ComplexState> for B {
        fn enter(superstates: &mut Self::Superstates<'_>) -> StateEntry<ComplexState, Self> {
            superstates.top.transition_log.push("enter B".to_string());
            Self.into()
        }

        fn exit(self, superstates: &mut Self::Superstates<'_>) -> impl Into<Next<ComplexState>> {
            superstates.top.transition_log.push("exit B".to_string());
            None
        }
    }

    pub struct B1;

    #[superstate(B)]
    impl State<ComplexState> for B1 {
        fn enter(superstates: &mut Self::Superstates<'_>) -> StateEntry<ComplexState, Self> {
            superstates.top.transition_log.push("enter B1".to_string());
            Self.into()
        }

        fn exit(self, superstates: &mut Self::Superstates<'_>) -> impl Into<Next<ComplexState>> {
            superstates.top.transition_log.push("exit B1".to_string());
            None
        }
    }

    pub struct B2;

    #[superstate(B)]
    impl State<ComplexState> for B2 {
        fn enter(superstates: &mut Self::Superstates<'_>) -> StateEntry<ComplexState, Self> {
            superstates.top.transition_log.push("enter B2".to_string());
            Self.into()
        }

        fn exit(self, superstates: &mut Self::Superstates<'_>) -> impl Into<Next<ComplexState>> {
            superstates.top.transition_log.push("exit B2".to_string());
            None
        }
    }
}

mod complex_transitions_tests {
    use super::*;
    use complex::{machine::*, *};

    #[test]
    fn state_chart() {
        assert_eq!(
            COMPLEX_STATE_CHART,
            "Top
├─ A
│  ├─ A1
│  └─ A2
└─ B
   ├─ B1
   └─ B2"
        );
    }

    #[test]
    fn sibling_transition() {
        let mut machine = ComplexMachineBuilder::new(Top {
            transition_log: vec![],
        })
        .build();

        machine.transition(ComplexState::A1);
        machine.top_mut().transition_log.clear();

        // Transition from A1 to A2 (siblings)
        machine.transition(ComplexState::A2);

        assert_eq!(
            machine.top_ref().transition_log,
            vec!["exit A1", "enter A2"]
        );
    }

    #[test]
    fn cousin_transition() {
        let mut machine = ComplexMachineBuilder::new(Top {
            transition_log: vec![],
        })
        .build();

        machine.transition(ComplexState::A1);
        machine.top_mut().transition_log.clear();

        // Transition from A1 to B1 (cousins - different parent branches)
        machine.transition(ComplexState::B1);

        assert_eq!(
            machine.top_ref().transition_log,
            vec!["exit A1", "exit A", "enter B", "enter B1"]
        );
    }

    #[test]
    fn deep_to_shallow_transition() {
        let mut machine = ComplexMachineBuilder::new(Top {
            transition_log: vec![],
        })
        .build();

        machine.transition(ComplexState::A1);
        machine.top_mut().transition_log.clear();

        // Transition from A1 to B (shallow)
        machine.transition(ComplexState::B);

        assert_eq!(
            machine.top_ref().transition_log,
            vec!["exit A1", "exit A", "enter B"]
        );
    }

    #[test]
    fn shallow_to_deep_transition() {
        let mut machine = ComplexMachineBuilder::new(Top {
            transition_log: vec![],
        })
        .build();

        machine.transition(ComplexState::A);
        machine.top_mut().transition_log.clear();

        // Transition from A to B2 (deeper)
        machine.transition(ComplexState::B2);

        assert_eq!(
            machine.top_ref().transition_log,
            vec!["exit A", "enter B", "enter B2"]
        );
    }
}

// =============================================================================
// Test 3: Superstates Data Access and Mutation
// =============================================================================

#[state_machine]
mod data {
    use moku::*;

    #[machine_module]
    pub mod machine {}
    use machine::DataState;

    pub struct Top {
        pub counter: u32,
    }

    impl TopState<DataState> for Top {}

    pub struct Parent {
        pub multiplier: u32,
    }

    #[superstate(Top)]
    impl State<DataState> for Parent {
        fn enter(superstates: &mut Self::Superstates<'_>) -> StateEntry<DataState, Self> {
            // Access Top's counter during enter
            let val = superstates.top.counter;
            Self {
                multiplier: val * 2,
            }
            .into()
        }
    }

    pub struct Child;

    #[superstate(Parent)]
    impl State<DataState> for Child {
        fn update(
            &mut self,
            superstates: &mut Self::Superstates<'_>,
        ) -> impl Into<Next<DataState>> {
            // Modify both parent and top state
            superstates.top.counter += 1;
            superstates.parent.multiplier *= 2;
            None
        }
    }

    pub struct Sibling;

    #[superstate(Parent)]
    impl State<DataState> for Sibling {
        fn update(
            &mut self,
            superstates: &mut Self::Superstates<'_>,
        ) -> impl Into<Next<DataState>> {
            // Different modification
            superstates.top.counter += 10;
            superstates.parent.multiplier += 1;
            None
        }
    }
}

mod superstate_data_tests {
    use super::*;
    use data::{machine::*, *};

    #[test]
    fn superstate_access_in_enter() {
        let mut machine = DataMachineBuilder::new(Top { counter: 5 }).build();

        machine.transition(DataState::Parent);

        let parent: &Parent = machine.state_ref().unwrap();
        assert_eq!(parent.multiplier, 10); // 5 * 2
    }

    #[test]
    fn superstate_mutation_in_update() {
        let mut machine = DataMachineBuilder::new(Top { counter: 5 }).build();

        machine.transition(DataState::Child);

        let parent: &Parent = machine.state_ref().unwrap();
        assert_eq!(parent.multiplier, 10); // 5 * 2 from enter

        machine.update();

        assert_eq!(machine.top_ref().counter, 6); // 5 + 1
        let parent: &Parent = machine.state_ref().unwrap();
        assert_eq!(parent.multiplier, 20); // 10 * 2
    }

    #[test]
    fn sibling_keeps_parent_state() {
        let mut machine = DataMachineBuilder::new(Top { counter: 5 }).build();

        machine.transition(DataState::Child);
        machine.update(); // counter = 6, multiplier = 20

        // Transition to sibling - Parent is NOT re-entered (common ancestor stays)
        machine.transition(DataState::Sibling);

        // Parent was NOT re-entered, so multiplier stays at 20
        let parent: &Parent = machine.state_ref().unwrap();
        assert_eq!(parent.multiplier, 20);

        machine.update();
        assert_eq!(machine.top_ref().counter, 16); // 6 + 10
        let parent: &Parent = machine.state_ref().unwrap();
        assert_eq!(parent.multiplier, 21); // 20 + 1
    }
}

// =============================================================================
// Test 4: Initial Transition Chains
// =============================================================================

#[state_machine]
mod chains {
    use moku::*;

    #[machine_module]
    pub mod machine {}
    use machine::ChainsState;

    pub struct Top;

    impl TopState<ChainsState> for Top {
        fn init(&mut self) -> impl Into<Next<ChainsState>> {
            Some(ChainsState::A)
        }
    }

    struct A;

    #[superstate(Top)]
    impl State<ChainsState> for A {
        fn init(
            &mut self,
            _superstates: &mut Self::Superstates<'_>,
        ) -> impl Into<Next<ChainsState>> {
            Some(ChainsState::AA)
        }
    }

    struct AA;

    #[superstate(A)]
    impl State<ChainsState> for AA {
        fn init(
            &mut self,
            _superstates: &mut Self::Superstates<'_>,
        ) -> impl Into<Next<ChainsState>> {
            Some(ChainsState::AAA)
        }
    }

    struct AAA;

    #[superstate(AA)]
    impl State<ChainsState> for AAA {}

    struct B;

    #[superstate(Top)]
    impl State<ChainsState> for B {
        fn init(
            &mut self,
            _superstates: &mut Self::Superstates<'_>,
        ) -> impl Into<Next<ChainsState>> {
            // Init transitions to a different branch!
            Some(ChainsState::A)
        }
    }
}

mod init_chains_tests {
    use super::*;
    use chains::{machine::*, *};

    #[test]
    fn chained_init_transitions() {
        let machine = ChainsMachineBuilder::new(Top).build();

        // Top -> A -> AA -> AAA through init chain
        assert!(matches!(machine.state(), ChainsState::AAA));
    }

    #[test]
    fn init_to_different_branch() {
        let mut machine = ChainsMachineBuilder::new(Top).build();

        // Transition to B, which will init to A, which chains to AAA
        machine.transition(ChainsState::B);

        assert!(matches!(machine.state(), ChainsState::AAA));
    }
}

// =============================================================================
// Test 5: Short-Circuit Transitions
// =============================================================================

#[state_machine]
mod circuit {
    use moku::*;

    #[machine_module]
    pub mod machine {}
    use machine::CircuitState;

    pub struct Top {
        pub log: Vec<String>,
    }

    impl TopState<CircuitState> for Top {}

    struct Normal;

    #[superstate(Top)]
    impl State<CircuitState> for Normal {}

    struct EnterShortCircuit;

    #[superstate(Top)]
    impl State<CircuitState> for EnterShortCircuit {
        fn enter(superstates: &mut Self::Superstates<'_>) -> StateEntry<CircuitState, Self> {
            superstates
                .top
                .log
                .push("enter EnterShortCircuit".to_string());
            StateEntry::Target(CircuitState::Normal)
        }
    }

    struct ExitShortCircuit;

    #[superstate(Top)]
    impl State<CircuitState> for ExitShortCircuit {
        fn enter(superstates: &mut Self::Superstates<'_>) -> StateEntry<CircuitState, Self> {
            superstates
                .top
                .log
                .push("enter ExitShortCircuit".to_string());
            Self.into()
        }

        fn exit(self, superstates: &mut Self::Superstates<'_>) -> impl Into<Next<CircuitState>> {
            superstates
                .top
                .log
                .push("exit ExitShortCircuit".to_string());
            Some(CircuitState::Normal)
        }
    }

    struct Target;

    #[superstate(Top)]
    impl State<CircuitState> for Target {
        fn enter(superstates: &mut Self::Superstates<'_>) -> StateEntry<CircuitState, Self> {
            superstates.top.log.push("enter Target".to_string());
            Self.into()
        }
    }

    // For testing chained short circuits
    struct Chain1;

    #[superstate(Top)]
    impl State<CircuitState> for Chain1 {
        fn enter(_superstates: &mut Self::Superstates<'_>) -> StateEntry<CircuitState, Self> {
            StateEntry::Target(CircuitState::Chain2)
        }
    }

    struct Chain2;

    #[superstate(Top)]
    impl State<CircuitState> for Chain2 {
        fn enter(_superstates: &mut Self::Superstates<'_>) -> StateEntry<CircuitState, Self> {
            StateEntry::Target(CircuitState::Chain3)
        }
    }

    struct Chain3;

    #[superstate(Top)]
    impl State<CircuitState> for Chain3 {}
}

mod short_circuit_tests {
    use super::*;
    use circuit::{machine::*, *};

    #[test]
    fn enter_short_circuit() {
        let mut machine = CircuitMachineBuilder::new(Top { log: vec![] }).build();

        machine.transition(CircuitState::EnterShortCircuit);

        // Should end up in Normal, not EnterShortCircuit
        assert!(matches!(machine.state(), CircuitState::Normal));
        assert!(machine
            .top_ref()
            .log
            .contains(&"enter EnterShortCircuit".to_string()));
    }

    #[test]
    fn exit_short_circuit() {
        let mut machine = CircuitMachineBuilder::new(Top { log: vec![] }).build();

        machine.transition(CircuitState::ExitShortCircuit);
        assert!(matches!(machine.state(), CircuitState::ExitShortCircuit));

        machine.top_mut().log.clear();
        machine.transition(CircuitState::Target);

        // Exit of ExitShortCircuit redirects to Normal
        assert!(matches!(machine.state(), CircuitState::Normal));
        assert!(machine
            .top_ref()
            .log
            .contains(&"exit ExitShortCircuit".to_string()));
    }

    #[test]
    fn chained_short_circuits() {
        let mut machine = CircuitMachineBuilder::new(Top { log: vec![] }).build();

        machine.transition(CircuitState::Chain1);

        // Chain1 -> Chain2 -> Chain3
        assert!(matches!(machine.state(), CircuitState::Chain3));
    }
}

// =============================================================================
// Test 6: Events with Complex Hierarchies
// =============================================================================

#[state_machine]
mod evt {
    use moku::*;

    #[machine_module]
    pub mod machine {}
    use machine::EvtState;

    #[derive(Debug, Clone, PartialEq)]
    pub enum Event {
        GoToA,
        GoToB,
        GoToDeep,
        Handled,
        Unhandled,
    }

    impl StateMachineEvent for Event {}

    pub struct Top {
        pub handled_by: Option<String>,
    }

    impl TopState<EvtState, Event> for Top {
        fn handle_event(&mut self, event: &Event) -> impl Into<Next<EvtState>> {
            self.handled_by = Some("Top".to_string());
            match event {
                Event::GoToA => Some(EvtState::A),
                Event::GoToB => Some(EvtState::B),
                _ => None,
            }
        }
    }

    pub struct A;

    #[superstate(Top)]
    impl State<EvtState, Event> for A {}

    pub struct AA;

    #[superstate(A)]
    impl State<EvtState, Event> for AA {
        fn handle_event(
            &mut self,
            event: &Event,
            superstates: &mut Self::Superstates<'_>,
        ) -> impl Into<EventResponse<EvtState>> {
            match event {
                Event::Handled => {
                    superstates.top.handled_by = Some("AA".to_string());
                    EventResponse::Drop
                }
                Event::GoToDeep => {
                    superstates.top.handled_by = Some("AA".to_string());
                    EvtState::BB.into()
                }
                _ => None.into(), // Defer to superstate
            }
        }
    }

    pub struct B;

    #[superstate(Top)]
    impl State<EvtState, Event> for B {}

    pub struct BB;

    #[superstate(B)]
    impl State<EvtState, Event> for BB {
        fn handle_event(
            &mut self,
            event: &Event,
            superstates: &mut Self::Superstates<'_>,
        ) -> impl Into<EventResponse<EvtState>> {
            match event {
                Event::Handled => {
                    superstates.top.handled_by = Some("BB".to_string());
                    EventResponse::Drop
                }
                _ => None.into(),
            }
        }
    }
}

mod complex_events_tests {
    use super::*;
    use evt::{machine::*, *};

    #[test]
    fn event_bubbles_to_top() {
        let mut machine = EvtMachineBuilder::new(Top { handled_by: None }).build();

        machine.transition(EvtState::AA);
        machine.handle_event(&Event::Unhandled);

        // AA doesn't handle Unhandled, so it bubbles to Top
        assert_eq!(machine.top_ref().handled_by, Some("Top".to_string()));
    }

    #[test]
    fn event_handled_by_deep_state() {
        let mut machine = EvtMachineBuilder::new(Top { handled_by: None }).build();

        machine.transition(EvtState::AA);
        machine.handle_event(&Event::Handled);

        assert_eq!(machine.top_ref().handled_by, Some("AA".to_string()));
    }

    #[test]
    fn event_triggers_cross_branch_transition() {
        let mut machine = EvtMachineBuilder::new(Top { handled_by: None }).build();

        machine.transition(EvtState::AA);
        machine.handle_event(&Event::GoToDeep);

        assert!(matches!(machine.state(), EvtState::BB));
        assert_eq!(machine.top_ref().handled_by, Some("AA".to_string()));
    }

    #[test]
    fn event_handled_at_top_causes_transition() {
        let mut machine = EvtMachineBuilder::new(Top { handled_by: None }).build();

        machine.transition(EvtState::BB);
        machine.handle_event(&Event::GoToA);

        assert!(matches!(machine.state(), EvtState::A));
    }
}

// =============================================================================
// Test 7: StateEnum Trait Implementations
// =============================================================================

#[state_machine]
mod traits {
    use moku::*;

    #[machine_module]
    pub mod machine {}
    use machine::TraitsState;

    pub struct Top;
    impl TopState<TraitsState> for Top {}

    struct A;
    #[superstate(Top)]
    impl State<TraitsState> for A {}

    struct B;
    #[superstate(Top)]
    impl State<TraitsState> for B {}
}

mod state_enum_traits_tests {
    use super::traits::machine::*;

    #[test]
    fn state_enum_debug() {
        let state = TraitsState::A;
        let debug_str = format!("{:?}", state);
        assert_eq!(debug_str, "A");
    }

    #[test]
    fn state_enum_clone() {
        let state = TraitsState::A;
        #[allow(clippy::clone_on_copy)]
        let cloned = state.clone();
        assert_eq!(state, cloned);
    }

    #[test]
    fn state_enum_copy() {
        let state = TraitsState::A;
        let copied: TraitsState = state; // Copy
        assert_eq!(state, copied);
    }

    #[test]
    fn state_enum_eq() {
        assert_eq!(TraitsState::A, TraitsState::A);
        assert_ne!(TraitsState::A, TraitsState::B);
        assert_ne!(TraitsState::A, TraitsState::Top);
    }
}

// =============================================================================
// Test 8: Next and EventResponse From Implementations
// =============================================================================

#[state_machine]
mod froms {
    use moku::*;

    #[machine_module]
    pub mod machine {}
    use machine::FromsState;

    pub struct Top;
    impl TopState<FromsState> for Top {}

    struct A;
    #[superstate(Top)]
    impl State<FromsState> for A {}
}

mod from_impls_tests {
    use super::*;
    use froms::machine::*;

    #[test]
    fn next_from_state_enum() {
        let next: Next<FromsState> = FromsState::A.into();
        assert!(matches!(next, Next::Target(FromsState::A)));
    }

    #[test]
    fn next_from_some() {
        let next: Next<FromsState> = Some(FromsState::A).into();
        assert!(matches!(next, Next::Target(FromsState::A)));
    }

    #[test]
    fn next_from_none() {
        let next: Next<FromsState> = None.into();
        assert!(matches!(next, Next::None));
    }

    #[test]
    fn event_response_from_state_enum() {
        let resp: EventResponse<FromsState> = FromsState::A.into();
        assert!(matches!(
            resp,
            EventResponse::Next(Next::Target(FromsState::A))
        ));
    }

    #[test]
    fn event_response_from_some() {
        let resp: EventResponse<FromsState> = Some(FromsState::A).into();
        assert!(matches!(
            resp,
            EventResponse::Next(Next::Target(FromsState::A))
        ));
    }

    #[test]
    fn event_response_from_none() {
        let resp: EventResponse<FromsState> = None.into();
        assert!(matches!(resp, EventResponse::Next(Next::None)));
    }

    #[test]
    fn event_response_from_next() {
        let next = Next::Target(FromsState::A);
        let resp: EventResponse<FromsState> = next.into();
        assert!(matches!(
            resp,
            EventResponse::Next(Next::Target(FromsState::A))
        ));

        let next = Next::ExactTarget(FromsState::A);
        let resp: EventResponse<FromsState> = next.into();
        assert!(matches!(
            resp,
            EventResponse::Next(Next::ExactTarget(FromsState::A))
        ));
    }
}

// =============================================================================
// Test 9: Wide State Machines (Many Siblings)
// =============================================================================

#[state_machine]
mod wide {
    use moku::*;

    #[machine_module]
    pub mod machine {}
    use machine::WideState;

    pub struct Top {
        pub visit_count: u32,
    }

    impl TopState<WideState> for Top {}

    struct S1;
    #[superstate(Top)]
    impl State<WideState> for S1 {}

    struct S2;
    #[superstate(Top)]
    impl State<WideState> for S2 {}

    struct S3;
    #[superstate(Top)]
    impl State<WideState> for S3 {}

    struct S4;
    #[superstate(Top)]
    impl State<WideState> for S4 {}

    struct S5;
    #[superstate(Top)]
    impl State<WideState> for S5 {}

    struct S6;
    #[superstate(Top)]
    impl State<WideState> for S6 {}

    struct S7;
    #[superstate(Top)]
    impl State<WideState> for S7 {}

    struct S8;
    #[superstate(Top)]
    impl State<WideState> for S8 {}
}

mod wide_machine_tests {
    use super::*;
    use wide::{machine::*, *};

    #[test]
    fn wide_state_chart() {
        assert_eq!(
            WIDE_STATE_CHART,
            "Top
├─ S1
├─ S2
├─ S3
├─ S4
├─ S5
├─ S6
├─ S7
└─ S8"
        );
    }

    #[test]
    fn cycle_through_all_states() {
        let mut machine = WideMachineBuilder::new(Top { visit_count: 0 }).build();

        let states = [
            WideState::S1,
            WideState::S2,
            WideState::S3,
            WideState::S4,
            WideState::S5,
            WideState::S6,
            WideState::S7,
            WideState::S8,
        ];

        for state in states {
            machine.transition(state);
            assert_eq!(machine.state(), state);
            machine.top_mut().visit_count += 1;
        }

        assert_eq!(machine.top_ref().visit_count, 8);
    }
}

// =============================================================================
// Test 10: Update Continues After Mid-Update Transition
// =============================================================================

#[state_machine]
mod cont {
    use moku::*;

    #[machine_module]
    pub mod machine {}
    use machine::ContState;

    pub struct Top {
        pub update_log: Vec<String>,
    }

    impl TopState<ContState> for Top {
        fn update(&mut self) -> impl Into<Next<ContState>> {
            self.update_log.push("Top update".to_string());
            None
        }
    }

    pub struct A;

    #[superstate(Top)]
    impl State<ContState> for A {
        fn update(
            &mut self,
            superstates: &mut Self::Superstates<'_>,
        ) -> impl Into<Next<ContState>> {
            superstates.top.update_log.push("A update".to_string());
            None
        }
    }

    pub struct AA;

    #[superstate(A)]
    impl State<ContState> for AA {
        fn update(
            &mut self,
            superstates: &mut Self::Superstates<'_>,
        ) -> impl Into<Next<ContState>> {
            superstates.top.update_log.push("AA update".to_string());
            // Transition to B during update
            Some(ContState::B)
        }
    }

    pub struct B;

    #[superstate(Top)]
    impl State<ContState> for B {
        fn update(
            &mut self,
            superstates: &mut Self::Superstates<'_>,
        ) -> impl Into<Next<ContState>> {
            superstates.top.update_log.push("B update".to_string());
            None
        }
    }
}

mod update_continuation_tests {
    use super::*;
    use cont::{machine::*, *};

    #[test]
    fn update_continues_from_common_ancestor() {
        let mut machine = ContMachineBuilder::new(Top { update_log: vec![] }).build();

        machine.transition(ContState::AA);
        machine.top_mut().update_log.clear();

        machine.update();

        // AA triggers transition to B
        // Update continues from common ancestor (Top)
        let log = &machine.top_ref().update_log;
        assert!(log.contains(&"AA update".to_string()));
        assert!(log.contains(&"Top update".to_string()));
        // A should NOT be updated since we transitioned away before reaching it
        assert!(!log.contains(&"A update".to_string()));
    }
}

// =============================================================================
// Test 11: Exact Transitions (ExactTarget)
// =============================================================================

#[state_machine]
mod exact {
    use moku::*;

    #[machine_module]
    pub mod machine {}
    use machine::ExactState;

    pub struct Top {
        pub init_count: u32,
    }

    impl TopState<ExactState> for Top {
        fn init(&mut self) -> impl Into<Next<ExactState>> {
            self.init_count += 1;
            None
        }
    }

    pub struct A {
        pub init_count: u32,
        pub enter_count: u32,
    }

    #[superstate(Top)]
    impl State<ExactState> for A {
        fn enter(_superstates: &mut Self::Superstates<'_>) -> StateEntry<ExactState, Self> {
            Self {
                init_count: 0,
                enter_count: 1,
            }
            .into()
        }

        fn init(
            &mut self,
            _superstates: &mut Self::Superstates<'_>,
        ) -> impl Into<Next<ExactState>> {
            self.init_count += 1;
            None
        }
    }

    pub struct AA {
        pub enter_count: u32,
    }

    #[superstate(A)]
    impl State<ExactState> for AA {
        fn enter(_superstates: &mut Self::Superstates<'_>) -> StateEntry<ExactState, Self> {
            Self { enter_count: 1 }.into()
        }
    }
}

mod exact_transitions_tests {
    use super::*;
    use exact::{machine::*, *};

    #[test]
    fn exact_transition_to_active_superstate() {
        let mut machine = ExactMachineBuilder::new(Top { init_count: 0 }).build();

        machine.transition(ExactState::AA);

        let a: &A = machine.state_ref().unwrap();
        assert_eq!(a.enter_count, 1);
        assert_eq!(a.init_count, 0); // Not init'd because AA was the target

        let aa: &AA = machine.state_ref().unwrap();
        assert_eq!(aa.enter_count, 1);

        // Normal transition to A (already active) - no change
        machine.transition(ExactState::A);
        assert!(matches!(machine.state(), ExactState::AA)); // Still in AA

        let a: &A = machine.state_ref().unwrap();
        assert_eq!(a.enter_count, 1); // Not re-entered

        // Exact transition to A - forces re-entry
        machine.exact_transition(ExactState::A);
        assert!(matches!(machine.state(), ExactState::A)); // Now in A

        let a: &A = machine.state_ref().unwrap();
        assert_eq!(a.enter_count, 1); // Fresh enter
        assert_eq!(a.init_count, 1); // Init was called
    }

    #[test]
    fn exact_transition_to_top() {
        let mut machine = ExactMachineBuilder::new(Top { init_count: 0 }).build();
        assert_eq!(machine.top_ref().init_count, 1);

        machine.transition(ExactState::A);

        // Normal transition to Top - no effect (already in Top hierarchy)
        machine.transition(ExactState::Top);
        assert!(matches!(machine.state(), ExactState::A));
        assert_eq!(machine.top_ref().init_count, 1);

        // Exact transition to Top - reinitializes
        machine.exact_transition(ExactState::Top);
        assert!(matches!(machine.state(), ExactState::Top));
        assert_eq!(machine.top_ref().init_count, 2);
    }
}

// =============================================================================
// Test 12: StateEntry Variants
// =============================================================================

#[state_machine]
mod entry {
    use moku::*;

    #[machine_module]
    pub mod machine {}
    use machine::EntryState;

    pub struct Top;
    impl TopState<EntryState> for Top {}

    struct Normal;
    #[superstate(Top)]
    impl State<EntryState> for Normal {}

    struct RedirectOnEnter;
    #[superstate(Top)]
    impl State<EntryState> for RedirectOnEnter {
        fn enter(_superstates: &mut Self::Superstates<'_>) -> StateEntry<EntryState, Self> {
            StateEntry::Target(EntryState::Normal)
        }
    }

    struct ExactRedirectOnEnter;
    #[superstate(Top)]
    impl State<EntryState> for ExactRedirectOnEnter {
        fn enter(_superstates: &mut Self::Superstates<'_>) -> StateEntry<EntryState, Self> {
            StateEntry::ExactTarget(EntryState::Top)
        }
    }

    pub struct Counter {
        pub value: u32,
    }

    #[superstate(Top)]
    impl State<EntryState> for Counter {
        fn enter(_superstates: &mut Self::Superstates<'_>) -> StateEntry<EntryState, Self> {
            StateEntry::State(Self { value: 42 })
        }
    }
}

mod state_entry_tests {
    use super::*;
    use entry::{machine::*, *};

    #[test]
    fn state_entry_state() {
        let mut machine = EntryMachineBuilder::new(Top).build();

        machine.transition(EntryState::Counter);
        let counter: &Counter = machine.state_ref().unwrap();
        assert_eq!(counter.value, 42);
    }

    #[test]
    fn state_entry_target() {
        let mut machine = EntryMachineBuilder::new(Top).build();

        machine.transition(EntryState::RedirectOnEnter);
        assert!(matches!(machine.state(), EntryState::Normal));
    }

    #[test]
    fn state_entry_exact_target() {
        let mut machine = EntryMachineBuilder::new(Top).build();

        machine.transition(EntryState::Normal);
        machine.transition(EntryState::ExactRedirectOnEnter);

        // ExactTarget to Top should reinitialize Top
        assert!(matches!(machine.state(), EntryState::Top));
    }
}

// =============================================================================
// Test 13: Multiple Machines Independence
// =============================================================================

#[state_machine(MachineA)]
mod machine_a {
    use moku::*;

    #[machine_module]
    pub mod machine {}
    use machine::MachineAState;

    pub struct Top {
        pub value: u32,
    }
    impl TopState<MachineAState> for Top {}

    struct StateA;
    #[superstate(Top)]
    impl State<MachineAState> for StateA {}
}

#[state_machine(MachineB)]
mod machine_b {
    use moku::*;

    #[machine_module]
    pub mod machine {}
    use machine::MachineBState;

    pub struct Top {
        pub value: String,
    }
    impl TopState<MachineBState> for Top {}

    struct StateB;
    #[superstate(Top)]
    impl State<MachineBState> for StateB {}
}

mod multiple_machines_tests {
    use super::*;
    use machine_a::machine::*;
    use machine_b::machine::*;

    #[test]
    fn machines_are_independent() {
        let mut machine_a = MachineAMachineBuilder::new(machine_a::Top { value: 100 }).build();
        let mut machine_b = MachineBMachineBuilder::new(machine_b::Top {
            value: "hello".to_string(),
        })
        .build();

        machine_a.transition(MachineAState::StateA);
        machine_b.transition(MachineBState::StateB);

        assert!(matches!(machine_a.state(), MachineAState::StateA));
        assert!(matches!(machine_b.state(), MachineBState::StateB));

        assert_eq!(machine_a.top_ref().value, 100);
        assert_eq!(machine_b.top_ref().value, "hello");

        machine_a.top_mut().value = 200;
        assert_eq!(machine_a.top_ref().value, 200);
        assert_eq!(machine_b.top_ref().value, "hello"); // Unchanged
    }
}

// =============================================================================
// Test 14: Empty Events (Unit Type)
// =============================================================================

#[state_machine]
mod unit {
    use moku::*;

    #[machine_module]
    pub mod machine {}
    use machine::UnitState;

    pub struct Top {
        pub event_count: u32,
    }

    // Using default () event type
    impl TopState<UnitState> for Top {
        fn handle_event(&mut self, _event: &()) -> impl Into<Next<UnitState>> {
            self.event_count += 1;
            None
        }
    }

    struct A;
    #[superstate(Top)]
    impl State<UnitState> for A {}
}

mod unit_events_tests {
    use super::*;
    use unit::{machine::*, *};

    #[test]
    fn unit_event_handling() {
        let mut machine = UnitMachineBuilder::new(Top { event_count: 0 }).build();

        machine.handle_event(&());
        machine.handle_event(&());
        machine.handle_event(&());

        assert_eq!(machine.top_ref().event_count, 3);
    }
}

// =============================================================================
// Test 15: Transition to Same State (No-op vs Exact)
// =============================================================================

#[state_machine]
mod same {
    use std::{cell::Cell, rc::Rc};

    use moku::*;

    #[machine_module]
    pub mod machine {}
    use machine::SameState;

    pub struct Top;
    impl TopState<SameState> for Top {}

    pub struct Counter {
        pub enter_count: Rc<Cell<u32>>,
        pub exit_count: Rc<Cell<u32>>,
    }

    #[superstate(Top)]
    impl State<SameState> for Counter {
        fn enter(_superstates: &mut Self::Superstates<'_>) -> StateEntry<SameState, Self> {
            Self {
                enter_count: Rc::new(Cell::new(1)),
                exit_count: Rc::new(Cell::new(0)),
            }
            .into()
        }

        fn exit(self, _superstates: &mut Self::Superstates<'_>) -> impl Into<Next<SameState>> {
            self.exit_count.set(self.exit_count.get() + 1);
            None
        }
    }
}

mod same_state_transition_tests {
    use super::*;
    use same::{machine::*, *};

    #[test]
    fn normal_transition_to_self_is_noop() {
        let mut machine = SameMachineBuilder::new(Top).build();

        machine.transition(SameState::Counter);
        let counter: &Counter = machine.state_ref().unwrap();
        let enter_count = counter.enter_count.clone();
        let exit_count = counter.exit_count.clone();

        assert_eq!(enter_count.get(), 1);
        assert_eq!(exit_count.get(), 0);

        // Normal transition to self - no change
        machine.transition(SameState::Counter);

        assert_eq!(enter_count.get(), 1); // Not re-entered
        assert_eq!(exit_count.get(), 0); // Not exited
    }

    #[test]
    fn exact_transition_to_self_reenters() {
        let mut machine = SameMachineBuilder::new(Top).build();

        machine.transition(SameState::Counter);
        let counter: &Counter = machine.state_ref().unwrap();
        let old_exit_count = counter.exit_count.clone();

        assert_eq!(old_exit_count.get(), 0);

        // Exact transition to self - exits and re-enters
        machine.exact_transition(SameState::Counter);

        assert_eq!(old_exit_count.get(), 1); // Old instance was exited

        let counter: &Counter = machine.state_ref().unwrap();
        assert_eq!(counter.enter_count.get(), 1); // New instance entered
    }
}

// =============================================================================
// Test 16: Top-Down Update Transitions
// =============================================================================

#[state_machine]
mod tdu {
    use moku::*;

    #[machine_module]
    pub mod machine {}
    use machine::TduState;

    pub struct Top {
        pub log: Vec<String>,
    }

    impl TopState<TduState> for Top {
        fn top_down_update(&mut self) -> impl Into<Next<TduState>> {
            self.log.push("Top tdu".to_string());
            // Top triggers transition to B
            Some(TduState::B)
        }
    }

    pub struct A;

    #[superstate(Top)]
    impl State<TduState> for A {
        fn top_down_update(
            &mut self,
            superstates: &mut Self::Superstates<'_>,
        ) -> impl Into<Next<TduState>> {
            superstates.top.log.push("A tdu".to_string());
            None
        }
    }

    pub struct B;

    #[superstate(Top)]
    impl State<TduState> for B {
        fn top_down_update(
            &mut self,
            superstates: &mut Self::Superstates<'_>,
        ) -> impl Into<Next<TduState>> {
            superstates.top.log.push("B tdu".to_string());
            None
        }
    }
}

mod top_down_transitions_tests {
    use super::*;
    use tdu::{machine::*, *};

    #[test]
    fn top_triggers_transition_in_tdu() {
        let mut machine = TduMachineBuilder::new(Top { log: vec![] }).build();

        machine.transition(TduState::A);
        machine.top_mut().log.clear();

        machine.top_down_update();

        // Top's tdu runs first and triggers transition to B
        // A should NOT get its tdu called since we transitioned away
        let log = &machine.top_ref().log;
        assert!(log.contains(&"Top tdu".to_string()));
        assert!(!log.contains(&"A tdu".to_string()));
        assert!(log.contains(&"B tdu".to_string()));
        assert!(matches!(machine.state(), TduState::B));
    }
}

// =============================================================================
// Test 17: Event with Exact Transitions
// =============================================================================

#[state_machine]
mod eexact {
    use moku::*;

    #[machine_module]
    pub mod machine {}
    use machine::EexactState;

    #[derive(Debug)]
    pub enum Event {
        ExactToTop,
        NormalToTop,
    }

    impl StateMachineEvent for Event {}

    pub struct Top {
        pub init_count: u32,
    }

    impl TopState<EexactState, Event> for Top {
        fn init(&mut self) -> impl Into<Next<EexactState>> {
            self.init_count += 1;
            None
        }

        fn handle_event(&mut self, event: &Event) -> impl Into<Next<EexactState>> {
            match event {
                Event::ExactToTop => Next::ExactTarget(EexactState::Top),
                Event::NormalToTop => Next::Target(EexactState::Top),
            }
        }
    }

    struct A;
    #[superstate(Top)]
    impl State<EexactState, Event> for A {}
}

mod event_exact_tests {
    use super::*;
    use eexact::{machine::*, *};

    #[test]
    fn event_exact_transition_reinits_top() {
        let mut machine = EexactMachineBuilder::new(Top { init_count: 0 }).build();
        assert_eq!(machine.top_ref().init_count, 1);

        machine.transition(EexactState::A);

        machine.handle_event(&Event::ExactToTop);
        assert_eq!(machine.top_ref().init_count, 2);
        assert!(matches!(machine.state(), EexactState::Top));
    }

    #[test]
    fn event_normal_transition_is_noop_when_in_substate() {
        let mut machine = EexactMachineBuilder::new(Top { init_count: 0 }).build();
        assert_eq!(machine.top_ref().init_count, 1);

        machine.transition(EexactState::A);

        machine.handle_event(&Event::NormalToTop);
        assert_eq!(machine.top_ref().init_count, 1); // Not reinited
        assert!(matches!(machine.state(), EexactState::A)); // Still in A
    }
}
