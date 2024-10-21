/// A [FixedContingencyTable] is used to model scenarios where the
/// frequencies are fixed (i.e. known ahead of time), like fair dice.
/// It is mostly used for testing.
pub struct FixedContingencyTable {
    frequency: f64,
}
