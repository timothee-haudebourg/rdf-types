//! This is a utility library providing common types
//! when dealing with RDF data:
//! blank node identifier, literal, subject, predicate, object,
//! graph label, gRDF term, triple and quad.
//!
//! The optional feature `meta` provides compatibility
//! with the `locspan` crate to locate every sub-component
//! of a term.
use iref::{Iri, IriBuf};

mod blankid;
mod display;
mod grdf;
mod literal;
mod quad;
mod schema;
mod term;
mod triple;

pub use blankid::*;
pub use display::*;
pub use grdf::*;
pub use literal::*;
pub use quad::*;
pub use schema::*;
pub use term::*;
pub use triple::*;

pub mod generator;
pub mod interpretation;
pub mod vocabulary;

pub use generator::Generator;
pub use interpretation::{Interpretation, InterpretationMut};
pub use vocabulary::{Vocabulary, VocabularyMut};

pub const XSD_STRING: &Iri = static_iref::iri!("http://www.w3.org/2001/XMLSchema#string");

/// IRI type that may be <http://www.w3.org/2001/XMLSchema#string>.
///
/// This is used upon formatting RDF literals to omit the type when it is not
/// required (because it is implicitly `xsd:string`).
pub trait IsXsdStringIri {
	/// Checks if this IRI is <http://www.w3.org/2001/XMLSchema#string>.
	fn is_xsd_string_iri(&self) -> bool;
}

impl IsXsdStringIri for IriBuf {
	fn is_xsd_string_iri(&self) -> bool {
		self == XSD_STRING
	}
}

impl IsXsdStringIri for Iri {
	fn is_xsd_string_iri(&self) -> bool {
		self == XSD_STRING
	}
}
