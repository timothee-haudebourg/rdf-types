use std::convert::TryFrom;
use std::hash::Hash;

use crate::Literal;

/// Literal index.
#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Debug)]
pub struct LiteralIndex(usize);

impl From<usize> for LiteralIndex {
	fn from(i: usize) -> Self {
		Self(i)
	}
}

impl From<LiteralIndex> for usize {
	fn from(value: LiteralIndex) -> Self {
		value.0
	}
}

impl<T, S> IndexedLiteral<T, S> for LiteralIndex {
	fn literal_index(&self) -> LiteralOrIndex<&'_ Literal<T, S>> {
		LiteralOrIndex::Index(self.0)
	}
}

impl<'a, T, S> TryFrom<&'a Literal<T, S>> for LiteralIndex {
	type Error = ();

	fn try_from(_value: &'a Literal<T, S>) -> Result<Self, Self::Error> {
		Err(())
	}
}

#[cfg(feature = "contextual")]
impl<V: crate::LiteralVocabulary<Literal = Self>> crate::RdfDisplayWithContext<V> for LiteralIndex
where
	V::Type: crate::RdfDisplayWithContext<V> + crate::RdfDisplayTypeSeparator,
	V::Value: crate::RdfDisplay,
{
	fn rdf_fmt_with(&self, vocabulary: &V, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		vocabulary
			.literal(self)
			.unwrap()
			.rdf_fmt_with(vocabulary, f)
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
pub enum LiteralOrIndex<I> {
	/// Index of the IRI in the vocabulary.
	Index(usize),

	/// Non indexed IRI.
	Literal(I),
}

impl<I> From<usize> for LiteralOrIndex<I> {
	fn from(i: usize) -> Self {
		Self::Index(i)
	}
}

impl<'a, T, S, L: TryFrom<&'a Literal<T, S>>> TryFrom<&'a Literal<T, S>> for LiteralOrIndex<L> {
	type Error = L::Error;

	fn try_from(value: &'a Literal<T, S>) -> Result<Self, Self::Error> {
		Ok(Self::Literal(L::try_from(value)?))
	}
}

#[cfg(feature = "contextual")]
impl<I, V: crate::LiteralVocabulary<Literal = LiteralOrIndex<I>>> contextual::DisplayWithContext<V>
	for LiteralOrIndex<I>
where
	V::Type: crate::RdfDisplayWithContext<V> + crate::RdfDisplayTypeSeparator,
	V::Value: crate::RdfDisplay,
{
	fn fmt_with(&self, vocabulary: &V, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		crate::RdfDisplayWithContext::rdf_fmt_with(vocabulary.literal(self).unwrap(), vocabulary, f)
	}
}

#[cfg(feature = "contextual")]
impl<I, V: crate::LiteralVocabulary<Literal = LiteralOrIndex<I>>> crate::RdfDisplayWithContext<V>
	for LiteralOrIndex<I>
where
	V::Type: crate::RdfDisplayWithContext<V> + crate::RdfDisplayTypeSeparator,
	V::Value: crate::RdfDisplay,
{
	fn rdf_fmt_with(&self, vocabulary: &V, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		vocabulary
			.literal(self)
			.unwrap()
			.rdf_fmt_with(vocabulary, f)
	}
}

/// Partly indexed literal value type.
pub trait IndexedLiteral<T, S>: From<usize> + for<'a> TryFrom<&'a Literal<T, S>> {
	fn literal_index(&self) -> LiteralOrIndex<&Literal<T, S>>;
}

impl<T, S, L> IndexedLiteral<T, S> for LiteralOrIndex<L>
where
	L: AsRef<Literal<T, S>> + for<'a> TryFrom<&'a Literal<T, S>>,
{
	fn literal_index(&self) -> LiteralOrIndex<&Literal<T, S>> {
		match self {
			Self::Literal(i) => LiteralOrIndex::Literal(i.as_ref()),
			Self::Index(i) => LiteralOrIndex::Index(*i),
		}
	}
}
