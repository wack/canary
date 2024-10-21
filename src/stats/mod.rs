use std::collections::HashMap;

pub use chi::EnumerableCategory;

/// The alpha cutoff is the amount of confidence must have in the result
/// to feel comfortable that the result is not due to chance, but instead
/// do to the independent variable. The valu is expressed as a confidence
/// percentage: 0.05 means we are 95% confident that the observed difference
/// is not due to chance, but actually because the experimental group differs
/// from the control group.
const DEFAULT_ALPHA_CUTOFF: f64 = 0.05;

/// The [ChiSquareEngine] calculates the Chi Square test statistic
/// based on the data stored in its contingency tables.
pub struct ChiSquareEngine {
    control: ContingencyTable,
    experimental: ContingencyTable,
    total_control_count: usize,
    total_experimental_count: usize,
    alpha_cutoff: f64,
}

impl ChiSquareEngine {
    pub fn new() -> Self {
        Self {
            control: HashMap::default(),
            experimental: HashMap::default(),
            total_control_count: 0,
            total_experimental_count: 0,
            alpha_cutoff: DEFAULT_ALPHA_CUTOFF,
        }
    }

    pub fn add_observation(&mut self, obs: Observation) {
        // Fetch the count of observations for the given group.
        let entry = match obs.group {
            Group::Control => {
                self.total_control_count += 1;
                self.control.entry(obs.outcome)
            }
            Group::Experimental => {
                self.total_experimental_count += 1;
                self.experimental.entry(obs.outcome)
            }
        };
        // Increment the count.
        entry.and_modify(|count| *count += 1).or_insert(1);
    }

    /// calculate the test statistic from the contingency tables.
    pub fn calc_test_statistic(&self) -> f64 {
        let mut error = 0.0;
        let categories = [
            StatusCategory::_1XX,
            StatusCategory::_2XX,
            StatusCategory::_3XX,
            StatusCategory::_4XX,
            StatusCategory::_5XX,
        ];
        // For each category, we calculate the squared error between the
        // expected and the observed probabilies.
        for category in categories {
            let expected = self.expected_frequency(category);
            let observed = self.observed_frequency(category);
            error += (observed - expected).powi(2) / expected;
        }
        error
    }

    /// calculate the expected frequency for this category.
    fn expected_frequency(&self, category: StatusCategory) -> f64 {
        let observation_count = self.control[&category] as f64;
        let total_count = self.control[&category] as f64;
        observation_count / total_count
    }

    /// calculate the observed frequency for this category.
    fn observed_frequency(&self, category: StatusCategory) -> f64 {
        let observation_count = self.experimental[&category] as f64;
        let total_count = self.experimental[&category] as f64;
        observation_count / total_count
    }
}

/// This type maps the dependent variable to its count.
pub type ContingencyTable = HashMap<StatusCategory, usize>;

/// An [Observation] represents a measured outcome that
/// belongs to either a control group or an experimental
/// group (i.e. canary).
pub struct Observation {
    /// The experimental group or the control group.
    pub group: Group,
    /// The outcome of the observation, by status code.
    pub outcome: StatusCategory,
}

/// The [Group] indicates from whence a given observation
/// was generated: either by a control group deployment or by
/// a canary deployment.
#[derive(Hash, Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum Group {
    /// The control group is the current running deployment.
    Control,
    /// The experimental group represents the canary deployment.
    Experimental,
}

/// [StatusCategory] groups HTTP response status codes according
/// to five general categories. This type is used as the dependent
/// variable in statical observations.
#[derive(Hash, Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum StatusCategory {
    // Information responses
    _1XX,
    // Successful responses
    _2XX,
    // Redirection messages
    _3XX,
    // Client error responses
    _4XX,
    // Server error responses
    _5XX,
}

/// contains the engine to calculate the chi square test statistic.
mod chi;
/// contains implementations of contingency tables.
mod table;
