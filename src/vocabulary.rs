use crate::BlankId;
use iref::Iri;
use std::fmt;

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
pub trait Vocabulary<I, B>: IriVocabulary<I> + BlankIdVocabulary<B> {}

/// Mutable vocabulary.
pub trait VocabularyMut<I, B>:
	Vocabulary<I, B> + IriVocabularyMut<I> + BlankIdVocabularyMut<B>
{
}

/// IRI vocabulary.
pub trait IriVocabulary<I> {
	/// Returns the IRI associated to the given IRI id.
	fn iri<'i>(&'i self, id: &'i I) -> Option<Iri<'i>>;

	/// Returns the id of the given IRI, if any.
	fn get(&self, iri: Iri) -> Option<I>;
}

/// Mutable IRI vocabulary.
pub trait IriVocabularyMut<I>: IriVocabulary<I> {
	/// Inserts an IRI to the vocabulary and returns its id.
	///
	/// If the IRI was already present in the vocabulary, no new id is created
	/// and the current one is returned.
	fn insert(&mut self, iri: Iri) -> I;
}

/// Blank node identifier vocabulary.
pub trait BlankIdVocabulary<B> {
	/// Returns the blank node identifier associated to the given id.
	fn blank_id<'b>(&'b self, id: &'b B) -> Option<&'b BlankId>;

	/// Returns the vocabulary id of the given blank node identifier, if any.
	fn get_blank_id(&self, id: &BlankId) -> Option<B>;
}

/// Mutable blank node identifier vocabulary.
pub trait BlankIdVocabularyMut<B>: BlankIdVocabulary<B> {
	/// Inserts a blank node identifier to the vocabulary and returns its id.
	///
	/// If the blank id was already present in the vocabulary, no new
	/// vocabulary id is created and the current one is returned.
	fn insert_blank_id(&mut self, id: &BlankId) -> B;
}

/// Borrow some value with a vocabulary.
///
/// Useful to print the value with the lexical representation of IRIs and
/// blank node identifiers.
pub trait BorrowWithVocabulary {
	/// Borrows the value with the given vocabulary.
	fn with_vocabulary<'n, V>(&self, vocabulary: &'n V) -> WithVocabulary<&Self, &'n V> {
		WithVocabulary(self, vocabulary)
	}

	/// Attaches the value to the given vocabulary.
	fn into_with_vocabulary<V>(self, vocabulary: &V) -> WithVocabulary<Self, &V>
	where
		Self: Sized,
	{
		WithVocabulary(self, vocabulary)
	}
}

impl<T> BorrowWithVocabulary for T {}

/// Some value with a vocabulary.
#[derive(Clone, Copy)]
pub struct WithVocabulary<T, V>(pub T, pub V);

impl<T, V> std::ops::Deref for WithVocabulary<T, V> {
	type Target = T;

	fn deref(&self) -> &T {
		&self.0
	}
}

impl<T, V> std::ops::DerefMut for WithVocabulary<T, V> {
	fn deref_mut(&mut self) -> &mut T {
		&mut self.0
	}
}

impl<'t, 'n, T: DisplayWithVocabulary<V>, V> fmt::Display for WithVocabulary<&'t T, &'n V> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		self.0.fmt_with(self.1, f)
	}
}

/// Display function with a vocabulary.
pub trait DisplayWithVocabulary<V> {
	/// Displays the value with the given vocabulary and formatter.
	fn fmt_with(&self, vocabulary: &V, f: &mut fmt::Formatter) -> fmt::Result;
}
