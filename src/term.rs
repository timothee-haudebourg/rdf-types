use crate::{BlankIdBuf, Literal, RdfDisplay, VocabularyMut};
use iref::IriBuf;
use std::fmt;
use std::{cmp::Ordering, hash::Hash};

#[cfg(feature = "contextual")]
use contextual::{AsRefWithContext, DisplayWithContext};

#[cfg(feature = "meta")]
use locspan_derive::*;

/// gRDF term.
///
/// Either a node identifier or a literal value.
///
/// # `Hash` implementation
///
/// It is guaranteed that the `Hash` implementation of `Term` is *transparent*,
/// meaning that the hash of `Term::Id(id)` the same as `id` and the hash of
/// `Term::Literal(l)` is the same as `l`.
#[derive(Clone, Copy, Eq, Ord, Debug)]
#[cfg_attr(
	feature = "meta",
	derive(StrippedPartialEq, StrippedEq, StrippedPartialOrd, StrippedOrd)
)]
pub enum Term<I = Id, L = Literal> {
	/// Node identifier.
	Id(I),

	/// Literal value.
	Literal(L),
}

impl<I: Hash, L: Hash> Hash for Term<I, L> {
	fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
		match self {
			Self::Id(id) => id.hash(state),
			Self::Literal(l) => l.hash(state),
		}
	}
}

#[cfg(feature = "meta")]
impl<I: locspan::StrippedHash, L: locspan::StrippedHash> locspan::StrippedHash for Term<I, L> {
	fn stripped_hash<H: std::hash::Hasher>(&self, state: &mut H) {
		match self {
			Self::Id(id) => id.stripped_hash(state),
			Self::Literal(l) => l.stripped_hash(state),
		}
	}
}

impl<I, L> Term<I, L> {
	pub fn is_id(&self) -> bool {
		matches!(self, Self::Id(_))
	}

	pub fn is_literal(&self) -> bool {
		matches!(self, Self::Literal(_))
	}

	pub fn as_id(&self) -> Option<&I> {
		match self {
			Self::Id(id) => Some(id),
			_ => None,
		}
	}

	pub fn into_id(self) -> Option<I> {
		match self {
			Self::Id(id) => Some(id),
			_ => None,
		}
	}

	pub fn as_literal(&self) -> Option<&L> {
		match self {
			Self::Literal(lit) => Some(lit),
			_ => None,
		}
	}

	pub fn into_literal(self) -> Option<L> {
		match self {
			Self::Literal(lit) => Some(lit),
			_ => None,
		}
	}

	/// Converts from `&Term<I, L>` to `Term<&I, &L>`.
	pub fn as_ref(&self) -> Term<&I, &L> {
		match self {
			Self::Id(id) => Term::Id(id),
			Self::Literal(l) => Term::Literal(l),
		}
	}
}

impl<'a, I, L> Term<&'a I, &'a L> {
	pub fn cloned(self) -> Term<I, L>
	where
		I: Clone,
		L: Clone,
	{
		match self {
			Self::Id(id) => Term::Id(id.clone()),
			Self::Literal(l) => Term::Literal(l.clone()),
		}
	}

	pub fn copied(self) -> Term<I, L>
	where
		I: Copy,
		L: Copy,
	{
		match self {
			Self::Id(id) => Term::Id(*id),
			Self::Literal(l) => Term::Literal(*l),
		}
	}
}

impl<I, B, L> Term<Id<I, B>, L> {
	pub fn is_blank(&self) -> bool {
		matches!(self, Self::Id(Id::Blank(_)))
	}

	pub fn is_iri(&self) -> bool {
		matches!(self, Self::Id(Id::Iri(_)))
	}

	pub fn as_blank(&self) -> Option<&B> {
		match self {
			Self::Id(id) => id.as_blank(),
			_ => None,
		}
	}

	pub fn into_blank(self) -> Option<B> {
		match self {
			Self::Id(id) => id.into_blank(),
			_ => None,
		}
	}

	pub fn as_iri(&self) -> Option<&I> {
		match self {
			Self::Id(id) => id.as_iri(),
			_ => None,
		}
	}

	pub fn into_iri(self) -> Option<I> {
		match self {
			Self::Id(id) => id.into_iri(),
			_ => None,
		}
	}
}

impl<S, L> Term<Id<IriBuf, BlankIdBuf>, Literal<S, IriBuf, L>> {
	#[allow(clippy::type_complexity)]
	pub fn inserted_into<V: VocabularyMut>(
		&self,
		vocabulary: &mut V,
	) -> Term<Id<V::Iri, V::BlankId>, Literal<S, V::Iri, L>>
	where
		S: Clone,
		L: Clone,
	{
		match self {
			Self::Id(id) => Term::Id(id.inserted_into(vocabulary)),
			Self::Literal(l) => Term::Literal(l.inserted_into(vocabulary)),
		}
	}

	#[allow(clippy::type_complexity)]
	pub fn insert_into<V: VocabularyMut>(
		self,
		vocabulary: &mut V,
	) -> Term<Id<V::Iri, V::BlankId>, Literal<S, V::Iri, L>> {
		match self {
			Self::Id(id) => Term::Id(id.insert_into(vocabulary)),
			Self::Literal(l) => Term::Literal(l.insert_into(vocabulary)),
		}
	}
}

impl<I1: PartialEq<I2>, L1: PartialEq<L2>, I2, L2> PartialEq<Term<I2, L2>> for Term<I1, L1> {
	fn eq(&self, other: &Term<I2, L2>) -> bool {
		match (self, other) {
			(Self::Id(a), Term::Id(b)) => a == b,
			(Self::Literal(a), Term::Literal(b)) => a == b,
			_ => false,
		}
	}
}

impl<I1: PartialOrd<I2>, L1: PartialOrd<L2>, I2, L2> PartialOrd<Term<I2, L2>> for Term<I1, L1> {
	fn partial_cmp(&self, other: &Term<I2, L2>) -> Option<Ordering> {
		match (self, other) {
			(Self::Id(a), Term::Id(b)) => a.partial_cmp(b),
			(Self::Id(_), Term::Literal(_)) => Some(Ordering::Less),
			(Self::Literal(a), Term::Literal(b)) => a.partial_cmp(b),
			(Self::Literal(_), Term::Id(_)) => Some(Ordering::Greater),
		}
	}
}

impl<I: fmt::Display, L: fmt::Display> fmt::Display for Term<I, L> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			Self::Id(id) => id.fmt(f),
			Self::Literal(lit) => lit.fmt(f),
		}
	}
}

impl<I: RdfDisplay, L: RdfDisplay> RdfDisplay for Term<I, L> {
	fn rdf_fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			Self::Id(id) => id.rdf_fmt(f),
			Self::Literal(lit) => lit.rdf_fmt(f),
		}
	}
}

#[cfg(feature = "contextual")]
impl<I: DisplayWithContext<V>, L: DisplayWithContext<V>, V> DisplayWithContext<V> for Term<I, L> {
	fn fmt_with(&self, vocabulary: &V, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			Self::Id(id) => id.fmt_with(vocabulary, f),
			Self::Literal(lit) => lit.fmt_with(vocabulary, f),
		}
	}
}

#[cfg(feature = "contextual")]
impl<I: crate::RdfDisplayWithContext<V>, L: crate::RdfDisplayWithContext<V>, V>
	crate::RdfDisplayWithContext<V> for Term<I, L>
{
	fn rdf_fmt_with(&self, vocabulary: &V, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			Self::Id(id) => id.rdf_fmt_with(vocabulary, f),
			Self::Literal(lit) => lit.rdf_fmt_with(vocabulary, f),
		}
	}
}

#[cfg(feature = "contextual")]
impl<I: AsRefWithContext<str, V>, L: AsRef<str>, V> AsRefWithContext<str, V> for Term<I, L> {
	fn as_ref_with<'a>(&'a self, vocabulary: &'a V) -> &'a str {
		match self {
			Self::Id(id) => id.as_ref_with(vocabulary),
			Self::Literal(l) => l.as_ref(),
		}
	}
}

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
impl<I, B, V: crate::Vocabulary<Iri = I, BlankId = B>> DisplayWithContext<V> for Id<I, B> {
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
impl<I, B, V: crate::Vocabulary<Iri = I, BlankId = B>> AsRefWithContext<str, V> for Id<I, B> {
	fn as_ref_with<'a>(&'a self, vocabulary: &'a V) -> &'a str {
		match self {
			Self::Blank(b) => vocabulary.blank_id(b).unwrap().as_str(),
			Self::Iri(i) => vocabulary.iri(i).unwrap().into_str(),
		}
	}
}

/// RDF triple/quad subject.
pub type Subject<I = IriBuf, B = BlankIdBuf> = Id<I, B>;

/// RDF triple/quad object.
pub type Object<I = Id, L = Literal> = Term<I, L>;

/// RDF quad graph Label.
pub type GraphLabel<I = IriBuf, B = BlankIdBuf> = Id<I, B>;

pub trait AsRdfTerm<I, B, L> {
	fn as_rdf_term(&self) -> Term<Id<&I, &B>, &L>;
}

impl<I, B, L> AsRdfTerm<I, B, L> for Id<I, B> {
	fn as_rdf_term(&self) -> Term<Id<&I, &B>, &L> {
		Term::Id(self.as_ref())
	}
}

impl<I, B, L> AsRdfTerm<I, B, L> for Term<Id<I, B>, L> {
	fn as_rdf_term(&self) -> Term<Id<&I, &B>, &L> {
		match self {
			Self::Id(id) => Term::Id(id.as_ref()),
			Self::Literal(l) => Term::Literal(l),
		}
	}
}
