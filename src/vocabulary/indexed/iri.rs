use iref::Iri;
use std::convert::TryFrom;
use std::hash::Hash;

/// Iri index.
#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Debug)]
pub struct IriIndex(usize);

impl From<usize> for IriIndex {
	fn from(i: usize) -> Self {
		Self(i)
	}
}

impl From<IriIndex> for usize {
	fn from(value: IriIndex) -> Self {
		value.0
	}
}

impl IndexedIri for IriIndex {
	fn index(&self) -> IriOrIndex<Iri<'_>> {
		IriOrIndex::Index(self.0)
	}
}

impl<'a> TryFrom<Iri<'a>> for IriIndex {
	type Error = ();

	fn try_from(_value: Iri<'a>) -> Result<Self, Self::Error> {
		Err(())
	}
}

#[cfg(feature = "contextual")]
impl<V: crate::IriVocabulary<Iri = Self>> crate::RdfDisplayWithContext<V> for IriIndex {
	fn rdf_fmt_with(&self, vocabulary: &V, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		std::fmt::Display::fmt(&vocabulary.iri(self).unwrap(), f)
	}
}

/// Partly indexed IRI identifier type.
pub trait IndexedIri: From<usize> + for<'a> TryFrom<Iri<'a>> {
	fn index(&self) -> IriOrIndex<Iri<'_>>;
}

impl<I> IndexedIri for IriOrIndex<I>
where
	I: iref::AsIri + for<'a> TryFrom<Iri<'a>>,
{
	fn index(&self) -> IriOrIndex<Iri<'_>> {
		match self {
			Self::Iri(i) => IriOrIndex::Iri(i.as_iri()),
			Self::Index(i) => IriOrIndex::Index(*i),
		}
	}
}

/// IRI or index.
///
/// This can be used as an IRI identifier that mixes IRIs that are statically
/// known (of type `I`) and IRIs added at run time with a dynamic index.
///
/// This type can directly be used as an IRI identifier with the
/// `IndexVocabulary` type.
#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Debug)]
pub enum IriOrIndex<I> {
	/// Index of the IRI in the vocabulary.
	Index(usize),

	/// Non indexed IRI.
	Iri(I),
}

impl<I> From<usize> for IriOrIndex<I> {
	fn from(i: usize) -> Self {
		Self::Index(i)
	}
}

impl<'a, I: TryFrom<Iri<'a>>> TryFrom<Iri<'a>> for IriOrIndex<I> {
	type Error = I::Error;

	fn try_from(value: Iri<'a>) -> Result<Self, Self::Error> {
		Ok(Self::Iri(I::try_from(value)?))
	}
}

#[cfg(feature = "contextual")]
impl<I, V: crate::IriVocabulary<Iri = IriOrIndex<I>>> contextual::DisplayWithContext<V>
	for IriOrIndex<I>
{
	fn fmt_with(&self, vocabulary: &V, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		std::fmt::Display::fmt(&vocabulary.iri(self).unwrap(), f)
	}
}

#[cfg(feature = "contextual")]
impl<I, V: crate::IriVocabulary<Iri = IriOrIndex<I>>> crate::RdfDisplayWithContext<V>
	for IriOrIndex<I>
{
	fn rdf_fmt_with(&self, vocabulary: &V, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		write!(f, "<{}>", &vocabulary.iri(self).unwrap())
	}
}
