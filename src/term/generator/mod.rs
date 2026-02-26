use iref::IriBuf;

use crate::Id;

mod blank;
mod uuid;

pub use blank::BlankIdGenerator;
pub use uuid::Uuid;

/// IRI generator.
pub trait IriGenerator {
	/// Generates a fresh IRI.
	fn next_iri(&mut self) -> IriBuf;
}

impl<G: IriGenerator> IriGenerator for &mut G {
	fn next_iri(&mut self) -> IriBuf {
		(*self).next_iri()
	}
}

/// Lexical identifier generator.
pub trait Generator {
	/// Generates a fresh lexical identifier.
	fn next_id(&mut self) -> Id;
}

impl<G: Generator> Generator for &mut G {
	fn next_id(&mut self) -> Id {
		(*self).next_id()
	}
}
