use crate::{LocalTerm, Term};

mod blank;
pub use blank::Blank;

mod uuid;
pub use uuid::Uuid;

mod interpretation;
pub use interpretation::LocalGeneratorInterpretation;

/// Subject identifier generator.
pub trait Generator {
	/// Generate a fresh term.
	fn next_term(&mut self) -> Term;
}

impl<'a, G: Generator> Generator for &'a mut G {
	fn next_term(&mut self) -> Term {
		(*self).next_term()
	}
}

pub trait LocalGenerator {
	fn next_local_term(&mut self) -> LocalTerm;
}

impl<'a, G: LocalGenerator> LocalGenerator for &'a mut G {
	fn next_local_term(&mut self) -> LocalTerm {
		(*self).next_local_term()
	}
}
