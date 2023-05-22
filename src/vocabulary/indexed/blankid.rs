use std::convert::TryFrom;
use std::hash::Hash;
use crate::BlankId;

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

#[cfg(feature = "contextual")]
impl<I, V: crate::BlankIdVocabulary<BlankId = BlankIdIndex<I>>> contextual::DisplayWithContext<V> for BlankIdIndex<I> {
	fn fmt_with(&self, vocabulary: &V, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		std::fmt::Display::fmt(&vocabulary.blank_id(self).unwrap(), f)
	}
}

#[cfg(feature = "contextual")]
impl<I, V: crate::BlankIdVocabulary<BlankId = BlankIdIndex<I>>> crate::RdfDisplayWithContext<V>
	for BlankIdIndex<I>
{
	fn rdf_fmt_with(&self, vocabulary: &V, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		std::fmt::Display::fmt(&vocabulary.blank_id(self).unwrap(), f)
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