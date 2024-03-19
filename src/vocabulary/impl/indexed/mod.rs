use std::hash::Hash;
use std::marker::PhantomData;

use crate::vocabulary::{
	BlankIdVocabulary, BlankIdVocabularyMut, IriVocabulary, IriVocabularyMut, LiteralVocabulary,
	LiteralVocabularyMut,
};
use crate::{BlankId, BlankIdBuf, Literal, LiteralRef};
use indexmap::IndexSet;
use iref::{Iri, IriBuf};

mod blankid;
mod iri;
mod literal;

pub use blankid::*;
pub use iri::*;
pub use literal::*;

/// Vocabulary that stores IRIs and blank node identifiers
/// with a unique index.
pub struct IndexVocabulary<I = IriIndex, B = BlankIdIndex, L = LiteralIndex> {
	iri: IndexSet<IriBuf>,
	blank_id: IndexSet<BlankIdBuf>,
	literal: IndexSet<Literal<I>>,
	bl: PhantomData<(B, L)>,
}

impl<I, B, L> Default for IndexVocabulary<I, B, L> {
	fn default() -> Self {
		Self {
			iri: IndexSet::new(),
			blank_id: IndexSet::new(),
			literal: IndexSet::new(),
			bl: PhantomData,
		}
	}
}

impl<I, B> IndexVocabulary<I, B> {
	pub fn new() -> Self {
		Self::default()
	}
}

impl<I: IndexedIri, B, L> IriVocabulary for IndexVocabulary<I, B, L> {
	type Iri = I;

	fn iri<'i>(&'i self, id: &'i I) -> Option<&'i Iri> {
		match id.index() {
			IriOrIndex::Iri(iri) => Some(iri),
			IriOrIndex::Index(i) => self.iri.get_index(i).map(IriBuf::as_iri),
		}
	}

	fn get(&self, iri: &Iri) -> Option<I> {
		match I::try_from(iri) {
			Ok(id) => Some(id),
			Err(_) => self.iri.get_index_of(&iri.to_owned()).map(I::from),
		}
	}
}

impl<I: IndexedIri, B, L> IriVocabularyMut for IndexVocabulary<I, B, L> {
	fn insert(&mut self, iri: &Iri) -> I {
		match I::try_from(iri) {
			Ok(id) => id,
			Err(_) => self.iri.insert_full(iri.to_owned()).0.into(),
		}
	}

	fn insert_owned(&mut self, iri: IriBuf) -> Self::Iri {
		if let Ok(id) = I::try_from(iri.as_iri()) {
			return id;
		}

		self.iri.insert_full(iri).0.into()
	}
}

impl<I, B: IndexedBlankId, L> BlankIdVocabulary for IndexVocabulary<I, B, L> {
	type BlankId = B;

	fn blank_id<'b>(&'b self, id: &'b B) -> Option<&'b BlankId> {
		match id.blank_id_index() {
			BlankIdOrIndex::BlankId(id) => Some(id),
			BlankIdOrIndex::Index(i) => self.blank_id.get_index(i).map(BlankIdBuf::as_blank_id_ref),
		}
	}

	fn get_blank_id(&self, blank_id: &BlankId) -> Option<B> {
		match B::try_from(blank_id) {
			Ok(id) => Some(id),
			Err(_) => self
				.blank_id
				.get_index_of(&blank_id.to_owned())
				.map(B::from),
		}
	}
}

impl<I, B: IndexedBlankId, L> BlankIdVocabularyMut for IndexVocabulary<I, B, L> {
	fn insert_blank_id(&mut self, blank_id: &BlankId) -> Self::BlankId {
		match B::try_from(blank_id) {
			Ok(id) => id,
			Err(_) => self.blank_id.insert_full(blank_id.to_owned()).0.into(),
		}
	}

	fn insert_owned_blank_id(&mut self, id: BlankIdBuf) -> Self::BlankId {
		if let Ok(id) = B::try_from(id.as_blank_id_ref()) {
			return id;
		}

		self.blank_id.insert_full(id).0.into()
	}
}

impl<I: Clone + IndexedIri + Eq + Hash, B, L: IndexedLiteral<I>> LiteralVocabulary
	for IndexVocabulary<I, B, L>
{
	type Literal = L;

	fn literal<'b>(&'b self, id: &'b L) -> Option<LiteralRef<'b, I>> {
		match id.literal_index() {
			LiteralOrIndex::Literal(id) => Some(id.as_ref()),
			LiteralOrIndex::Index(i) => self.literal.get_index(i).map(Literal::as_ref),
		}
	}

	fn owned_literal(&self, id: Self::Literal) -> Result<Literal<Self::Iri>, Self::Literal> {
		match id.into_literal_index() {
			LiteralOrIndex::Literal(id) => Ok(id),
			LiteralOrIndex::Index(i) => match self.literal.get_index(i).cloned() {
				Some(t) => Ok(t),
				None => Err(i.into()),
			},
		}
	}

	fn get_literal(&self, literal: LiteralRef<Self::Iri>) -> Option<L> {
		match L::try_from(literal) {
			Ok(id) => Some(id),
			Err(_) => self
				.literal
				.get_index_of(&literal.into_owned())
				.map(L::from),
		}
	}
}

impl<I: IndexedIri + Clone + Eq + Hash, B, L: IndexedLiteral<I>> LiteralVocabularyMut
	for IndexVocabulary<I, B, L>
{
	fn insert_literal(&mut self, literal: LiteralRef<Self::Iri>) -> Self::Literal {
		match L::try_from(literal) {
			Ok(id) => id,
			Err(_) => self.literal.insert_full(literal.into_owned()).0.into(),
		}
	}

	fn insert_owned_literal(&mut self, literal: Literal<I>) -> Self::Literal {
		match L::try_from(literal) {
			Ok(id) => id,
			Err(literal) => self.literal.insert_full(literal).0.into(),
		}
	}
}
