use iref::Iri;
use std::convert::TryFrom;
use std::hash::Hash;

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

#[cfg(feature = "contextual")]
impl<I, V: crate::IriVocabulary<Iri = IriIndex<I>>> contextual::DisplayWithContext<V> for IriIndex<I> {
	fn fmt_with(&self, vocabulary: &V, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		std::fmt::Display::fmt(&vocabulary.iri(self).unwrap(), f)
	}
}

#[cfg(feature = "contextual")]
impl<I, V: crate::IriVocabulary<Iri = IriIndex<I>>> crate::RdfDisplayWithContext<V> for IriIndex<I> {
	fn rdf_fmt_with(&self, vocabulary: &V, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		write!(f, "<{}>", &vocabulary.iri(self).unwrap())
	}
}