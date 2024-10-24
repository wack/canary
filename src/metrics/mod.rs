use crate::stats::EnumerableCategory;

/// [ResponseStatusCode] groups HTTP response status codes according
/// to five general categories. This type is used as the dependent
/// variable in statical observations.
#[derive(Hash, Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum ResponseStatusCode {
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

impl EnumerableCategory for ResponseStatusCode {
    fn groups() -> Box<dyn Iterator<Item = Self>> {
        Box::new([Self::_1XX, Self::_2XX, Self::_3XX, Self::_4XX, Self::_5XX].into_iter())
    }
}
