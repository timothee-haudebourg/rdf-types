use crate::{Quad, Triple};

pub mod resource_or_variable;
pub use resource_or_variable::ResourceOrVar;

pub mod quad;
pub use quad::CanonicalQuadPattern;

pub mod triple;
pub use triple::{CanonicalTriplePattern, TriplePatternMap};

/// Triple pattern.
pub type TriplePattern<T, X> = Triple<ResourceOrVar<T, X>>;

/// Triple pattern.
pub type QuadPattern<T, X> = Quad<ResourceOrVar<T, X>>;
