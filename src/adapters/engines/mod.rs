use crate::stats::EnumerableCategory;
use std::hash::Hash;

pub use action::Action;

/// Helper trait, since these requirements are often used by
/// our implementation of `ContingencyTables`.
trait HashableCategory: EnumerableCategory + Hash + Eq {}
impl<T: EnumerableCategory + Hash + Eq> HashableCategory for T {}

// TODO: We probably want to define an "EngineController" type that wraps the execution
//       of the engine with extra configurables, like "how often do we run the engine"
//       and "how many samples do we need before we run the engine again?"
//       The EngineController can also handle the async threading and stream interactions.

/// The decision engine receives observations from the monitor
/// and determines whether the canary should be promoted, yanked,
/// or scaled up or down.
pub trait DecisionEngine<T: HashableCategory> {
    /// [add_observation] provides a new observation that the engine
    /// should take under advisement before making a decision.
    fn add_observation(&mut self, observation: T);

    /// [compute] will ask the engine to run over all known observations.
    /// The engine isn't required to output an [Action]. It might determine
    /// there isn't enough data to make an affirmative decision.
    fn compute(&mut self) -> Option<Action>;
}

pub struct MockEngine;
impl<T: HashableCategory> DecisionEngine<T> for MockEngine {
    fn add_observation(&mut self, _: T) {
        todo!();
    }

    fn compute(&mut self) -> Option<Action> {
        todo!()
    }
}

// TODO: maybe this should be a nutype.
pub type BoxEngine<T: HashableCategory> = Box<dyn DecisionEngine<T>>;

mod action;
