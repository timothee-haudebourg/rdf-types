use crate::{BlankId, BlankIdBuf, Id, Namespace, Literal};
use iref::{Iri, IriBuf};

mod indexed;
mod none;
mod scoped;

pub use indexed::*;
use langtag::{LanguageTag, LanguageTagBuf};
pub use none::*;
pub use scoped::*;

/// Vocabulary.
///
/// A vocabulary is a collection that stores the lexical representation of
/// IRIs and blank node identifiers.
/// This allows the use of custom lightweight types to store, copy and compare
/// IRIs and blank IDs.
///
/// Any vocabulary implements the `Namespace` trait.
pub trait Vocabulary: IriVocabulary + BlankIdVocabulary {}

/// Mutable vocabulary.
pub trait VocabularyMut: Vocabulary + IriVocabularyMut + BlankIdVocabularyMut {}

impl<V: IriVocabulary + BlankIdVocabulary> Vocabulary for V {}

/// Any vocabulary is also a namespace.
impl<V: Vocabulary> Namespace for V {
	type Id = Id<V::Iri, V::BlankId>;
}

impl<V: IriVocabularyMut + BlankIdVocabularyMut> VocabularyMut for V {}

/// IRI vocabulary.
pub trait IriVocabulary {
	type Iri;

	/// Returns the IRI associated to the given IRI id.
	fn iri<'i>(&'i self, id: &'i Self::Iri) -> Option<Iri<'i>>;

	/// Returns a copy of the IRI associated to the given IRI id.
	fn owned_iri(&self, id: Self::Iri) -> Result<IriBuf, Self::Iri> {
		self.iri(&id).map(Iri::to_owned).ok_or(id)
	}

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

	fn insert_owned(&mut self, iri: IriBuf) -> Self::Iri {
		self.insert(iri.as_iri())
	}
}

/// Blank node identifier vocabulary.
pub trait BlankIdVocabulary {
	type BlankId;

	/// Returns the blank node identifier associated to the given id.
	fn blank_id<'b>(&'b self, id: &'b Self::BlankId) -> Option<&'b BlankId>;

	/// Returns a copy of the blank node identifier associated to the given id.
	fn owned_blank_id(&self, id: Self::BlankId) -> Result<BlankIdBuf, Self::BlankId> {
		self.blank_id(&id).map(BlankId::to_owned).ok_or(id)
	}

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

	fn insert_owned_blank_id(&mut self, id: BlankIdBuf) -> Self::BlankId {
		self.insert_blank_id(id.as_blank_id_ref())
	}
}

/// Literal value vocabulary.
pub trait LiteralVocabulary {
	/// Literal identifier type.
	type Literal;

	/// Literal type type.
	/// 
	/// Usually [`literal::Type<IriBuf, LanguageTagBuf>`](crate::literal::Type).
	type Type: Clone;

	/// Literal value type.
	/// 
	/// Usually [`String`].
	type Value: Clone;

	fn literal<'l>(&'l self, id: &'l Self::Literal) -> Option<&'l Literal<Self::Type, Self::Value>>;

	fn owned_literal(&self, id: Self::Literal) -> Result<Literal<Self::Type, Self::Value>, Self::Literal> {
		self.literal(&id).map(Literal::clone).ok_or(id)
	}

	/// Returns the vocabulary id of the given literal identifier, if any.
	fn get_literal(&self, id: &Literal<Self::Type, Self::Value>) -> Option<Self::Literal>;
}

/// Mutable literal value vocabulary.
pub trait LiteralVocabularyMut: LiteralVocabulary {
	fn insert_literal(&mut self, value: &Literal<Self::Type, Self::Value>) -> Self::Literal;

	fn insert_owned_literal(&mut self, value: Literal<Self::Type, Self::Value>) -> Self::Literal {
		self.insert_literal(&value)
	}
}

/// language tag vocabulary.
pub trait LanguageTagVocabulary {
	/// Language tag type.
	type LanguageTag;

	fn language_tag<'l>(&'l self, id: &'l Self::LanguageTag) -> Option<LanguageTag<'l>>;

	fn owned_language_tag(&self, id: Self::LanguageTag) -> Result<LanguageTagBuf, Self::LanguageTag> {
		self.language_tag(&id).map(|t| t.cloned()).ok_or(id)
	}

	/// Returns the vocabulary id of the given language tag identifier, if any.
	fn get_language_tag(&self, id: LanguageTag) -> Option<Self::LanguageTag>;
}

/// Mutable literal value vocabulary.
pub trait LanguageTagVocabularyMut: LanguageTagVocabulary {
	fn insert_language_tag(&mut self, value: LanguageTag) -> Self::LanguageTag;

	fn insert_owned_language_tag(&mut self, value: LanguageTagBuf) -> Self::LanguageTag {
		self.insert_language_tag(value.as_ref())
	}
}

pub trait InsertIntoVocabulary<V> {
	type Inserted;

	fn insert_into_vocabulary(self, vocabulary: &mut V) -> Self::Inserted;
}

impl<V, T: InsertIntoVocabulary<V>> InsertIntoVocabulary<V> for Option<T> {
	type Inserted = Option<T::Inserted>;

	fn insert_into_vocabulary(self, vocabulary: &mut V) -> Self::Inserted {
		self.map(|t| t.insert_into_vocabulary(vocabulary))
	}
}

impl<'a, V: IriVocabularyMut> InsertIntoVocabulary<V> for Iri<'a> {
	type Inserted = V::Iri;

	fn insert_into_vocabulary(self, vocabulary: &mut V) -> Self::Inserted {
		vocabulary.insert(self)
	}
}

impl<V: IriVocabularyMut> InsertIntoVocabulary<V> for IriBuf {
	type Inserted = V::Iri;

	fn insert_into_vocabulary(self, vocabulary: &mut V) -> Self::Inserted {
		vocabulary.insert_owned(self)
	}
}

impl<'a, V: BlankIdVocabularyMut> InsertIntoVocabulary<V> for &'a BlankId {
	type Inserted = V::BlankId;

	fn insert_into_vocabulary(self, vocabulary: &mut V) -> Self::Inserted {
		vocabulary.insert_blank_id(self)
	}
}

impl<V: BlankIdVocabularyMut> InsertIntoVocabulary<V> for BlankIdBuf {
	type Inserted = V::BlankId;

	fn insert_into_vocabulary(self, vocabulary: &mut V) -> Self::Inserted {
		vocabulary.insert_owned_blank_id(self)
	}
}

impl<'a, V: LanguageTagVocabularyMut> InsertIntoVocabulary<V> for LanguageTag<'a> {
	type Inserted = V::LanguageTag;

	fn insert_into_vocabulary(self, vocabulary: &mut V) -> Self::Inserted {
		vocabulary.insert_language_tag(self)
	}
}

impl<V: LanguageTagVocabularyMut> InsertIntoVocabulary<V> for LanguageTagBuf {
	type Inserted = V::LanguageTag;

	fn insert_into_vocabulary(self, vocabulary: &mut V) -> Self::Inserted {
		vocabulary.insert_owned_language_tag(self)
	}
}

pub trait InsertedIntoVocabulary<V> {
	type Inserted;

	fn inserted_into_vocabulary(&self, vocabulary: &mut V) -> Self::Inserted;
}

impl<V, T: InsertedIntoVocabulary<V>> InsertedIntoVocabulary<V> for Option<T> {
	type Inserted = Option<T::Inserted>;

	fn inserted_into_vocabulary(&self, vocabulary: &mut V) -> Self::Inserted {
		self.as_ref().map(|t| t.inserted_into_vocabulary(vocabulary))
	}
}

impl<'a, V: IriVocabularyMut> InsertedIntoVocabulary<V> for Iri<'a> {
	type Inserted = V::Iri;

	fn inserted_into_vocabulary(&self, vocabulary: &mut V) -> Self::Inserted {
		vocabulary.insert(*self)
	}
}

impl<V: IriVocabularyMut> InsertedIntoVocabulary<V> for IriBuf {
	type Inserted = V::Iri;

	fn inserted_into_vocabulary(&self, vocabulary: &mut V) -> Self::Inserted {
		vocabulary.insert(self.as_iri())
	}
}

impl<'a, V: BlankIdVocabularyMut> InsertedIntoVocabulary<V> for &'a BlankId {
	type Inserted = V::BlankId;

	fn inserted_into_vocabulary(&self, vocabulary: &mut V) -> Self::Inserted {
		vocabulary.insert_blank_id(self)
	}
}

impl<V: BlankIdVocabularyMut> InsertedIntoVocabulary<V> for BlankIdBuf {
	type Inserted = V::BlankId;

	fn inserted_into_vocabulary(&self, vocabulary: &mut V) -> Self::Inserted {
		vocabulary.insert_blank_id(self.as_blank_id_ref())
	}
}

impl<'a, V: LanguageTagVocabularyMut> InsertedIntoVocabulary<V> for LanguageTag<'a> {
	type Inserted = V::LanguageTag;

	fn inserted_into_vocabulary(&self, vocabulary: &mut V) -> Self::Inserted {
		vocabulary.insert_language_tag(*self)
	}
}

impl<V: LanguageTagVocabularyMut> InsertedIntoVocabulary<V> for LanguageTagBuf {
	type Inserted = V::LanguageTag;

	fn inserted_into_vocabulary(&self, vocabulary: &mut V) -> Self::Inserted {
		vocabulary.insert_language_tag(self.as_ref())
	}
}

pub trait TryExportFromVocabulary<V> {
	type Output;

	type Error;

	fn try_export_from_vocabulary(self, vocabulary: &V) -> Result<Self::Output, Self::Error>;
}

impl<V, T: TryExportFromVocabulary<V>> TryExportFromVocabulary<V> for Option<T> {
	type Output = Option<T::Output>;

	type Error = T::Error;

	fn try_export_from_vocabulary(self, vocabulary: &V) -> Result<Self::Output, Self::Error> {
		self.map(|t| t.try_export_from_vocabulary(vocabulary)).transpose()
	}
}