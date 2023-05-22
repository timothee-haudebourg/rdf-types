//! This is a utility library providing common types
//! when dealing with RDF data:
//! blank node identifier, literal, subject, predicate, object,
//! graph label, gRDF term, triple and quad.
//!
//! The optional feature `meta` provides compatibility
//! with the `locspan` crate to locate every sub-component
//! of a term.
mod blankid;
mod display;
pub mod generator;
mod literal;
mod quad;
mod term;
mod triple;
pub mod vocabulary;

#[cfg(feature = "meta")]
pub mod meta;

pub use blankid::*;
pub use display::*;
pub use generator::Generator;
pub use literal::*;
pub use quad::*;
pub use term::*;
pub use triple::*;
pub use vocabulary::{
	BlankIdVocabulary, BlankIdVocabularyMut, IndexVocabulary, IriVocabulary, IriVocabularyMut,
	NoVocabulary, Vocabulary, VocabularyMut,
	LiteralVocabulary, LiteralVocabularyMut,
	LanguageTagVocabulary, LanguageTagVocabularyMut,
	InsertIntoVocabulary, InsertedIntoVocabulary,
	TryExportFromVocabulary
};

#[cfg(feature = "meta")]
pub use generator::MetaGenerator;

/// Node identifier namespace.
pub trait Namespace {
	type Id;
}
