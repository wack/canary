use std::collections::{HashMap, HashSet};
use std::hash::Hash;

/// A ContingencyTable expresses the frequency with which a category was observed.
/// Usually, it tracks the number of observations in ecah category, but when the
/// number is already known (i.e. its fixed, like a fair dice or coin), it can
/// expose just the frequencies for each category.
pub trait ContingencyTable<Group> {
    /// return the frequency of the provided category as a number in the range [0, 1].
    /// If this is an empirical table (i.e. its values were from observations),
    /// then this is the number of times the category was observed
    /// divided by the total number of observations.
    fn group_count(&self, cat: &Group) -> usize;

    fn groups(&self) -> Box<dyn Iterator<Item = Group>>;

    /// returns the number of degrees of freedom for this table.
    /// This is typically the number of groups minus one.
    fn degrees_of_freedom(&self) -> usize {
        self.groups().count() - 1
    }

    // returns the total number of observations made. This should be the sum
    // of the group count for every group.
    fn total_count(&self) -> usize {
        self.groups()
            .fold(0, |sum, group| sum + self.group_count(&group))
    }
}

/// This helper trait identifies a category with a known set of groups.
/// For example, if modeling bools, the groups are True and False. If modeling
/// a six sided die, the groups would be 1 through 6.
///
/// You can think of an [EnumerableCategory] as a hashmap with fixed keys. When the map is
/// created, its keys must already be known and initialized with zero values.
///
/// ```rust
/// #[derive(PartialEq, Eq, Debug, Hash)]
/// enum Coin {
///   Heads,
///   Tails,
/// }
/// use std::collections::HashSet;
/// use canary::stats::EnumerableCategory;
/// impl EnumerableCategory for Coin {
///     fn groups() -> Box<dyn Iterator<Item = Self>> {
///         Box::new([Coin::Heads, Coin::Tails].into_iter())
///     }
/// }
///
/// let observed: HashSet<Coin> = Coin::groups().collect();
/// let expected = HashSet::from([Coin::Heads, Coin::Tails]);
/// assert_eq!(expected, observed);
/// ```
pub trait EnumerableCategory {
    /// This function returns an iterable over the "keys" into this
    /// category.
    fn groups() -> Box<dyn Iterator<Item = Self>>;
}

/// A [FixedContingencyTable] is used to model scenarios where the
/// frequencies are fixed (i.e. known ahead of time), like fair dice.
/// It is mostly used for testing. The category must be hashable
/// because a hashmap is used internally to store the frequencies.
/// If you'd like us to add a B-Tree based alternative, please open an issue.
pub struct FixedContingencyTable<C>
where
    C: EnumerableCategory + Hash + Eq,
{
    counts: HashMap<C, usize>,
}

impl<C> FixedContingencyTable<C>
where
    C: EnumerableCategory + Hash + Eq,
{
    /// Construct a new, empty contingency table. All frequencies are
    /// initialized to zero.
    pub fn new() -> Self {
        let mut counts = HashMap::new();
        for group in C::groups() {
            counts.entry(group).or_insert(0);
        }

        Self { counts }
    }

    /// Sets the expected count of the category to the value provided.
    pub fn set_group_count(&mut self, cat: C, count: usize) {
        self.counts.insert(cat, count);
    }

    /// Returns the frequency of the provide category.
    pub fn group_count(&self, cat: &C) -> usize {
        self.counts[cat]
    }
}

impl<C> ContingencyTable<C> for FixedContingencyTable<C>
where
    C: EnumerableCategory + Hash + Eq,
{
    /// Return the number of degrees of freedom, which is the number of
    /// groups minus 1.
    fn degrees_of_freedom(&self) -> usize {
        // The number of degrees of freedom is the number of groups minus one.
        self.counts.len() - 1
    }

    fn group_count(&self, cat: &C) -> usize {
        // delegate to the method on the base class.
        Self::group_count(self, cat)
    }

    fn groups(&self) -> Box<dyn Iterator<Item = C>> {
        // Delegate to the fixed list provided by the EnumerableCategory.
        C::groups()
    }
}

// calculate the chi square test statistic using the provided contingency tables.
fn test_statistic<Cat: EnumerableCategory + Hash + Eq>(
    control: impl ContingencyTable<Cat>,
    experimental: impl ContingencyTable<Cat>,
) -> f64 {
    // • First, get the set of groups. We can't assume that
    //   both table have the same groups, so we deduplicate them using
    //   a HashSet first.
    let groups: HashSet<_> = control.groups().chain(experimental.groups()).collect();
    // • Accumulate the total error. For each group, we calculate the error and keep
    //   a running sum.
    groups.into_iter().fold(0.0, |sum, group| {
        // Calcluate the error square: (O - E)^2
        let control_count = control.group_count(&group) as i64;
        let experimental_count = experimental.group_count(&group) as i64;
        let diff = control_count - experimental_count;
        let error = diff.pow(2) as f64;
        // Add the error to the running total.
        let incremental_error = error / (control_count as f64);
        sum + incremental_error
    })
}

#[cfg(test)]
mod tests {

    use std::collections::HashSet;

    use crate::stats::chi::FixedContingencyTable;

    use super::{test_statistic, ContingencyTable, EnumerableCategory};
    use pretty_assertions::assert_eq;
    use static_assertions::assert_obj_safe;

    // Require the contingency table is object-safe for certain commonly used categories.
    assert_obj_safe!(ContingencyTable<String>);

    impl EnumerableCategory for bool {
        fn groups() -> Box<dyn Iterator<Item = Self>> {
            Box::new([true, false].into_iter())
        }
    }

    /// This simple smoke test demonstrates that we can enumerable
    /// simple categories, like booleans.
    #[test]
    fn categories_enumerable() {
        let observed: HashSet<bool> = bool::groups().collect();
        let expected = HashSet::from([true, false]);
        assert_eq!(expected, observed);
    }

    /// This simple smoke test shows that the FixedFrequencyTable
    /// can have its frequencies set and accessed.
    #[test]
    fn enumerable_table() {
        let mut table = FixedContingencyTable::new();
        let groups = [(true, 30), (false, 70)];
        // Put the values into the table.
        for (group, freq) in groups {
            table.set_group_count(group, freq);
        }
        // Retreive the values from the table.
        for (group, freq) in groups {
            let expected = freq;
            let observed = table.group_count(&group);
            assert_eq!(expected, observed);
        }
        // Demonstrate the number of degrees of freedom matches expectations.
        assert_eq!(table.degrees_of_freedom(), 1);
    }

    /// Scenario: You flip a coin 50 times, and get 21 Heads and 29 Tails.
    /// You want to determine if the coin is fair. Output the test statistic.
    /// Let True represent Heads and False represent Tails.
    #[test]
    fn calc_test_statistic() {
        let mut control_group = FixedContingencyTable::new();
        control_group.set_group_count(true, 25);
        control_group.set_group_count(false, 25);
        let mut experimental_group = FixedContingencyTable::new();
        experimental_group.set_group_count(true, 21);
        experimental_group.set_group_count(false, 29);
        assert_eq!(control_group.degrees_of_freedom(), 1);
        assert_eq!(experimental_group.degrees_of_freedom(), 1);
        let stat = test_statistic(control_group, experimental_group);
        // Round the statistic to two decimal places.
        let observed = (stat * 100.0).round() / 100.0;
        let expected = 1.28;
        assert_eq!(observed, expected);
    }
}
