use crate::Id;

mod blank;
pub use blank::BlankIdGenerator;

mod uuid;
use iref::IriBuf;
pub use uuid::Uuid;

mod interpretation;
pub use interpretation::GeneratorInterpretation;

/// Subject identifier generator.
pub trait IriGenerator {
	/// Generate a fresh term.
	fn next_iri(&mut self) -> IriBuf;
}

impl<G: IriGenerator> IriGenerator for &mut G {
	fn next_iri(&mut self) -> IriBuf {
		(*self).next_iri()
	}
}

/// Subject identifier generator.
pub trait Generator {
	/// Generate a fresh term.
	fn next_id(&mut self) -> Id;
}

impl<G: Generator> Generator for &mut G {
	fn next_id(&mut self) -> Id {
		(*self).next_id()
	}
}
