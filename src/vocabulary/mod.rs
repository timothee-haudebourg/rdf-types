//! Lexical domain abstractions.
//!
//! Having to store, clone and compare blank ids, IRIs and literals is expensive
//! and may become a burden to a RDF-intensive application. This modules defines
//! the [`Vocabulary`] trait (and similar traits) to abstract away the lexical
//! representation of resources.
//!
//! Using vocabularies, an IRI can be represented as a simple integer, or enum
//! type, drastically reducing the cost of storage and comparison.
mod blank_id;
mod iri;
mod literal;

pub use blank_id::*;
pub use iri::*;
pub use literal::*;

mod r#impl;
pub use r#impl::*;

/// Vocabulary.
///
/// A vocabulary is a collection that stores the lexical representation of
/// IRIs and blank node identifiers.
/// This allows the use of custom lightweight types to store, copy and compare
/// IRIs and blank IDs.
///
/// Any vocabulary implements the `Namespace` trait.
pub trait Vocabulary: IriVocabulary + BlankIdVocabulary + LiteralVocabulary {}

/// Mutable vocabulary.
pub trait VocabularyMut:
	Vocabulary + IriVocabularyMut + BlankIdVocabularyMut + LiteralVocabularyMut
{
}

impl<V: IriVocabulary + BlankIdVocabulary + LiteralVocabulary> Vocabulary for V {}

impl<V: IriVocabularyMut + BlankIdVocabularyMut + LiteralVocabularyMut> VocabularyMut for V {}

/// Value that can be embedded into the given vocabulary by consuming it.
pub trait EmbedIntoVocabulary<V> {
	/// Type of the value once embedded into the vocabulary.
	type Embedded;

	fn embed_into_vocabulary(self, vocabulary: &mut V) -> Self::Embedded;
}

impl<V, T: EmbedIntoVocabulary<V>> EmbedIntoVocabulary<V> for Vec<T> {
	type Embedded = Vec<T::Embedded>;

	fn embed_into_vocabulary(self, vocabulary: &mut V) -> Self::Embedded {
		self.into_iter()
			.map(|t| t.embed_into_vocabulary(vocabulary))
			.collect()
	}
}

impl<V, T: EmbedIntoVocabulary<V>> EmbedIntoVocabulary<V> for Option<T> {
	type Embedded = Option<T::Embedded>;

	fn embed_into_vocabulary(self, vocabulary: &mut V) -> Self::Embedded {
		self.map(|t| t.embed_into_vocabulary(vocabulary))
	}
}

/// Value that can be embedded into the given vocabulary without consuming it.
pub trait EmbeddedIntoVocabulary<V> {
	type Embedded;

	fn embedded_into_vocabulary(&self, vocabulary: &mut V) -> Self::Embedded;
}

impl<V, T: EmbeddedIntoVocabulary<V>> EmbeddedIntoVocabulary<V> for Vec<T> {
	type Embedded = Vec<T::Embedded>;

	fn embedded_into_vocabulary(&self, vocabulary: &mut V) -> Self::Embedded {
		self.iter()
			.map(|t| t.embedded_into_vocabulary(vocabulary))
			.collect()
	}
}

impl<V, T: EmbeddedIntoVocabulary<V>> EmbeddedIntoVocabulary<V> for Option<T> {
	type Embedded = Option<T::Embedded>;

	fn embedded_into_vocabulary(&self, vocabulary: &mut V) -> Self::Embedded {
		self.as_ref()
			.map(|t| t.embedded_into_vocabulary(vocabulary))
	}
}

/// Extract the RDF component values (IRIs, blank node identifiers, etc.)
/// embedded into the vocabulary `V`.
///
/// For `V::Iri` the output will be `IriBuf`, for `V::BlankId` it will be
/// `BlankIdBuf`, etc.
pub trait ExtractFromVocabulary<V> {
	type Extracted;

	fn extract_from_vocabulary(self, vocabulary: &V) -> Self::Extracted;
}

impl<T: ExtractFromVocabulary<V>, V> ExtractFromVocabulary<V> for Option<T> {
	type Extracted = Option<T::Extracted>;

	fn extract_from_vocabulary(self, vocabulary: &V) -> Self::Extracted {
		self.map(|t| t.extract_from_vocabulary(vocabulary))
	}
}

/// Exports the RDF component values (IRIs, blank node identifiers, etc.)
/// embedded into the vocabulary `V`.
///
/// This trait is similar to `ExportFromVocabulary` but will clone the component
/// values. For `V::Iri` the output will be `IriBuf`, for `V::BlankId` it will be
/// `BlankIdBuf`, etc.
pub trait ExtractedFromVocabulary<V> {
	type Extracted;

	/// Exports a value embedded into the vocabulary `V`.
	///
	/// For `V::Iri` the output will be `IriBuf`, for `V::BlankId` it will be
	/// `BlankIdBuf`, etc.
	fn exported_from_vocabulary(&self, vocabulary: &V) -> Self::Extracted;
}

impl<T: ExtractedFromVocabulary<V>, V> ExtractedFromVocabulary<V> for Option<T> {
	type Extracted = Option<T::Extracted>;

	fn exported_from_vocabulary(&self, vocabulary: &V) -> Self::Extracted {
		self.as_ref()
			.map(|t| t.exported_from_vocabulary(vocabulary))
	}
}

/// Try to extract the RDF component values (IRIs, blank node identifiers, etc.)
/// embedded into the vocabulary `V`. This is the fallible version of
/// [`ExtractFromVocabulary`].
///
/// For `V::Iri` the output will be `IriBuf`, for `V::BlankId` it will be
/// `BlankIdBuf`, etc.
pub trait TryExtractFromVocabulary<V> {
	type Extracted;

	type Error;

	fn try_extract_from_vocabulary(self, vocabulary: &V) -> Result<Self::Extracted, Self::Error>;
}

impl<V, T: TryExtractFromVocabulary<V>> TryExtractFromVocabulary<V> for Option<T> {
	type Extracted = Option<T::Extracted>;

	type Error = T::Error;

	fn try_extract_from_vocabulary(self, vocabulary: &V) -> Result<Self::Extracted, Self::Error> {
		self.map(|t| t.try_extract_from_vocabulary(vocabulary))
			.transpose()
	}
}
