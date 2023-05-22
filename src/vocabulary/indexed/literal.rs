use std::convert::TryFrom;
use std::hash::Hash;

use crate::Literal;

/// IRI index.
///
/// This can be used as an IRI identifier that mixes IRIs that are statically
/// known (of type `I`) and IRIs added at run time with a dynamic index.
///
/// This type can directly be used as an IRI identifier with the
/// `IndexVocabulary` type.
#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Debug)]
pub enum LiteralIndex<I> {
	/// Index of the IRI in the vocabulary.
	Index(usize),

	/// Non indexed IRI.
	Literal(I),
}

impl<I> From<usize> for LiteralIndex<I> {
	fn from(i: usize) -> Self {
		Self::Index(i)
	}
}

impl<'a, T, S, L: TryFrom<&'a Literal<T, S>>> TryFrom<&'a Literal<T, S>> for LiteralIndex<L> {
	type Error = L::Error;

	fn try_from(value: &'a Literal<T, S>) -> Result<Self, Self::Error> {
		Ok(Self::Literal(L::try_from(value)?))
	}
}

#[cfg(feature = "contextual")]
impl<I, V: crate::LiteralVocabulary<Literal = LiteralIndex<I>>> contextual::DisplayWithContext<V> for LiteralIndex<I>
where
	V::Type: crate::RdfDisplayWithContext<V>,
	V::Value: crate::RdfDisplay
{
	fn fmt_with(&self, vocabulary: &V, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		crate::RdfDisplayWithContext::rdf_fmt_with(vocabulary.literal(self).unwrap(), vocabulary, f)
	}
}

#[cfg(feature = "contextual")]
impl<I, V: crate::LiteralVocabulary<Literal = LiteralIndex<I>>> crate::RdfDisplayWithContext<V> for LiteralIndex<I>
where
	V::Type: crate::RdfDisplayWithContext<V>,
	V::Value: crate::RdfDisplay
{
	fn rdf_fmt_with(&self, vocabulary: &V, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		vocabulary.literal(self).unwrap().rdf_fmt_with(vocabulary, f)
	}
}

/// Partly indexed literal value type.
pub trait IndexedLiteral<T, S>: From<usize> + for<'a> TryFrom<&'a Literal<T, S>> {
	fn literal_index(&self) -> LiteralIndex<&Literal<T, S>>;
}

impl<T, S, L> IndexedLiteral<T, S> for LiteralIndex<L>
where
	L: AsRef<Literal<T, S>> + for<'a> TryFrom<&'a Literal<T, S>>,
{
	fn literal_index(&self) -> LiteralIndex<&Literal<T, S>> {
		match self {
			Self::Literal(i) => LiteralIndex::Literal(i.as_ref()),
			Self::Index(i) => LiteralIndex::Index(*i),
		}
	}
}