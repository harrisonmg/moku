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

    pub struct Top {
        pub depth_visited: Vec<u8>,
    }

    impl TopState for Top {
        fn update(&mut self) -> impl Into<Next<Self::State>> {
            self.depth_visited.push(0);
        }
    }

    pub struct L1 {
        pub entered: bool,
    }

    impl Substate<Top> for L1 {
        fn enter(ctx: &mut Self::Context<'_>) -> impl Into<Entry<Self::State, Self>> {
            ctx.top.depth_visited.push(1);
            Self { entered: true }
        }

        fn update(&mut self, ctx: &mut Self::Context<'_>) -> impl Into<Next<Self::State>> {
            ctx.top.depth_visited.push(1);
        }
    }

    pub struct L2;

    impl Substate<L1> for L2 {
        fn enter(ctx: &mut Self::Context<'_>) -> impl Into<Entry<Self::State, Self>> {
            ctx.top.depth_visited.push(2);
            Self
        }

        fn update(&mut self, ctx: &mut Self::Context<'_>) -> impl Into<Next<Self::State>> {
            ctx.top.depth_visited.push(2);
        }
    }

    pub struct L3;

    impl Substate<L2> for L3 {
        fn enter(ctx: &mut Self::Context<'_>) -> impl Into<Entry<Self::State, Self>> {
            ctx.top.depth_visited.push(3);
            Self
        }

        fn update(&mut self, ctx: &mut Self::Context<'_>) -> impl Into<Next<Self::State>> {
            ctx.top.depth_visited.push(3);
        }
    }

    pub struct L4;

    impl Substate<L3> for L4 {
        fn enter(ctx: &mut Self::Context<'_>) -> impl Into<Entry<Self::State, Self>> {
            ctx.top.depth_visited.push(4);
            Self
        }

        fn update(&mut self, ctx: &mut Self::Context<'_>) -> impl Into<Next<Self::State>> {
            ctx.top.depth_visited.push(4);
        }
    }

    pub struct L5;

    impl Substate<L4> for L5 {
        fn enter(ctx: &mut Self::Context<'_>) -> impl Into<Entry<Self::State, Self>> {
            ctx.top.depth_visited.push(5);
            Self
        }

        fn update(&mut self, ctx: &mut Self::Context<'_>) -> impl Into<Next<Self::State>> {
            ctx.top.depth_visited.push(5);
        }
    }
}

mod deep_nesting_tests {
    use super::*;
    use deep::machine::{
        Builder as DeepMachineBuilder, State as DeepState, STATE_CHART as DEEP_STATE_CHART,
    };
    use deep::*;

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

    pub struct Top {
        pub transition_log: Vec<String>,
    }

    impl TopState for Top {}

    // Branch A
    pub struct A;

    impl Substate<Top> for A {
        fn enter(ctx: &mut Self::Context<'_>) -> impl Into<Entry<Self::State, Self>> {
            ctx.top.transition_log.push("enter A".to_string());
            Self
        }

        fn exit(self, ctx: &mut Self::Context<'_>) -> impl Into<Next<Self::State>> {
            ctx.top.transition_log.push("exit A".to_string());
        }
    }

    pub struct A1;

    impl Substate<A> for A1 {
        fn enter(ctx: &mut Self::Context<'_>) -> impl Into<Entry<Self::State, Self>> {
            ctx.top.transition_log.push("enter A1".to_string());
            Self
        }

        fn exit(self, ctx: &mut Self::Context<'_>) -> impl Into<Next<Self::State>> {
            ctx.top.transition_log.push("exit A1".to_string());
        }
    }

    pub struct A2;

    impl Substate<A> for A2 {
        fn enter(ctx: &mut Self::Context<'_>) -> impl Into<Entry<Self::State, Self>> {
            ctx.top.transition_log.push("enter A2".to_string());
            Self
        }

        fn exit(self, ctx: &mut Self::Context<'_>) -> impl Into<Next<Self::State>> {
            ctx.top.transition_log.push("exit A2".to_string());
        }
    }

    // Branch B
    pub struct B;

    impl Substate<Top> for B {
        fn enter(ctx: &mut Self::Context<'_>) -> impl Into<Entry<Self::State, Self>> {
            ctx.top.transition_log.push("enter B".to_string());
            Self
        }

        fn exit(self, ctx: &mut Self::Context<'_>) -> impl Into<Next<Self::State>> {
            ctx.top.transition_log.push("exit B".to_string());
        }
    }

    pub struct B1;

    impl Substate<B> for B1 {
        fn enter(ctx: &mut Self::Context<'_>) -> impl Into<Entry<Self::State, Self>> {
            ctx.top.transition_log.push("enter B1".to_string());
            Self
        }

        fn exit(self, ctx: &mut Self::Context<'_>) -> impl Into<Next<Self::State>> {
            ctx.top.transition_log.push("exit B1".to_string());
        }
    }

    pub struct B2;

    impl Substate<B> for B2 {
        fn enter(ctx: &mut Self::Context<'_>) -> impl Into<Entry<Self::State, Self>> {
            ctx.top.transition_log.push("enter B2".to_string());
            Self
        }

        fn exit(self, ctx: &mut Self::Context<'_>) -> impl Into<Next<Self::State>> {
            ctx.top.transition_log.push("exit B2".to_string());
        }
    }
}

mod complex_transitions_tests {
    use super::*;
    use complex::machine::{
        Builder as ComplexMachineBuilder, State as ComplexState, STATE_CHART as COMPLEX_STATE_CHART,
    };
    use complex::*;

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

    pub struct Top {
        pub counter: u32,
    }

    impl TopState for Top {}

    pub struct Parent {
        pub multiplier: u32,
    }

    impl Substate<Top> for Parent {
        fn enter(ctx: &mut Self::Context<'_>) -> impl Into<Entry<Self::State, Self>> {
            // Access Top's counter during enter
            let val = ctx.top.counter;
            Self {
                multiplier: val * 2,
            }
        }
    }

    pub struct Child;

    impl Substate<Parent> for Child {
        fn update(&mut self, ctx: &mut Self::Context<'_>) -> impl Into<Next<Self::State>> {
            // Modify both parent and top state
            ctx.top.counter += 1;
            ctx.parent.multiplier *= 2;
        }
    }

    pub struct Sibling;

    impl Substate<Parent> for Sibling {
        fn update(&mut self, ctx: &mut Self::Context<'_>) -> impl Into<Next<Self::State>> {
            // Different modification
            ctx.top.counter += 10;
            ctx.parent.multiplier += 1;
        }
    }
}

mod superstate_data_tests {
    use super::*;
    use data::machine::{Builder as DataMachineBuilder, State as DataState};
    use data::*;

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
    use machine::State;

    pub struct Top;

    impl TopState for Top {
        fn init(&mut self) -> impl Into<Next<Self::State>> {
            State::A
        }
    }

    struct A;

    impl Substate<Top> for A {
        fn init(&mut self, _ctx: &mut Self::Context<'_>) -> impl Into<Next<Self::State>> {
            State::AA
        }
    }

    struct AA;

    impl Substate<A> for AA {
        fn init(&mut self, _ctx: &mut Self::Context<'_>) -> impl Into<Next<Self::State>> {
            State::AAA
        }
    }

    struct AAA;

    impl Substate<AA> for AAA {}

    struct B;

    impl Substate<Top> for B {
        fn init(&mut self, _ctx: &mut Self::Context<'_>) -> impl Into<Next<Self::State>> {
            // Init transitions to a different branch!
            State::A
        }
    }
}

mod init_chains_tests {
    use super::*;
    use chains::machine::{Builder as ChainsMachineBuilder, State as ChainsState};
    use chains::*;

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
    use machine::State;

    pub struct Top {
        pub log: Vec<String>,
    }

    impl TopState for Top {}

    struct Normal;

    impl Substate<Top> for Normal {}

    struct EnterShortCircuit;

    impl Substate<Top> for EnterShortCircuit {
        fn enter(ctx: &mut Self::Context<'_>) -> impl Into<Entry<Self::State, Self>> {
            ctx.top.log.push("enter EnterShortCircuit".to_string());
            Entry::Target(State::Normal)
        }
    }

    struct ExitShortCircuit;

    impl Substate<Top> for ExitShortCircuit {
        fn enter(ctx: &mut Self::Context<'_>) -> impl Into<Entry<Self::State, Self>> {
            ctx.top.log.push("enter ExitShortCircuit".to_string());
            Self
        }

        fn exit(self, ctx: &mut Self::Context<'_>) -> impl Into<Next<Self::State>> {
            ctx.top.log.push("exit ExitShortCircuit".to_string());
            State::Normal
        }
    }

    struct Target;

    impl Substate<Top> for Target {
        fn enter(ctx: &mut Self::Context<'_>) -> impl Into<Entry<Self::State, Self>> {
            ctx.top.log.push("enter Target".to_string());
            Self
        }
    }

    // For testing chained short circuits
    struct Chain1;

    impl Substate<Top> for Chain1 {
        fn enter(_ctx: &mut Self::Context<'_>) -> impl Into<Entry<Self::State, Self>> {
            Entry::Target(State::Chain2)
        }
    }

    struct Chain2;

    impl Substate<Top> for Chain2 {
        fn enter(_ctx: &mut Self::Context<'_>) -> impl Into<Entry<Self::State, Self>> {
            Entry::Target(State::Chain3)
        }
    }

    struct Chain3;

    impl Substate<Top> for Chain3 {}
}

mod short_circuit_tests {
    use super::*;
    use circuit::machine::{Builder as CircuitMachineBuilder, State as CircuitState};
    use circuit::*;

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
    use machine::State;

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

    impl TopState for Top {
        fn handle_event(&mut self, event: &Self::Event) -> impl Into<Next<Self::State>> {
            self.handled_by = Some("Top".to_string());
            match event {
                Event::GoToA => State::A.into(),
                Event::GoToB => State::B.into(),
                _ => Next::None,
            }
        }
    }

    pub struct A;

    impl Substate<Top> for A {}

    pub struct AA;

    impl Substate<A> for AA {
        fn handle_event(
            &mut self,
            ctx: &mut Self::Context<'_>,
            event: &Self::Event,
        ) -> impl Into<Response<Self::State>> {
            match event {
                Event::Handled => {
                    ctx.top.handled_by = Some("AA".to_string());
                    Response::Drop
                }
                Event::GoToDeep => {
                    ctx.top.handled_by = Some("AA".to_string());
                    State::BB.into()
                }
                _ => None.into(), // Defer to superstate
            }
        }
    }

    pub struct B;

    impl Substate<Top> for B {}

    pub struct BB;

    impl Substate<B> for BB {
        fn handle_event(
            &mut self,
            ctx: &mut Self::Context<'_>,
            event: &Self::Event,
        ) -> impl Into<Response<Self::State>> {
            match event {
                Event::Handled => {
                    ctx.top.handled_by = Some("BB".to_string());
                    Response::Drop
                }
                _ => None.into(),
            }
        }
    }
}

mod complex_events_tests {
    use super::*;
    use evt::machine::{Builder as EvtMachineBuilder, State as EvtState};
    use evt::*;

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

    pub struct Top;
    impl TopState for Top {}

    struct A;
    impl Substate<Top> for A {}

    struct B;
    impl Substate<Top> for B {}
}

mod state_enum_traits_tests {
    use super::traits::machine::State as TraitsState;

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
// Test 8: Next and Response From Implementations
// =============================================================================

#[state_machine]
mod froms {
    use moku::*;

    #[machine_module]
    pub mod machine {}

    pub struct Top;
    impl TopState for Top {}

    struct A;
    impl Substate<Top> for A {}
}

mod from_impls_tests {
    use super::*;
    use froms::machine::State as FromsState;

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
        let resp: Response<FromsState> = FromsState::A.into();
        assert!(matches!(resp, Response::Next(Next::Target(FromsState::A))));
    }

    #[test]
    fn event_response_from_some() {
        let resp: Response<FromsState> = Some(FromsState::A).into();
        assert!(matches!(resp, Response::Next(Next::Target(FromsState::A))));
    }

    #[test]
    fn event_response_from_none() {
        let resp: Response<FromsState> = None.into();
        assert!(matches!(resp, Response::Next(Next::None)));
    }

    #[test]
    fn event_response_from_next() {
        let next = Next::Target(FromsState::A);
        let resp: Response<FromsState> = next.into();
        assert!(matches!(resp, Response::Next(Next::Target(FromsState::A))));

        let next = Next::ExactTarget(FromsState::A);
        let resp: Response<FromsState> = next.into();
        assert!(matches!(
            resp,
            Response::Next(Next::ExactTarget(FromsState::A))
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

    pub struct Top {
        pub visit_count: u32,
    }

    impl TopState for Top {}

    struct S1;
    impl Substate<Top> for S1 {}

    struct S2;
    impl Substate<Top> for S2 {}

    struct S3;
    impl Substate<Top> for S3 {}

    struct S4;
    impl Substate<Top> for S4 {}

    struct S5;
    impl Substate<Top> for S5 {}

    struct S6;
    impl Substate<Top> for S6 {}

    struct S7;
    impl Substate<Top> for S7 {}

    struct S8;
    impl Substate<Top> for S8 {}
}

mod wide_machine_tests {
    use super::*;
    use wide::machine::{
        Builder as WideMachineBuilder, State as WideState, STATE_CHART as WIDE_STATE_CHART,
    };
    use wide::*;

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
    use machine::State;

    pub struct Top {
        pub update_log: Vec<String>,
    }

    impl TopState for Top {
        fn update(&mut self) -> impl Into<Next<Self::State>> {
            self.update_log.push("Top update".to_string());
        }
    }

    pub struct A;

    impl Substate<Top> for A {
        fn update(&mut self, ctx: &mut Self::Context<'_>) -> impl Into<Next<Self::State>> {
            ctx.top.update_log.push("A update".to_string());
        }
    }

    pub struct AA;

    impl Substate<A> for AA {
        fn update(&mut self, ctx: &mut Self::Context<'_>) -> impl Into<Next<Self::State>> {
            ctx.top.update_log.push("AA update".to_string());
            // Transition to B during update
            State::B
        }
    }

    pub struct B;

    impl Substate<Top> for B {
        fn update(&mut self, ctx: &mut Self::Context<'_>) -> impl Into<Next<Self::State>> {
            ctx.top.update_log.push("B update".to_string());
        }
    }
}

mod update_continuation_tests {
    use super::*;
    use cont::machine::{Builder as ContMachineBuilder, State as ContState};
    use cont::*;

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

    pub struct Top {
        pub init_count: u32,
    }

    impl TopState for Top {
        fn init(&mut self) -> impl Into<Next<Self::State>> {
            self.init_count += 1;
        }
    }

    pub struct A {
        pub init_count: u32,
        pub enter_count: u32,
    }

    impl Substate<Top> for A {
        fn enter(_ctx: &mut Self::Context<'_>) -> impl Into<Entry<Self::State, Self>> {
            Self {
                init_count: 0,
                enter_count: 1,
            }
        }

        fn init(&mut self, _ctx: &mut Self::Context<'_>) -> impl Into<Next<Self::State>> {
            self.init_count += 1;
        }
    }

    pub struct AA {
        pub enter_count: u32,
    }

    impl Substate<A> for AA {
        fn enter(_ctx: &mut Self::Context<'_>) -> impl Into<Entry<Self::State, Self>> {
            Self { enter_count: 1 }
        }
    }
}

mod exact_transitions_tests {
    use super::*;
    use exact::machine::{Builder as ExactMachineBuilder, State as ExactState};
    use exact::*;

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
    use machine::State;

    pub struct Top;
    impl TopState for Top {}

    struct Normal;
    impl Substate<Top> for Normal {}

    struct RedirectOnEnter;
    impl Substate<Top> for RedirectOnEnter {
        fn enter(_ctx: &mut Self::Context<'_>) -> impl Into<Entry<Self::State, Self>> {
            Entry::Target(State::Normal)
        }
    }

    struct ExactRedirectOnEnter;
    impl Substate<Top> for ExactRedirectOnEnter {
        fn enter(_ctx: &mut Self::Context<'_>) -> impl Into<Entry<Self::State, Self>> {
            Entry::ExactTarget(State::Top)
        }
    }

    pub struct Counter {
        pub value: u32,
    }

    impl Substate<Top> for Counter {
        fn enter(_ctx: &mut Self::Context<'_>) -> impl Into<Entry<Self::State, Self>> {
            Entry::State(Self { value: 42 })
        }
    }
}

mod state_entry_tests {
    use super::*;
    use entry::machine::{Builder as EntryMachineBuilder, State as EntryState};
    use entry::*;

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

    pub struct Top {
        pub value: u32,
    }
    impl TopState for Top {}

    struct StateA;
    impl Substate<Top> for StateA {}
}

#[state_machine(MachineB)]
mod machine_b {
    use moku::*;

    #[machine_module]
    pub mod machine {}

    pub struct Top {
        pub value: String,
    }
    impl TopState for Top {}

    struct StateB;
    impl Substate<Top> for StateB {}
}

mod multiple_machines_tests {
    use super::*;
    use machine_a::machine::{Builder as MachineAMachineBuilder, State as MachineAState};
    use machine_b::machine::{Builder as MachineBMachineBuilder, State as MachineBState};

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

    pub struct Top {
        pub event_count: u32,
    }

    // Using default () event type
    impl TopState for Top {
        fn handle_event(&mut self, _event: &Self::Event) -> impl Into<Next<Self::State>> {
            self.event_count += 1;
        }
    }

    struct A;
    impl Substate<Top> for A {}
}

mod unit_events_tests {
    use super::*;
    use unit::machine::Builder as UnitMachineBuilder;
    use unit::*;

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

    pub struct Top;
    impl TopState for Top {}

    pub struct Counter {
        pub enter_count: Rc<Cell<u32>>,
        pub exit_count: Rc<Cell<u32>>,
    }

    impl Substate<Top> for Counter {
        fn enter(_ctx: &mut Self::Context<'_>) -> impl Into<Entry<Self::State, Self>> {
            Self {
                enter_count: Rc::new(Cell::new(1)),
                exit_count: Rc::new(Cell::new(0)),
            }
        }

        fn exit(self, _ctx: &mut Self::Context<'_>) -> impl Into<Next<Self::State>> {
            self.exit_count.set(self.exit_count.get() + 1);
        }
    }
}

mod same_state_transition_tests {
    use super::*;
    use same::machine::{Builder as SameMachineBuilder, State as SameState};
    use same::*;

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
    use machine::State;

    pub struct Top {
        pub log: Vec<String>,
    }

    impl TopState for Top {
        fn top_down_update(&mut self) -> impl Into<Next<Self::State>> {
            self.log.push("Top tdu".to_string());
            // Top triggers transition to B
            State::B
        }
    }

    pub struct A;

    impl Substate<Top> for A {
        fn top_down_update(&mut self, ctx: &mut Self::Context<'_>) -> impl Into<Next<Self::State>> {
            ctx.top.log.push("A tdu".to_string());
        }
    }

    pub struct B;

    impl Substate<Top> for B {
        fn top_down_update(&mut self, ctx: &mut Self::Context<'_>) -> impl Into<Next<Self::State>> {
            ctx.top.log.push("B tdu".to_string());
        }
    }
}

mod top_down_transitions_tests {
    use super::*;
    use tdu::machine::{Builder as TduMachineBuilder, State as TduState};
    use tdu::*;

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
    use machine::State;

    #[derive(Debug)]
    pub enum Event {
        ExactToTop,
        NormalToTop,
    }

    impl StateMachineEvent for Event {}

    pub struct Top {
        pub init_count: u32,
    }

    impl TopState for Top {
        fn init(&mut self) -> impl Into<Next<Self::State>> {
            self.init_count += 1;
        }

        fn handle_event(&mut self, event: &Self::Event) -> impl Into<Next<Self::State>> {
            match event {
                Event::ExactToTop => Next::ExactTarget(State::Top),
                Event::NormalToTop => Next::Target(State::Top),
            }
        }
    }

    struct A;
    impl Substate<Top> for A {}
}

mod event_exact_tests {
    use super::*;
    use eexact::machine::{Builder as EexactMachineBuilder, State as EexactState};
    use eexact::*;

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
