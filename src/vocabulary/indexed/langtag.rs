use std::convert::TryFrom;
use std::hash::Hash;

use langtag::{AsLanguageTag, LanguageTag};

/// Language tag index.
#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Debug)]
pub struct LanguageTagIndex(usize);

impl From<usize> for LanguageTagIndex {
	fn from(i: usize) -> Self {
		Self(i)
	}
}

impl From<LanguageTagIndex> for usize {
	fn from(value: LanguageTagIndex) -> Self {
		value.0
	}
}

impl IndexedLanguageTag for LanguageTagIndex {
	fn language_tag_index(&self) -> LanguageTagOrIndex<LanguageTag> {
		LanguageTagOrIndex::Index(self.0)
	}
}

impl<'a> TryFrom<LanguageTag<'a>> for LanguageTagIndex {
	type Error = ();

	fn try_from(_value: LanguageTag<'a>) -> Result<Self, Self::Error> {
		Err(())
	}
}

#[cfg(feature = "contextual")]
impl<V: crate::LanguageTagVocabulary<LanguageTag = Self>> crate::RdfDisplayWithContext<V>
	for LanguageTagIndex
{
	fn rdf_fmt_with(&self, vocabulary: &V, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		std::fmt::Display::fmt(&vocabulary.language_tag(self).unwrap(), f)
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
pub enum LanguageTagOrIndex<I> {
	/// Index of the IRI in the vocabulary.
	Index(usize),

	/// Non indexed IRI.
	LanguageTag(I),
}

impl<I> From<usize> for LanguageTagOrIndex<I> {
	fn from(i: usize) -> Self {
		Self::Index(i)
	}
}

impl<'a, L: TryFrom<LanguageTag<'a>>> TryFrom<LanguageTag<'a>> for LanguageTagOrIndex<L> {
	type Error = L::Error;

	fn try_from(value: LanguageTag<'a>) -> Result<Self, Self::Error> {
		Ok(Self::LanguageTag(L::try_from(value)?))
	}
}

#[cfg(feature = "contextual")]
impl<I, V: crate::LanguageTagVocabulary<LanguageTag = LanguageTagOrIndex<I>>>
	contextual::DisplayWithContext<V> for LanguageTagOrIndex<I>
{
	fn fmt_with(&self, vocabulary: &V, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		std::fmt::Display::fmt(&vocabulary.language_tag(self).unwrap(), f)
	}
}

#[cfg(feature = "contextual")]
impl<I, V: crate::LanguageTagVocabulary<LanguageTag = LanguageTagOrIndex<I>>>
	crate::RdfDisplayWithContext<V> for LanguageTagOrIndex<I>
{
	fn rdf_fmt_with(&self, vocabulary: &V, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		std::fmt::Display::fmt(&vocabulary.language_tag(self).unwrap(), f)
	}
}

/// Partly indexed literal value type.
pub trait IndexedLanguageTag: From<usize> + for<'a> TryFrom<LanguageTag<'a>> {
	fn language_tag_index(&self) -> LanguageTagOrIndex<LanguageTag>;
}

impl<L> IndexedLanguageTag for LanguageTagOrIndex<L>
where
	L: AsLanguageTag + for<'a> TryFrom<LanguageTag<'a>>,
{
	fn language_tag_index(&self) -> LanguageTagOrIndex<LanguageTag> {
		match self {
			Self::LanguageTag(i) => LanguageTagOrIndex::LanguageTag(i.as_language_tag()),
			Self::Index(i) => LanguageTagOrIndex::Index(*i),
		}
	}
}
