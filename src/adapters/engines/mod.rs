use crate::stats::EnumerableCategory;
use std::hash::Hash;

pub use action::Action;

/// Helper trait, since these requirements are often used by
/// our implementation of `ContingencyTables`.
trait HashableCategory: EnumerableCategory + Hash + Eq {}
impl<T: EnumerableCategory + Hash + Eq> HashableCategory for T {}

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

pub type BoxEngine<T> = Box<dyn DecisionEngine<T>>;

mod action;
mod controller;

/// The AlwaysPromote decision engine will always return the Promote
/// action when prompted. It discards all observations.
#[cfg(test)]
pub struct AlwaysPromote;

#[cfg(test)]
impl<T: HashableCategory> DecisionEngine<T> for AlwaysPromote {
    fn add_observation(&mut self, _: T) {}

    fn compute(&mut self) -> Option<Action> {
        // true to its name, it will always promote the canary.
        Some(Action::Promote)
    }
}

#[cfg(test)]
mod tests {
    use super::DecisionEngine;
    use crate::metrics::ResponseStatusCode;
    use static_assertions::assert_obj_safe;

    // We expect the DesignEngine to be boxed, and we expect
    // it to use response codes as input.
    assert_obj_safe!(DecisionEngine<ResponseStatusCode>);
}
