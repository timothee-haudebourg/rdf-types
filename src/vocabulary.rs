use crate::BlankId;
use iref::Iri;

mod index;
mod none;

pub use index::*;
pub use none::*;

/// Vocabulary.
///
/// A vocabulary is a collection that stores the lexical representation of
/// IRIs and blank node identifiers.
/// This allows the use of custom lightweight types to store, copy and compare
/// IRIs and blank IDs.
pub trait Vocabulary: IriVocabulary + BlankIdVocabulary {}

/// Mutable vocabulary.
pub trait VocabularyMut: Vocabulary + IriVocabularyMut + BlankIdVocabularyMut {}

impl<V: IriVocabulary + BlankIdVocabulary> Vocabulary for V {}

impl<V: IriVocabularyMut + BlankIdVocabularyMut> VocabularyMut for V {}

/// IRI vocabulary.
pub trait IriVocabulary {
	type Iri;

	/// Returns the IRI associated to the given IRI id.
	fn iri<'i>(&'i self, id: &'i Self::Iri) -> Option<Iri<'i>>;

	/// Returns the id of the given IRI, if any.
	fn get(&self, iri: Iri) -> Option<Self::Iri>;
}

/// Mutable IRI vocabulary.
pub trait IriVocabularyMut: IriVocabulary {
	/// Inserts an IRI to the vocabulary and returns its id.
	///
	/// If the IRI was already present in the vocabulary, no new id is created
	/// and the current one is returned.
	fn insert(&mut self, iri: Iri) -> Self::Iri;
}

/// Blank node identifier vocabulary.
pub trait BlankIdVocabulary {
	type BlankId;

	/// Returns the blank node identifier associated to the given id.
	fn blank_id<'b>(&'b self, id: &'b Self::BlankId) -> Option<&'b BlankId>;

	/// Returns the vocabulary id of the given blank node identifier, if any.
	fn get_blank_id(&self, id: &BlankId) -> Option<Self::BlankId>;
}

/// Mutable blank node identifier vocabulary.
pub trait BlankIdVocabularyMut: BlankIdVocabulary {
	/// Inserts a blank node identifier to the vocabulary and returns its id.
	///
	/// If the blank id was already present in the vocabulary, no new
	/// vocabulary id is created and the current one is returned.
	fn insert_blank_id(&mut self, id: &BlankId) -> Self::BlankId;
}
