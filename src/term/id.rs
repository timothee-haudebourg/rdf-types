use iref::IriBuf;
use std::{cmp::Ordering, fmt, hash::Hash};

#[cfg(feature = "meta")]
use locspan_derive::*;

use crate::{BlankIdBuf, RdfDisplay, Term, VocabularyMut};

/// RDF node identifier.
///
/// Either a blank node identifier or an IRI.
///
/// # `Hash` implementation
///
/// It is guaranteed that the `Hash` implementation of `Id` is
/// *transparent*, meaning that the hash of `Term::Blank(id)` the same as `id`
/// and the hash of `Subject::Iri(iri)` is the same as `iri`.
#[derive(Clone, Copy, Eq, Ord, Debug)]
#[cfg_attr(
	feature = "meta",
	derive(StrippedPartialEq, StrippedEq, StrippedPartialOrd, StrippedOrd)
)]
#[cfg_attr(feature = "meta", locspan(stripped(B, I)))]
pub enum Id<I = IriBuf, B = BlankIdBuf> {
	/// Blank node identifier.
	Blank(#[cfg_attr(feature = "meta", locspan(stripped))] B),

	/// IRI.
	Iri(#[cfg_attr(feature = "meta", locspan(stripped))] I),
}

impl<I, B> Id<I, B> {
	pub fn is_blank(&self) -> bool {
		matches!(self, Self::Blank(_))
	}

	pub fn is_iri(&self) -> bool {
		matches!(self, Self::Iri(_))
	}

	pub fn as_blank(&self) -> Option<&B> {
		match self {
			Self::Blank(id) => Some(id),
			_ => None,
		}
	}

	pub fn as_iri(&self) -> Option<&I> {
		match self {
			Self::Iri(iri) => Some(iri),
			_ => None,
		}
	}

	pub fn into_blank(self) -> Option<B> {
		match self {
			Self::Blank(id) => Some(id),
			_ => None,
		}
	}

	pub fn into_iri(self) -> Option<I> {
		match self {
			Self::Iri(iri) => Some(iri),
			_ => None,
		}
	}

	/// Converts this id reference into the term `Term::Id(&id)`.
	pub fn as_term<L>(&self) -> Term<&Self, &L> {
		Term::Id(self)
	}

	/// Converts the id into the term `Term::Id(id)`.
	pub fn into_term<L>(self) -> Term<Self, L> {
		Term::Id(self)
	}

	/// Returns a borrowed string representation of the id.
	pub fn as_str(&self) -> &str
	where
		I: AsRef<str>,
		B: AsRef<str>,
	{
		match self {
			Self::Iri(i) => i.as_ref(),
			Self::Blank(i) => i.as_ref(),
		}
	}

	/// Converts an `&Id<I, B>` into an `Id<&I, &B>`.
	pub fn as_ref(&self) -> Id<&I, &B> {
		match self {
			Self::Iri(i) => Id::Iri(i),
			Self::Blank(b) => Id::Blank(b),
		}
	}
}

impl<'a, I, B> Id<&'a I, &'a B> {
	/// Maps an `Id<&I, &B>` into an `Id<I, B>` by cloning the contents of the
	/// id.
	pub fn cloned(self) -> Id<I, B>
	where
		I: Clone,
		B: Clone,
	{
		match self {
			Self::Iri(i) => Id::Iri(i.clone()),
			Self::Blank(b) => Id::Blank(b.clone()),
		}
	}

	/// Maps an `Id<&I, &B>` into an `Id<I, B>` by copying the contents of the
	/// id.
	pub fn copied(self) -> Id<I, B>
	where
		I: Copy,
		B: Copy,
	{
		match self {
			Self::Iri(i) => Id::Iri(*i),
			Self::Blank(b) => Id::Blank(*b),
		}
	}
}

impl Id {
	pub fn inserted_into<V: VocabularyMut>(&self, vocabulary: &mut V) -> Id<V::Iri, V::BlankId> {
		match self {
			Self::Blank(b) => Id::Blank(vocabulary.insert_blank_id(b.as_blank_id_ref())),
			Self::Iri(i) => Id::Iri(vocabulary.insert(i.as_iri())),
		}
	}

	pub fn insert_into<V: VocabularyMut>(self, vocabulary: &mut V) -> Id<V::Iri, V::BlankId> {
		match self {
			Self::Blank(b) => Id::Blank(vocabulary.insert_blank_id(b.as_blank_id_ref())),
			Self::Iri(i) => Id::Iri(vocabulary.insert(i.as_iri())),
		}
	}
}

impl<I: Hash, B: Hash> Hash for Id<I, B> {
	fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
		match self {
			Self::Blank(id) => id.hash(state),
			Self::Iri(i) => i.hash(state),
		}
	}
}

#[cfg(feature = "meta")]
impl<I: Hash, B: Hash> locspan::StrippedHash for Id<I, B> {
	fn stripped_hash<H: std::hash::Hasher>(&self, state: &mut H) {
		match self {
			Self::Blank(id) => id.hash(state),
			Self::Iri(i) => i.hash(state),
		}
	}
}

impl<I1: PartialEq<I2>, B1: PartialEq<B2>, I2, B2> PartialEq<Id<I2, B2>> for Id<I1, B1> {
	fn eq(&self, other: &Id<I2, B2>) -> bool {
		match (self, other) {
			(Self::Blank(a), Id::Blank(b)) => a == b,
			(Self::Iri(a), Id::Iri(b)) => a == b,
			_ => false,
		}
	}
}

impl<I1: PartialOrd<I2>, B1: PartialOrd<B2>, I2, B2> PartialOrd<Id<I2, B2>> for Id<I1, B1> {
	fn partial_cmp(&self, other: &Id<I2, B2>) -> Option<Ordering> {
		match (self, other) {
			(Self::Blank(a), Id::Blank(b)) => a.partial_cmp(b),
			(Self::Blank(_), _) => Some(Ordering::Less),
			(Self::Iri(a), Id::Iri(b)) => a.partial_cmp(b),
			_ => Some(Ordering::Greater),
		}
	}
}

impl<I: fmt::Display, B: fmt::Display> fmt::Display for Id<I, B> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			Self::Blank(id) => id.fmt(f),
			Self::Iri(iri) => write!(f, "{iri}"),
		}
	}
}

#[cfg(feature = "contextual")]
impl<I, B, V: crate::Vocabulary<Iri = I, BlankId = B>> contextual::DisplayWithContext<V>
	for Id<I, B>
{
	fn fmt_with(&self, vocabulary: &V, f: &mut fmt::Formatter) -> fmt::Result {
		use fmt::Display;
		match self {
			Self::Blank(id) => vocabulary.blank_id(id).unwrap().fmt(f),
			Self::Iri(iri) => write!(f, "{}", vocabulary.iri(iri).unwrap()),
		}
	}
}

impl<I: fmt::Display, B: fmt::Display> RdfDisplay for Id<I, B> {
	fn rdf_fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			Self::Blank(id) => id.fmt(f),
			Self::Iri(iri) => write!(f, "<{iri}>"),
		}
	}
}

#[cfg(feature = "contextual")]
impl<I, B, V: crate::Vocabulary<Iri = I, BlankId = B>> crate::RdfDisplayWithContext<V>
	for Id<I, B>
{
	fn rdf_fmt_with(&self, vocabulary: &V, f: &mut fmt::Formatter) -> fmt::Result {
		use fmt::Display;
		match self {
			Self::Blank(id) => vocabulary.blank_id(id).unwrap().fmt(f),
			Self::Iri(iri) => write!(f, "<{}>", vocabulary.iri(iri).unwrap()),
		}
	}
}

#[cfg(feature = "contextual")]
impl<I, B, V: crate::Vocabulary<Iri = I, BlankId = B>> contextual::AsRefWithContext<str, V>
	for Id<I, B>
{
	fn as_ref_with<'a>(&'a self, vocabulary: &'a V) -> &'a str {
		match self {
			Self::Blank(b) => vocabulary.blank_id(b).unwrap().as_str(),
			Self::Iri(i) => vocabulary.iri(i).unwrap().into_str(),
		}
	}
}
