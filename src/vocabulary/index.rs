use crate::{BlankId, BlankIdBuf};
use iref::{Iri, IriBuf};
use std::collections::HashMap;
use std::convert::TryFrom;
use std::fmt;
use std::hash::Hash;

use super::{
	BlankIdVocabulary, BlankIdVocabularyMut, DisplayWithVocabulary, IriVocabulary,
	IriVocabularyMut, Vocabulary, VocabularyMut,
};

/// Vocabulary term index.
#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Debug)]
pub struct Index(usize);

impl From<usize> for Index {
	fn from(i: usize) -> Self {
		Self(i)
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

/// IRI index.
///
/// This can be used as an IRI identifier that mixes IRIs that are statically
/// known (of type `I`) and IRIs added at run time with a dynamic index.
///
/// This type can directly be used as an IRI identifier with the
/// `IndexVocabulary` type.
#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Debug)]
pub enum IriIndex<I> {
	/// Index of the IRI in the vocabulary.
	Index(usize),

	/// Non indexed IRI.
	Iri(I),
}

impl<I> From<usize> for IriIndex<I> {
	fn from(i: usize) -> Self {
		Self::Index(i)
	}
}

impl<'a, I: TryFrom<Iri<'a>>> TryFrom<Iri<'a>> for IriIndex<I> {
	type Error = I::Error;

	fn try_from(value: Iri<'a>) -> Result<Self, Self::Error> {
		Ok(Self::Iri(I::try_from(value)?))
	}
}

impl<I, V: IriVocabulary<IriIndex<I>>> DisplayWithVocabulary<V> for IriIndex<I> {
	fn fmt_with(&self, vocabulary: &V, f: &mut fmt::Formatter) -> fmt::Result {
		fmt::Display::fmt(&vocabulary.iri(self).unwrap(), f)
	}
}

/// Blank node identifier index.
///
/// This can be used as an blank id identifier that mixes blank ids that are
/// statically known (of type `B`) and blank ids added at run time with a
/// dynamic index.
///
/// This type can directly be used as an blank id identifier with the
/// `IndexVocabulary` type.
#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Debug)]
pub enum BlankIdIndex<B> {
	/// Index of the blank node identifier in the vocabulary.
	Index(usize),

	/// Non indexed blank node identifier.
	BlankId(B),
}

impl<I> From<usize> for BlankIdIndex<I> {
	fn from(i: usize) -> Self {
		Self::Index(i)
	}
}

impl<'a, I: TryFrom<&'a BlankId>> TryFrom<&'a BlankId> for BlankIdIndex<I> {
	type Error = I::Error;

	fn try_from(value: &'a BlankId) -> Result<Self, Self::Error> {
		Ok(Self::BlankId(I::try_from(value)?))
	}
}

impl<I, V: BlankIdVocabulary<BlankIdIndex<I>>> DisplayWithVocabulary<V> for BlankIdIndex<I> {
	fn fmt_with(&self, vocabulary: &V, f: &mut fmt::Formatter) -> fmt::Result {
		fmt::Display::fmt(&vocabulary.blank_id(self).unwrap(), f)
	}
}

/// Partly indexed IRI identifier type.
pub trait IndexedIri: From<usize> + for<'a> TryFrom<Iri<'a>> {
	fn index(&self) -> IriIndex<Iri<'_>>;
}

impl<I> IndexedIri for IriIndex<I>
where
	I: iref::AsIri + for<'a> TryFrom<Iri<'a>>,
{
	fn index(&self) -> IriIndex<Iri<'_>> {
		match self {
			Self::Iri(i) => IriIndex::Iri(i.as_iri()),
			Self::Index(i) => IriIndex::Index(*i),
		}
	}
}

/// Partly indexed blank node identifier type.
pub trait IndexedBlankId: From<usize> + for<'a> TryFrom<&'a BlankId> {
	fn blank_id_index(&self) -> BlankIdIndex<&'_ BlankId>;
}

impl<B> IndexedBlankId for BlankIdIndex<B>
where
	B: AsRef<BlankId> + for<'a> TryFrom<&'a BlankId>,
{
	fn blank_id_index(&self) -> BlankIdIndex<&'_ BlankId> {
		match self {
			Self::BlankId(i) => BlankIdIndex::BlankId(i.as_ref()),
			Self::Index(i) => BlankIdIndex::Index(*i),
		}
	}
}

/// Vocabulary that stores IRIs and blank node identifiers
/// with a unique index.
#[derive(Default)]
pub struct IndexVocabulary {
	allocated: Vec<IriBuf>,
	map: HashMap<IriBuf, usize>,
	blank_allocated: Vec<BlankIdBuf>,
	blank_map: HashMap<BlankIdBuf, usize>,
}

impl IndexVocabulary {
	pub fn new() -> Self {
		Self::default()
	}
}

impl<I: IndexedIri> IriVocabulary<I> for IndexVocabulary {
	fn iri<'i>(&'i self, id: &'i I) -> Option<Iri<'i>> {
		match id.index() {
			IriIndex::Iri(iri) => Some(iri),
			IriIndex::Index(i) => self.allocated.get(i).map(IriBuf::as_iri),
		}
	}

	fn get(&self, iri: Iri) -> Option<I> {
		match I::try_from(iri) {
			Ok(id) => Some(id),
			Err(_) => self.map.get(&iri.to_owned()).cloned().map(I::from),
		}
	}
}

impl<I: IndexedIri> IriVocabularyMut<I> for IndexVocabulary {
	fn insert(&mut self, iri: Iri) -> I {
		match I::try_from(iri) {
			Ok(id) => id,
			Err(_) => I::from(*self.map.entry(iri.to_owned()).or_insert_with_key(|key| {
				let index = self.allocated.len();
				self.allocated.push(key.clone());
				index
			})),
		}
	}
}

impl<B: IndexedBlankId> BlankIdVocabulary<B> for IndexVocabulary {
	fn blank_id<'b>(&'b self, id: &'b B) -> Option<&'b BlankId> {
		match id.blank_id_index() {
			BlankIdIndex::BlankId(id) => Some(id),
			BlankIdIndex::Index(i) => self.blank_allocated.get(i).map(BlankIdBuf::as_blank_id_ref),
		}
	}

	fn get_blank_id(&self, blank_id: &BlankId) -> Option<B> {
		match B::try_from(blank_id) {
			Ok(id) => Some(id),
			Err(_) => self.blank_map.get(blank_id).cloned().map(B::from),
		}
	}
}

impl<B: IndexedBlankId> BlankIdVocabularyMut<B> for IndexVocabulary {
	fn insert_blank_id(&mut self, blank_id: &BlankId) -> B {
		match B::try_from(blank_id) {
			Ok(id) => id,
			Err(_) => B::from(
				*self
					.blank_map
					.entry(blank_id.to_owned())
					.or_insert_with_key(|key| {
						let index = self.blank_allocated.len();
						self.blank_allocated.push(key.clone());
						index
					}),
			),
		}
	}
}

impl<I: IndexedIri, B: IndexedBlankId> Vocabulary<I, B> for IndexVocabulary {}

impl<I: IndexedIri, B: IndexedBlankId> VocabularyMut<I, B> for IndexVocabulary {}
