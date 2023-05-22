use super::{BlankIdVocabulary, BlankIdVocabularyMut, IriVocabulary, IriVocabularyMut};
use crate::{BlankId, BlankIdBuf, Literal, Type, LiteralVocabulary, LiteralVocabularyMut, LanguageTagVocabulary, LanguageTagVocabularyMut};
use indexmap::IndexSet;
use iref::{Iri, IriBuf};
use ::langtag::{LanguageTagBuf, LanguageTag};
use std::convert::TryFrom;
use std::hash::Hash;
use std::marker::PhantomData;

mod iri;
mod blankid;
mod literal;
mod langtag;

pub use iri::*;
pub use blankid::*;
pub use literal::*;
pub use self::langtag::*;

/// Vocabulary term index.
#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Debug)]
pub struct Index(usize);

impl From<usize> for Index {
	fn from(i: usize) -> Self {
		Self(i)
	}
}

impl From<Index> for usize {
	fn from(value: Index) -> Self {
		value.0
	}
}

impl<'a> TryFrom<Iri<'a>> for Index {
	type Error = ();

	fn try_from(_value: Iri<'a>) -> Result<Self, Self::Error> {
		Err(())
	}
}

impl IndexedIri for Index {
	fn index(&self) -> IriIndex<Iri<'_>> {
		IriIndex::Index(self.0)
	}
}

impl IndexedBlankId for Index {
	fn blank_id_index(&self) -> BlankIdIndex<&'_ BlankId> {
		BlankIdIndex::Index(self.0)
	}
}

impl<'a> TryFrom<&'a BlankId> for Index {
	type Error = ();

	fn try_from(_value: &'a BlankId) -> Result<Self, Self::Error> {
		Err(())
	}
}

/// Vocabulary that stores IRIs and blank node identifiers
/// with a unique index.
pub struct IndexVocabulary<I = Index, B = Index, L = Index, T = Index, TY = Type<I, T>, TV = String> {
	iri: IndexSet<IriBuf>,
	blank_id: IndexSet<BlankIdBuf>,
	literal: IndexSet<Literal<TY, TV>>,
	language_tag: IndexSet<LanguageTagBuf>,
	types: PhantomData<(I, B, L, T)>
}

impl<I, B, L, T, TY, TV> Default for IndexVocabulary<I, B, L, T, TY, TV> {
	fn default() -> Self {
		Self {
			iri: IndexSet::new(),
			blank_id: IndexSet::new(),
			literal: IndexSet::new(),
			language_tag: IndexSet::new(),
			types: PhantomData,
		}
	}
}

impl<I, B> IndexVocabulary<I, B> {
	pub fn new() -> Self {
		Self::default()
	}
}

impl<I: IndexedIri, B, L, T, TY, TV> IriVocabulary for IndexVocabulary<I, B, L, T, TY, TV> {
	type Iri = I;

	fn iri<'i>(&'i self, id: &'i I) -> Option<Iri<'i>> {
		match id.index() {
			IriIndex::Iri(iri) => Some(iri),
			IriIndex::Index(i) => self.iri.get_index(i).map(IriBuf::as_iri),
		}
	}

	fn get(&self, iri: Iri) -> Option<I> {
		match I::try_from(iri) {
			Ok(id) => Some(id),
			Err(_) => self.iri.get_index_of(&iri.to_owned()).map(I::from),
		}
	}
}

impl<I: IndexedIri, B, L, T, TY, TV> IriVocabularyMut for IndexVocabulary<I, B, L, T, TY, TV> {
	fn insert(&mut self, iri: Iri) -> I {
		match I::try_from(iri) {
			Ok(id) => id,
			Err(_) => self.iri.insert_full(iri.to_owned()).0.into(),
		}
	}

	fn insert_owned(&mut self, iri: IriBuf) -> Self::Iri {
		if let Ok(id) = I::try_from(iri.as_iri()) {
			return id
		}

		self.iri.insert_full(iri).0.into()
	}
}

impl<I, B: IndexedBlankId, L, T, TY, TV> BlankIdVocabulary for IndexVocabulary<I, B, L, T, TY, TV> {
	type BlankId = B;

	fn blank_id<'b>(&'b self, id: &'b B) -> Option<&'b BlankId> {
		match id.blank_id_index() {
			BlankIdIndex::BlankId(id) => Some(id),
			BlankIdIndex::Index(i) => self.blank_id.get_index(i).map(BlankIdBuf::as_blank_id_ref),
		}
	}

	fn get_blank_id(&self, blank_id: &BlankId) -> Option<B> {
		match B::try_from(blank_id) {
			Ok(id) => Some(id),
			Err(_) => self.blank_id.get_index_of(&blank_id.to_owned()).map(B::from),
		}
	}
}

impl<I, B: IndexedBlankId, L, T, TY, TV> BlankIdVocabularyMut for IndexVocabulary<I, B, L, T, TY, TV> {
	fn insert_blank_id(&mut self, blank_id: &BlankId) -> Self::BlankId {
		match B::try_from(blank_id) {
			Ok(id) => id,
			Err(_) => self.blank_id.insert_full(blank_id.to_owned()).0.into(),
		}
	}

	fn insert_owned_blank_id(&mut self, id: BlankIdBuf) -> Self::BlankId {
		if let Ok(id) = B::try_from(id.as_blank_id_ref()) {
			return id
		}

		self.blank_id.insert_full(id).0.into()
	}
}

impl<I, B, L: IndexedLiteral<TY, TV>, T, TY: Clone + Eq + Hash, TV: Clone + Eq + Hash> LiteralVocabulary for IndexVocabulary<I, B, L, T, TY, TV> {
	type Literal = L;

	type Type = TY;

	type Value = TV;

	fn literal<'b>(&'b self, id: &'b L) -> Option<&'b Literal<TY, TV>> {
		match id.literal_index() {
			LiteralIndex::Literal(id) => Some(id),
			LiteralIndex::Index(i) => self.literal.get_index(i),
		}
	}

	fn get_literal(&self, literal: &Literal<TY, TV>) -> Option<L> {
		match L::try_from(literal) {
			Ok(id) => Some(id),
			Err(_) => self.literal.get_index_of(&literal.to_owned()).map(L::from),
		}
	}
}

impl<I, B, L: IndexedLiteral<TY, TV>, T, TY: Clone + Eq + Hash, TV: Clone + Eq + Hash> LiteralVocabularyMut for IndexVocabulary<I, B, L, T, TY, TV> {
	fn insert_literal(&mut self, literal: &Literal<TY, TV>) -> Self::Literal {
		match L::try_from(literal) {
			Ok(id) => id,
			Err(_) => self.literal.insert_full(literal.to_owned()).0.into(),
		}
	}

	fn insert_owned_literal(&mut self, id: Literal<TY, TV>) -> Self::Literal {
		if let Ok(id) = L::try_from(&id) {
			return id
		}

		self.literal.insert_full(id).0.into()
	}
}

impl<I, B, L, T: IndexedLanguageTag, TY, TV> LanguageTagVocabulary for IndexVocabulary<I, B, L, T, TY, TV> {
	type LanguageTag = T;

	fn language_tag<'b>(&'b self, id: &'b T) -> Option<LanguageTag<'b>> {
		match id.language_tag_index() {
			LanguageTagIndex::LanguageTag(id) => Some(id),
			LanguageTagIndex::Index(i) => self.language_tag.get_index(i).map(LanguageTagBuf::as_ref),
		}
	}

	fn get_language_tag(&self, language_tag: LanguageTag) -> Option<T> {
		match T::try_from(language_tag) {
			Ok(id) => Some(id),
			Err(_) => self.language_tag.get_index_of(&language_tag.cloned()).map(T::from),
		}
	}
}

impl<I, B, L, T: IndexedLanguageTag, TY, TV> LanguageTagVocabularyMut for IndexVocabulary<I, B, L, T, TY, TV> {
	fn insert_language_tag(&mut self, language_tag: LanguageTag) -> Self::LanguageTag {
		match T::try_from(language_tag) {
			Ok(id) => id,
			Err(_) => self.language_tag.insert_full(language_tag.cloned()).0.into(),
		}
	}

	fn insert_owned_language_tag(&mut self, id: LanguageTagBuf) -> Self::LanguageTag {
		if let Ok(id) = T::try_from(id.as_ref()) {
			return id
		}

		self.language_tag.insert_full(id).0.into()
	}
}