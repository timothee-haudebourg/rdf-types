use crate::{BlankIdBuf, Literal, RdfDisplay, VocabularyMut};
use iref::IriBuf;
use std::fmt;
use std::{cmp::Ordering, hash::Hash};

mod id;
mod maybe_blank;
mod maybe_iri;

pub use id::*;
pub use maybe_blank::*;
pub use maybe_iri::*;

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
	pub fn blank(id: I::BlankId) -> Self
	where
		I: FromBlankId,
	{
		Self::Id(I::from_blank(id))
	}

	pub fn iri(iri: I::Iri) -> Self
	where
		I: FromIri,
	{
		Self::Id(I::from_iri(iri))
	}

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

	pub fn is_blank(&self) -> bool
	where
		I: AsBlankId,
	{
		match self {
			Self::Id(id) => id.is_blank(),
			Self::Literal(_) => false,
		}
	}

	pub fn is_iri(&self) -> bool
	where
		I: AsIri,
	{
		match self {
			Self::Id(id) => id.is_iri(),
			Self::Literal(_) => false,
		}
	}

	pub fn as_blank(&self) -> Option<&I::BlankId>
	where
		I: AsBlankId,
	{
		match self {
			Self::Id(id) => id.as_blank(),
			_ => None,
		}
	}

	pub fn into_blank(self) -> Option<I::BlankId>
	where
		I: IntoBlankId,
	{
		match self {
			Self::Id(id) => id.into_blank(),
			_ => None,
		}
	}

	pub fn as_iri(&self) -> Option<&I::Iri>
	where
		I: AsIri,
	{
		match self {
			Self::Id(id) => id.as_iri(),
			_ => None,
		}
	}

	pub fn into_iri(self) -> Option<I::Iri>
	where
		I: IntoIri,
	{
		match self {
			Self::Id(id) => id.into_iri(),
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
