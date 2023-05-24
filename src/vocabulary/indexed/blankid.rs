use crate::BlankId;
use std::convert::TryFrom;
use std::hash::Hash;

/// Blank id index.
#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Debug)]
pub struct BlankIdIndex(usize);

impl From<usize> for BlankIdIndex {
	fn from(i: usize) -> Self {
		Self(i)
	}
}

impl From<BlankIdIndex> for usize {
	fn from(value: BlankIdIndex) -> Self {
		value.0
	}
}

impl IndexedBlankId for BlankIdIndex {
	fn blank_id_index(&self) -> BlankIdOrIndex<&'_ BlankId> {
		BlankIdOrIndex::Index(self.0)
	}
}

impl<'a> TryFrom<&'a BlankId> for BlankIdIndex {
	type Error = ();

	fn try_from(_value: &'a BlankId) -> Result<Self, Self::Error> {
		Err(())
	}
}

#[cfg(feature = "contextual")]
impl<V: crate::BlankIdVocabulary<BlankId = Self>> crate::RdfDisplayWithContext<V> for BlankIdIndex {
	fn rdf_fmt_with(&self, vocabulary: &V, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		std::fmt::Display::fmt(&vocabulary.blank_id(self).unwrap(), f)
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
pub enum BlankIdOrIndex<B> {
	/// Index of the blank node identifier in the vocabulary.
	Index(usize),

	/// Non indexed blank node identifier.
	BlankId(B),
}

impl<I> From<usize> for BlankIdOrIndex<I> {
	fn from(i: usize) -> Self {
		Self::Index(i)
	}
}

impl<'a, I: TryFrom<&'a BlankId>> TryFrom<&'a BlankId> for BlankIdOrIndex<I> {
	type Error = I::Error;

	fn try_from(value: &'a BlankId) -> Result<Self, Self::Error> {
		Ok(Self::BlankId(I::try_from(value)?))
	}
}

#[cfg(feature = "contextual")]
impl<I, V: crate::BlankIdVocabulary<BlankId = BlankIdOrIndex<I>>> contextual::DisplayWithContext<V>
	for BlankIdOrIndex<I>
{
	fn fmt_with(&self, vocabulary: &V, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		std::fmt::Display::fmt(&vocabulary.blank_id(self).unwrap(), f)
	}
}

#[cfg(feature = "contextual")]
impl<I, V: crate::BlankIdVocabulary<BlankId = BlankIdOrIndex<I>>> crate::RdfDisplayWithContext<V>
	for BlankIdOrIndex<I>
{
	fn rdf_fmt_with(&self, vocabulary: &V, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		std::fmt::Display::fmt(&vocabulary.blank_id(self).unwrap(), f)
	}
}

/// Partly indexed blank node identifier type.
pub trait IndexedBlankId: From<usize> + for<'a> TryFrom<&'a BlankId> {
	fn blank_id_index(&self) -> BlankIdOrIndex<&'_ BlankId>;
}

impl<B> IndexedBlankId for BlankIdOrIndex<B>
where
	B: AsRef<BlankId> + for<'a> TryFrom<&'a BlankId>,
{
	fn blank_id_index(&self) -> BlankIdOrIndex<&'_ BlankId> {
		match self {
			Self::BlankId(i) => BlankIdOrIndex::BlankId(i.as_ref()),
			Self::Index(i) => BlankIdOrIndex::Index(*i),
		}
	}
}
