use std::convert::TryFrom;
use std::hash::Hash;

use crate::vocabulary::{ExtractFromVocabulary, ExtractedFromVocabulary, LiteralVocabulary};
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

impl<T> IndexedLiteral<T> for LiteralIndex {
	fn literal_index(&self) -> LiteralOrIndex<&'_ Literal<T>> {
		LiteralOrIndex::Index(self.0)
	}

	fn into_literal_index(self) -> LiteralOrIndex<Literal<T>> {
		LiteralOrIndex::Index(self.0)
	}
}

impl<V> ExtractFromVocabulary<V> for LiteralIndex
where
	V: LiteralVocabulary<Literal = LiteralIndex>,
{
	type Extracted = Literal;

	fn extract_from_vocabulary(self, vocabulary: &V) -> Self::Extracted {
		let literal = vocabulary.owned_literal(self).unwrap();
		let value = literal.value;
		let type_ = literal.type_.extract_from_vocabulary(vocabulary);
		Literal::new(value, type_)
	}
}

impl<V> ExtractedFromVocabulary<V> for LiteralIndex
where
	V: LiteralVocabulary<Literal = LiteralIndex>,
{
	type Extracted = Literal;

	fn exported_from_vocabulary(&self, vocabulary: &V) -> Self::Extracted {
		let literal = vocabulary.literal(self).unwrap();
		let value = literal.value.clone();
		let type_ = literal.type_.exported_from_vocabulary(vocabulary);
		Literal::new(value, type_)
	}
}

impl<'a, T> TryFrom<&'a Literal<T>> for LiteralIndex {
	type Error = ();

	fn try_from(_value: &'a Literal<T>) -> Result<Self, Self::Error> {
		Err(())
	}
}

#[cfg(feature = "contextual")]
impl<V: crate::vocabulary::LiteralVocabulary<Literal = Self>> contextual::DisplayWithContext<V>
	for LiteralIndex
where
	V::Iri: crate::RdfDisplayWithContext<V>,
{
	fn fmt_with(&self, vocabulary: &V, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		use crate::RdfDisplayWithContext;
		vocabulary
			.literal(self)
			.unwrap()
			.rdf_fmt_with(vocabulary, f)
	}
}

#[cfg(feature = "contextual")]
impl<V: crate::vocabulary::LiteralVocabulary<Literal = Self>> crate::RdfDisplayWithContext<V>
	for LiteralIndex
where
	V::Iri: crate::RdfDisplayWithContext<V>,
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

impl<'a, T, L: TryFrom<&'a Literal<T>>> TryFrom<&'a Literal<T>> for LiteralOrIndex<L> {
	type Error = L::Error;

	fn try_from(value: &'a Literal<T>) -> Result<Self, Self::Error> {
		Ok(Self::Literal(L::try_from(value)?))
	}
}

#[cfg(feature = "contextual")]
impl<I, V: crate::vocabulary::LiteralVocabulary<Literal = LiteralOrIndex<I>>>
	contextual::DisplayWithContext<V> for LiteralOrIndex<I>
where
	V::Iri: crate::RdfDisplayWithContext<V>,
{
	fn fmt_with(&self, vocabulary: &V, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		crate::RdfDisplayWithContext::rdf_fmt_with(vocabulary.literal(self).unwrap(), vocabulary, f)
	}
}

#[cfg(feature = "contextual")]
impl<I, V: crate::vocabulary::LiteralVocabulary<Literal = LiteralOrIndex<I>>>
	crate::RdfDisplayWithContext<V> for LiteralOrIndex<I>
where
	V::Iri: crate::RdfDisplayWithContext<V>,
{
	fn rdf_fmt_with(&self, vocabulary: &V, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		vocabulary
			.literal(self)
			.unwrap()
			.rdf_fmt_with(vocabulary, f)
	}
}

/// Partly indexed literal value type.
pub trait IndexedLiteral<T>: From<usize> + for<'a> TryFrom<&'a Literal<T>> {
	fn literal_index(&self) -> LiteralOrIndex<&Literal<T>>;

	fn into_literal_index(self) -> LiteralOrIndex<Literal<T>>;
}

impl<T, L> IndexedLiteral<T> for LiteralOrIndex<L>
where
	L: AsRef<Literal<T>> + Into<Literal<T>> + for<'a> TryFrom<&'a Literal<T>>,
{
	fn literal_index(&self) -> LiteralOrIndex<&Literal<T>> {
		match self {
			Self::Literal(i) => LiteralOrIndex::Literal(i.as_ref()),
			Self::Index(i) => LiteralOrIndex::Index(*i),
		}
	}

	fn into_literal_index(self) -> LiteralOrIndex<Literal<T>> {
		match self {
			Self::Literal(i) => LiteralOrIndex::Literal(i.into()),
			Self::Index(i) => LiteralOrIndex::Index(i),
		}
	}
}
