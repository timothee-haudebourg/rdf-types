use crate::{BlankId, BlankIdBuf, Literal};
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
pub trait Vocabulary:
	IriVocabulary + BlankIdVocabulary + LiteralVocabulary + LanguageTagVocabulary
{
}

/// Mutable vocabulary.
pub trait VocabularyMut:
	Vocabulary
	+ IriVocabularyMut
	+ BlankIdVocabularyMut
	+ LiteralVocabularyMut
	+ LanguageTagVocabularyMut
{
}

impl<V: IriVocabulary + BlankIdVocabulary + LiteralVocabulary + LanguageTagVocabulary> Vocabulary
	for V
{
}

impl<
		V: IriVocabularyMut + BlankIdVocabularyMut + LiteralVocabularyMut + LanguageTagVocabularyMut,
	> VocabularyMut for V
{
}

/// IRI vocabulary.
pub trait IriVocabulary {
	type Iri;

	/// Returns the IRI associated to the given IRI id.
	fn iri<'i>(&'i self, id: &'i Self::Iri) -> Option<&'i Iri>;

	/// Returns a copy of the IRI associated to the given IRI id.
	fn owned_iri(&self, id: Self::Iri) -> Result<IriBuf, Self::Iri> {
		self.iri(&id).map(Iri::to_owned).ok_or(id)
	}

	/// Returns the id of the given IRI, if any.
	fn get(&self, iri: &Iri) -> Option<Self::Iri>;
}

impl<'a, V: IriVocabulary> IriVocabulary for &'a V {
	type Iri = V::Iri;

	fn iri<'i>(&'i self, id: &'i Self::Iri) -> Option<&'i Iri> {
		V::iri(*self, id)
	}

	fn owned_iri(&self, id: Self::Iri) -> Result<IriBuf, Self::Iri> {
		V::owned_iri(*self, id)
	}

	fn get(&self, iri: &Iri) -> Option<Self::Iri> {
		V::get(*self, iri)
	}
}

impl<'a, V: IriVocabulary> IriVocabulary for &'a mut V {
	type Iri = V::Iri;

	fn iri<'i>(&'i self, id: &'i Self::Iri) -> Option<&'i Iri> {
		V::iri(*self, id)
	}

	fn owned_iri(&self, id: Self::Iri) -> Result<IriBuf, Self::Iri> {
		V::owned_iri(*self, id)
	}

	fn get(&self, iri: &Iri) -> Option<Self::Iri> {
		V::get(*self, iri)
	}
}

/// Mutable IRI vocabulary.
pub trait IriVocabularyMut: IriVocabulary {
	/// Inserts an IRI to the vocabulary and returns its id.
	///
	/// If the IRI was already present in the vocabulary, no new id is created
	/// and the current one is returned.
	fn insert(&mut self, iri: &Iri) -> Self::Iri;

	fn insert_owned(&mut self, iri: IriBuf) -> Self::Iri {
		self.insert(iri.as_iri())
	}
}

impl<'a, V: IriVocabularyMut> IriVocabularyMut for &'a mut V {
	fn insert(&mut self, iri: &Iri) -> Self::Iri {
		V::insert(*self, iri)
	}

	fn insert_owned(&mut self, iri: IriBuf) -> Self::Iri {
		V::insert_owned(*self, iri)
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

impl<'a, V: BlankIdVocabulary> BlankIdVocabulary for &'a V {
	type BlankId = V::BlankId;

	fn blank_id<'b>(&'b self, id: &'b Self::BlankId) -> Option<&'b BlankId> {
		V::blank_id(*self, id)
	}

	fn owned_blank_id(&self, id: Self::BlankId) -> Result<BlankIdBuf, Self::BlankId> {
		V::owned_blank_id(*self, id)
	}

	fn get_blank_id(&self, id: &BlankId) -> Option<Self::BlankId> {
		V::get_blank_id(*self, id)
	}
}

impl<'a, V: BlankIdVocabulary> BlankIdVocabulary for &'a mut V {
	type BlankId = V::BlankId;

	fn blank_id<'b>(&'b self, id: &'b Self::BlankId) -> Option<&'b BlankId> {
		V::blank_id(*self, id)
	}

	fn owned_blank_id(&self, id: Self::BlankId) -> Result<BlankIdBuf, Self::BlankId> {
		V::owned_blank_id(*self, id)
	}

	fn get_blank_id(&self, id: &BlankId) -> Option<Self::BlankId> {
		V::get_blank_id(*self, id)
	}
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

impl<'a, V: BlankIdVocabularyMut> BlankIdVocabularyMut for &'a mut V {
	fn insert_blank_id(&mut self, id: &BlankId) -> Self::BlankId {
		V::insert_blank_id(*self, id)
	}

	fn insert_owned_blank_id(&mut self, id: BlankIdBuf) -> Self::BlankId {
		V::insert_owned_blank_id(*self, id)
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

	fn literal<'l>(&'l self, id: &'l Self::Literal)
		-> Option<&'l Literal<Self::Type, Self::Value>>;

	fn owned_literal(
		&self,
		id: Self::Literal,
	) -> Result<Literal<Self::Type, Self::Value>, Self::Literal> {
		self.literal(&id).map(Literal::clone).ok_or(id)
	}

	/// Returns the vocabulary id of the given literal identifier, if any.
	fn get_literal(&self, id: &Literal<Self::Type, Self::Value>) -> Option<Self::Literal>;
}

impl<'a, V: LiteralVocabulary> LiteralVocabulary for &'a V {
	type Literal = V::Literal;
	type Type = V::Type;
	type Value = V::Value;

	fn literal<'l>(
		&'l self,
		id: &'l Self::Literal,
	) -> Option<&'l Literal<Self::Type, Self::Value>> {
		V::literal(*self, id)
	}

	fn owned_literal(
		&self,
		id: Self::Literal,
	) -> Result<Literal<Self::Type, Self::Value>, Self::Literal> {
		V::owned_literal(*self, id)
	}

	fn get_literal(&self, id: &Literal<Self::Type, Self::Value>) -> Option<Self::Literal> {
		V::get_literal(*self, id)
	}
}

impl<'a, V: LiteralVocabulary> LiteralVocabulary for &'a mut V {
	type Literal = V::Literal;
	type Type = V::Type;
	type Value = V::Value;

	fn literal<'l>(
		&'l self,
		id: &'l Self::Literal,
	) -> Option<&'l Literal<Self::Type, Self::Value>> {
		V::literal(*self, id)
	}

	fn owned_literal(
		&self,
		id: Self::Literal,
	) -> Result<Literal<Self::Type, Self::Value>, Self::Literal> {
		V::owned_literal(*self, id)
	}

	fn get_literal(&self, id: &Literal<Self::Type, Self::Value>) -> Option<Self::Literal> {
		V::get_literal(*self, id)
	}
}

/// Mutable literal value vocabulary.
pub trait LiteralVocabularyMut: LiteralVocabulary {
	fn insert_literal(&mut self, value: &Literal<Self::Type, Self::Value>) -> Self::Literal;

	fn insert_owned_literal(&mut self, value: Literal<Self::Type, Self::Value>) -> Self::Literal {
		self.insert_literal(&value)
	}
}

impl<'a, V: LiteralVocabularyMut> LiteralVocabularyMut for &'a mut V {
	fn insert_literal(&mut self, value: &Literal<Self::Type, Self::Value>) -> Self::Literal {
		V::insert_literal(*self, value)
	}

	fn insert_owned_literal(&mut self, value: Literal<Self::Type, Self::Value>) -> Self::Literal {
		V::insert_owned_literal(*self, value)
	}
}

/// language tag vocabulary.
pub trait LanguageTagVocabulary {
	/// Language tag type.
	type LanguageTag;

	fn language_tag<'l>(&'l self, id: &'l Self::LanguageTag) -> Option<LanguageTag<'l>>;

	fn owned_language_tag(
		&self,
		id: Self::LanguageTag,
	) -> Result<LanguageTagBuf, Self::LanguageTag> {
		self.language_tag(&id).map(|t| t.cloned()).ok_or(id)
	}

	/// Returns the vocabulary id of the given language tag identifier, if any.
	fn get_language_tag(&self, id: LanguageTag) -> Option<Self::LanguageTag>;
}

impl<'a, V: LanguageTagVocabulary> LanguageTagVocabulary for &'a V {
	type LanguageTag = V::LanguageTag;

	fn language_tag<'l>(&'l self, id: &'l Self::LanguageTag) -> Option<LanguageTag<'l>> {
		V::language_tag(*self, id)
	}

	fn owned_language_tag(
		&self,
		id: Self::LanguageTag,
	) -> Result<LanguageTagBuf, Self::LanguageTag> {
		V::owned_language_tag(*self, id)
	}

	fn get_language_tag(&self, id: LanguageTag) -> Option<Self::LanguageTag> {
		V::get_language_tag(*self, id)
	}
}

impl<'a, V: LanguageTagVocabulary> LanguageTagVocabulary for &'a mut V {
	type LanguageTag = V::LanguageTag;

	fn language_tag<'l>(&'l self, id: &'l Self::LanguageTag) -> Option<LanguageTag<'l>> {
		V::language_tag(*self, id)
	}

	fn owned_language_tag(
		&self,
		id: Self::LanguageTag,
	) -> Result<LanguageTagBuf, Self::LanguageTag> {
		V::owned_language_tag(*self, id)
	}

	fn get_language_tag(&self, id: LanguageTag) -> Option<Self::LanguageTag> {
		V::get_language_tag(*self, id)
	}
}

/// Mutable literal value vocabulary.
pub trait LanguageTagVocabularyMut: LanguageTagVocabulary {
	fn insert_language_tag(&mut self, value: LanguageTag) -> Self::LanguageTag;

	fn insert_owned_language_tag(&mut self, value: LanguageTagBuf) -> Self::LanguageTag {
		self.insert_language_tag(value.as_ref())
	}
}

impl<'a, V: LanguageTagVocabularyMut> LanguageTagVocabularyMut for &'a mut V {
	fn insert_language_tag(&mut self, value: LanguageTag) -> Self::LanguageTag {
		V::insert_language_tag(*self, value)
	}

	fn insert_owned_language_tag(&mut self, value: LanguageTagBuf) -> Self::LanguageTag {
		V::insert_owned_language_tag(*self, value)
	}
}

pub trait InsertIntoVocabulary<V> {
	type Inserted;

	fn insert_into_vocabulary(self, vocabulary: &mut V) -> Self::Inserted;
}

impl<V, T: InsertIntoVocabulary<V>> InsertIntoVocabulary<V> for Vec<T> {
	type Inserted = Vec<T::Inserted>;

	fn insert_into_vocabulary(self, vocabulary: &mut V) -> Self::Inserted {
		self.into_iter()
			.map(|t| t.insert_into_vocabulary(vocabulary))
			.collect()
	}
}

impl<V, T: InsertIntoVocabulary<V>> InsertIntoVocabulary<V> for Option<T> {
	type Inserted = Option<T::Inserted>;

	fn insert_into_vocabulary(self, vocabulary: &mut V) -> Self::Inserted {
		self.map(|t| t.insert_into_vocabulary(vocabulary))
	}
}

impl<'a, V: IriVocabularyMut> InsertIntoVocabulary<V> for &'a Iri {
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

impl<V, T: InsertedIntoVocabulary<V>> InsertedIntoVocabulary<V> for Vec<T> {
	type Inserted = Vec<T::Inserted>;

	fn inserted_into_vocabulary(&self, vocabulary: &mut V) -> Self::Inserted {
		self.iter()
			.map(|t| t.inserted_into_vocabulary(vocabulary))
			.collect()
	}
}

impl<V, T: InsertedIntoVocabulary<V>> InsertedIntoVocabulary<V> for Option<T> {
	type Inserted = Option<T::Inserted>;

	fn inserted_into_vocabulary(&self, vocabulary: &mut V) -> Self::Inserted {
		self.as_ref()
			.map(|t| t.inserted_into_vocabulary(vocabulary))
	}
}

impl<'a, V: IriVocabularyMut> InsertedIntoVocabulary<V> for &'a Iri {
	type Inserted = V::Iri;

	fn inserted_into_vocabulary(&self, vocabulary: &mut V) -> Self::Inserted {
		vocabulary.insert(self)
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

/// Exports the RDF component values (IRIs, blank node identifiers, etc.)
/// embedded into the vocabulary `V`.
///
/// For `V::Iri` the output will be `IriBuf`, for `V::BlankId` it will be
/// `BlankIdBuf`, etc.
pub trait ExportFromVocabulary<V> {
	type Output;

	fn export_from_vocabulary(self, vocabulary: &V) -> Self::Output;
}

impl<T: ExportFromVocabulary<V>, V> ExportFromVocabulary<V> for Option<T> {
	type Output = Option<T::Output>;

	fn export_from_vocabulary(self, vocabulary: &V) -> Self::Output {
		self.map(|t| t.export_from_vocabulary(vocabulary))
	}
}

/// Exports the RDF component values (IRIs, blank node identifiers, etc.)
/// embedded into the vocabulary `V`.
///
/// This trait is similar to `ExportFromVocabulary` but will clone the component
/// values. For `V::Iri` the output will be `IriBuf`, for `V::BlankId` it will be
/// `BlankIdBuf`, etc.
pub trait ExportedFromVocabulary<V> {
	type Output;

	/// Exports a value embedded into the vocabulary `V`.
	///
	/// For `V::Iri` the output will be `IriBuf`, for `V::BlankId` it will be
	/// `BlankIdBuf`, etc.
	fn exported_from_vocabulary(&self, vocabulary: &V) -> Self::Output;
}

impl<T: ExportedFromVocabulary<V>, V> ExportedFromVocabulary<V> for Option<T> {
	type Output = Option<T::Output>;

	fn exported_from_vocabulary(&self, vocabulary: &V) -> Self::Output {
		self.as_ref()
			.map(|t| t.exported_from_vocabulary(vocabulary))
	}
}

/// Exports the RDF component references (IRIs, blank node identifiers, etc.)
/// embedded into the vocabulary `V`.
///
/// This trait is similar to `ExportFromVocabulary` but works on references.
/// For `&V::Iri` the output will be `IriBuf`, for `&V::BlankId` it will be
/// `BlankIdBuf`, etc.
pub trait ExportRefFromVocabulary<V> {
	type Output;

	fn export_ref_from_vocabulary(self, vocabulary: &V) -> Self::Output;
}

impl<T: ExportRefFromVocabulary<V>, V> ExportRefFromVocabulary<V> for Option<T> {
	type Output = Option<T::Output>;

	fn export_ref_from_vocabulary(self, vocabulary: &V) -> Self::Output {
		self.map(|t| t.export_ref_from_vocabulary(vocabulary))
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
		self.map(|t| t.try_export_from_vocabulary(vocabulary))
			.transpose()
	}
}
