pub trait Ingress {}

pub struct MockIngress;
impl Ingress for MockIngress {}

impl From<MockIngress> for BoxIngress {
    fn from(value: MockIngress) -> Self {
        Box::new(value)
    }
}

/// Convenience alias since this type is often dynamically
/// dispatched.
pub type BoxIngress = Box<dyn Ingress>;
