use iref::Iri;
use std::convert::TryFrom;
use std::hash::Hash;

use crate::XSD_STRING;

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
	fn index(&self) -> IriOrIndex<&Iri> {
		IriOrIndex::Index(self.0)
	}
}

impl<'a> TryFrom<&'a Iri> for IriIndex {
	type Error = ();

	fn try_from(_value: &'a Iri) -> Result<Self, Self::Error> {
		Err(())
	}
}

impl<V: crate::IriVocabulary<Iri = Self>> crate::literal::RdfTypeIriWithContext<V> for IriIndex {
	fn is_xsd_string_with(&self, vocabulary: &V) -> bool {
		vocabulary.iri(self).unwrap() == XSD_STRING
	}
}

#[cfg(feature = "contextual")]
impl<V: crate::IriVocabulary<Iri = Self>> contextual::DisplayWithContext<V> for IriIndex {
	fn fmt_with(&self, vocabulary: &V, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		std::fmt::Display::fmt(&vocabulary.iri(self).unwrap(), f)
	}
}

#[cfg(feature = "contextual")]
impl<V: crate::IriVocabulary<Iri = Self>> crate::RdfDisplayWithContext<V> for IriIndex {
	fn rdf_fmt_with(&self, vocabulary: &V, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		std::fmt::Display::fmt(&vocabulary.iri(self).unwrap(), f)
	}
}

/// Partly indexed IRI identifier type.
pub trait IndexedIri: From<usize> + for<'a> TryFrom<&'a Iri> {
	fn index(&self) -> IriOrIndex<&Iri>;
}

impl<I> IndexedIri for IriOrIndex<I>
where
	I: AsRef<Iri> + for<'a> TryFrom<&'a Iri>,
{
	fn index(&self) -> IriOrIndex<&Iri> {
		match self {
			Self::Iri(i) => IriOrIndex::Iri(i.as_ref()),
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

impl<'a, I: TryFrom<&'a Iri>> TryFrom<&'a Iri> for IriOrIndex<I> {
	type Error = I::Error;

	fn try_from(value: &'a Iri) -> Result<Self, Self::Error> {
		Ok(Self::Iri(I::try_from(value)?))
	}
}

impl<I, V: crate::IriVocabulary<Iri = Self>> crate::literal::RdfTypeIriWithContext<V>
	for IriOrIndex<I>
{
	fn is_xsd_string_with(&self, vocabulary: &V) -> bool {
		vocabulary.iri(self).unwrap() == XSD_STRING
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
