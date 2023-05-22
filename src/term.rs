use crate::{BlankIdBuf, Literal, RdfDisplay, InsertIntoVocabulary, InsertedIntoVocabulary, TryExportFromVocabulary};
use iref::IriBuf;
use std::fmt;
use std::{cmp::Ordering, hash::Hash};

mod id;
mod into;
mod maybe_blank;
mod maybe_iri;

pub use id::*;
pub use into::*;
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
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
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

/// Standard gRDF term reference.
pub type TermRef<'a, L = Literal> = Term<IdRef<'a>, &'a L>;

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

	pub fn try_into_id(self) -> Result<I, L> {
		match self {
			Self::Id(id) => Ok(id),
			Self::Literal(l) => Err(l),
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

	pub fn try_into_literal(self) -> Result<L, I> {
		match self {
			Self::Literal(lit) => Ok(lit),
			Self::Id(id) => Err(id),
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

	pub fn try_into_blank(self) -> Result<I::BlankId, Self>
	where
		I: IntoBlankId,
	{
		match self {
			Self::Id(id) => id.try_into_blank().map_err(Self::Id),
			other => Err(other),
		}
	}

	pub fn into_blank(self) -> Option<I::BlankId>
	where
		I: IntoBlankId,
	{
		self.try_into_blank().ok()
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

	pub fn try_into_iri(self) -> Result<I::Iri, Self>
	where
		I: IntoIri,
	{
		match self {
			Self::Id(id) => id.try_into_iri().map_err(Self::Id),
			other => Err(other),
		}
	}

	pub fn into_iri(self) -> Option<I::Iri>
	where
		I: IntoIri,
	{
		self.try_into_iri().ok()
	}

	/// Converts from `&Term<I, L>` to `Term<&I, &L>`.
	pub fn as_ref(&self) -> Term<&I, &L> {
		match self {
			Self::Id(id) => Term::Id(id),
			Self::Literal(l) => Term::Literal(l),
		}
	}
}

impl<V, I: TryExportFromVocabulary<V>, L: TryExportFromVocabulary<V>> TryExportFromVocabulary<V> for Term<I, L> {
	type Output = Term<I::Output, L::Output>;

	type Error = Term<I::Error, L::Error>;

	fn try_export_from_vocabulary(self, vocabulary: &V) -> Result<Self::Output, Self::Error> {
		match self {
			Self::Id(i) => i
				.try_export_from_vocabulary(vocabulary)
				.map_err(Term::Id)
				.map(Term::Id),
			Self::Literal(l) => l
				.try_export_from_vocabulary(vocabulary)
				.map_err(Term::Literal)
				.map(Term::Literal),
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

impl<L> Term<Id, L> {
	#[inline(always)]
	pub fn as_term_ref(&self) -> TermRef<L> {
		match self {
			Self::Id(id) => Term::Id(id.as_id_ref()),
			Self::Literal(l) => Term::Literal(l),
		}
	}

	/// Alias for `as_term_ref`.
	#[inline(always)]
	pub fn as_object_ref(&self) -> ObjectRef<L> {
		self.as_term_ref()
	}
}

impl<V, I: InsertIntoVocabulary<V>, L: InsertIntoVocabulary<V>> InsertIntoVocabulary<V> for Term<I, L> {
	type Inserted = Term<I::Inserted, L::Inserted>;

	fn insert_into_vocabulary(self, vocabulary: &mut V) -> Self::Inserted {
		match self {
			Self::Id(id) => Term::Id(id.insert_into_vocabulary(vocabulary)),
			Self::Literal(l) => Term::Literal(l.insert_into_vocabulary(vocabulary)),
		}
	}
}

impl<V, I: InsertedIntoVocabulary<V>, L: InsertedIntoVocabulary<V>> InsertedIntoVocabulary<V> for Term<I, L> {
	type Inserted = Term<I::Inserted, L::Inserted>;

	fn inserted_into_vocabulary(&self, vocabulary: &mut V) -> Self::Inserted {
		match self {
			Self::Id(id) => Term::Id(id.inserted_into_vocabulary(vocabulary)),
			Self::Literal(l) => Term::Literal(l.inserted_into_vocabulary(vocabulary)),
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

impl<'a, L> TermRef<'a, L> {
	pub fn into_owned(self) -> Term<Id, L>
	where
		L: Clone,
	{
		match self {
			Self::Id(id) => Term::Id(id.into_owned()),
			Self::Literal(l) => Term::Literal(l.clone()),
		}
	}
}

/// RDF triple/quad subject.
pub type Subject<I = IriBuf, B = BlankIdBuf> = Id<I, B>;

/// Standard RDF subject reference.
pub type SubjectRef<'a> = IdRef<'a>;

/// RDF triple/quad object.
pub type Object<I = Id, L = Literal> = Term<I, L>;

/// Standard RDF object reference.
pub type ObjectRef<'a, L = Literal> = TermRef<'a, L>;

/// RDF quad graph Label.
pub type GraphLabel<I = IriBuf, B = BlankIdBuf> = Id<I, B>;

/// Standard RDF graph label reference.
pub type GraphLabelRef<'a> = IdRef<'a>;

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
