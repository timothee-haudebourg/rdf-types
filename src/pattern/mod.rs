use crate::Triple;

pub mod resource_or_variable;
pub use resource_or_variable::ResourceOrVar;

pub mod canonical;
pub use canonical::CanonicalTriplePattern;

pub mod map;
pub use map::TriplePatternMap;

/// Triple pattern.
pub type TriplePattern<T, X> = Triple<ResourceOrVar<T, X>>;
