/// The decision engine receives observations from the monitor
/// and determines whether the canary should be promoted, yanked,
/// or scaled up or down.
pub trait DecisionEngine {}
pub struct MockEngine;
impl DecisionEngine for MockEngine {}

impl From<MockEngine> for BoxEngine {
    fn from(value: MockEngine) -> Self {
        Box::new(value)
    }
}

pub type BoxEngine = Box<dyn DecisionEngine>;
