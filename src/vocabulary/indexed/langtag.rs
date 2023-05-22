use std::convert::TryFrom;
use std::hash::Hash;

use langtag::{LanguageTag, AsLanguageTag};

/// IRI index.
///
/// This can be used as an IRI identifier that mixes IRIs that are statically
/// known (of type `I`) and IRIs added at run time with a dynamic index.
///
/// This type can directly be used as an IRI identifier with the
/// `IndexVocabulary` type.
#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Debug)]
pub enum LanguageTagIndex<I> {
	/// Index of the IRI in the vocabulary.
	Index(usize),

	/// Non indexed IRI.
	LanguageTag(I),
}

impl<I> From<usize> for LanguageTagIndex<I> {
	fn from(i: usize) -> Self {
		Self::Index(i)
	}
}

impl<'a, L: TryFrom<LanguageTag<'a>>> TryFrom<LanguageTag<'a>> for LanguageTagIndex<L> {
	type Error = L::Error;

	fn try_from(value: LanguageTag<'a>) -> Result<Self, Self::Error> {
		Ok(Self::LanguageTag(L::try_from(value)?))
	}
}

#[cfg(feature = "contextual")]
impl<I, V: crate::LanguageTagVocabulary<LanguageTag = LanguageTagIndex<I>>> contextual::DisplayWithContext<V> for LanguageTagIndex<I> {
	fn fmt_with(&self, vocabulary: &V, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		std::fmt::Display::fmt(&vocabulary.language_tag(self).unwrap(), f)
	}
}

#[cfg(feature = "contextual")]
impl<I, V: crate::LanguageTagVocabulary<LanguageTag = LanguageTagIndex<I>>> crate::RdfDisplayWithContext<V> for LanguageTagIndex<I> {
	fn rdf_fmt_with(&self, vocabulary: &V, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		std::fmt::Display::fmt(&vocabulary.language_tag(self).unwrap(), f)
	}
}

/// Partly indexed literal value type.
pub trait IndexedLanguageTag: From<usize> + for<'a> TryFrom<LanguageTag<'a>> {
	fn language_tag_index(&self) -> LanguageTagIndex<LanguageTag>;
}

impl<L> IndexedLanguageTag for LanguageTagIndex<L>
where
	L: AsLanguageTag + for<'a> TryFrom<LanguageTag<'a>>,
{
	fn language_tag_index(&self) -> LanguageTagIndex<LanguageTag> {
		match self {
			Self::LanguageTag(i) => LanguageTagIndex::LanguageTag(i.as_language_tag()),
			Self::Index(i) => LanguageTagIndex::Index(*i),
		}
	}
}