use crate::{BlankId, BlankIdBuf, Literal, RdfDisplay, VocabularyMut};
use iref::{Iri, IriBuf};
use std::fmt;
use std::{cmp::Ordering, hash::Hash};

#[cfg(feature = "contextual")]
use contextual::{AsRefWithContext, DisplayWithContext};

#[cfg(feature = "meta")]
use locspan_derive::*;

/// gRDF term.
///
/// Either a blank node identifier, IRI or literal value.
///
/// # `Hash` implementation
///
/// It is guaranteed that the `Hash` implementation of `Term` is *transparent*,
/// meaning that the hash of `Term::Blank(id)` the same as `id`, the hash of
/// `Term::Iri(iri)` is the same as `iri` and the hash of `Term::Literal(l)` is
/// the same as `l`.
#[derive(Clone, Copy, Eq, Ord, Debug)]
#[cfg_attr(
	feature = "meta",
	derive(StrippedPartialEq, StrippedEq, StrippedPartialOrd, StrippedOrd)
)]
#[cfg_attr(feature = "meta", locspan(stripped(B, I)))]
pub enum Term<I = IriBuf, B = BlankIdBuf, L = Literal<String, I>> {
	/// Node identifier.
	Id(Id<I, B>),

	/// Literal value.
	Literal(L),
}

impl<I: Hash, B: Hash, L: Hash> Hash for Term<I, B, L> {
	fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
		match self {
			Self::Id(id) => id.hash(state),
			Self::Literal(l) => l.hash(state),
		}
	}
}

#[cfg(feature = "meta")]
impl<I: Hash, B: Hash, L: locspan::StrippedHash> locspan::StrippedHash for Term<I, B, L> {
	fn stripped_hash<H: std::hash::Hasher>(&self, state: &mut H) {
		match self {
			Self::Id(id) => id.hash(state),
			Self::Literal(l) => l.stripped_hash(state),
		}
	}
}

impl<I, B, L> Term<I, B, L> {
	pub fn is_id(&self) -> bool {
		matches!(self, Self::Id(_))
	}

	pub fn is_blank(&self) -> bool {
		matches!(self, Self::Id(Id::Blank(_)))
	}

	pub fn is_iri(&self) -> bool {
		matches!(self, Self::Id(Id::Iri(_)))
	}

	pub fn is_literal(&self) -> bool {
		matches!(self, Self::Literal(_))
	}

	pub fn as_id(&self) -> Option<&Id<I, B>> {
		match self {
			Self::Id(id) => Some(id),
			_ => None,
		}
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

	pub fn into_id(self) -> Option<Id<I, B>> {
		match self {
			Self::Id(id) => Some(id),
			_ => None,
		}
	}

	pub fn into_iri(self) -> Option<I> {
		match self {
			Self::Id(id) => id.into_iri(),
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
}

impl Term {
	pub fn as_term_ref(&self) -> TermRef {
		match self {
			Self::Id(id) => TermRef::Id(id.as_subject_ref()),
			Self::Literal(lit) => TermRef::Literal(lit),
		}
	}

	pub fn as_object_ref(&self) -> TermRef {
		self.as_term_ref()
	}
}

impl<S, L> Term<IriBuf, BlankIdBuf, Literal<S, IriBuf, L>> {
	#[allow(clippy::type_complexity)]
	pub fn inserted_into<V: VocabularyMut>(
		&self,
		vocabulary: &mut V,
	) -> Term<V::Iri, V::BlankId, Literal<S, V::Iri, L>>
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
	) -> Term<V::Iri, V::BlankId, Literal<S, V::Iri, L>> {
		match self {
			Self::Id(id) => Term::Id(id.insert_into(vocabulary)),
			Self::Literal(l) => Term::Literal(l.insert_into(vocabulary)),
		}
	}
}

impl<I1: PartialEq<I2>, B1: PartialEq<B2>, L1: PartialEq<L2>, I2, B2, L2>
	PartialEq<Term<I2, B2, L2>> for Term<I1, B1, L1>
{
	fn eq(&self, other: &Term<I2, B2, L2>) -> bool {
		match (self, other) {
			(Self::Id(a), Term::Id(b)) => a == b,
			(Self::Literal(a), Term::Literal(b)) => a == b,
			_ => false,
		}
	}
}

impl<I1: PartialOrd<I2>, B1: PartialOrd<B2>, L1: PartialOrd<L2>, I2, B2, L2>
	PartialOrd<Term<I2, B2, L2>> for Term<I1, B1, L1>
{
	fn partial_cmp(&self, other: &Term<I2, B2, L2>) -> Option<Ordering> {
		match (self, other) {
			(Self::Id(a), Term::Id(b)) => a.partial_cmp(b),
			(Self::Id(_), Term::Literal(_)) => Some(Ordering::Less),
			(Self::Literal(a), Term::Literal(b)) => a.partial_cmp(b),
			(Self::Literal(_), Term::Id(_)) => Some(Ordering::Greater),
		}
	}
}

impl<I: fmt::Display, B: fmt::Display, L: fmt::Display> fmt::Display for Term<I, B, L> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			Self::Id(id) => id.fmt(f),
			Self::Literal(lit) => lit.fmt(f),
		}
	}
}

impl<I: fmt::Display, B: fmt::Display, L: RdfDisplay> RdfDisplay for Term<I, B, L> {
	fn rdf_fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			Self::Id(id) => id.rdf_fmt(f),
			Self::Literal(lit) => lit.rdf_fmt(f),
		}
	}
}

#[cfg(feature = "contextual")]
impl<I, B, L: DisplayWithContext<V>, V: crate::Vocabulary<Iri = I, BlankId = B>>
	DisplayWithContext<V> for Term<I, B, L>
{
	fn fmt_with(&self, vocabulary: &V, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			Self::Id(id) => id.fmt_with(vocabulary, f),
			Self::Literal(lit) => lit.fmt_with(vocabulary, f),
		}
	}
}

#[cfg(feature = "contextual")]
impl<I, B, L: crate::RdfDisplayWithContext<V>, V: crate::Vocabulary<Iri = I, BlankId = B>>
	crate::RdfDisplayWithContext<V> for Term<I, B, L>
{
	fn rdf_fmt_with(&self, vocabulary: &V, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			Self::Id(id) => id.rdf_fmt_with(vocabulary, f),
			Self::Literal(lit) => lit.rdf_fmt_with(vocabulary, f),
		}
	}
}

#[cfg(feature = "contextual")]
impl<I, B, L: AsRef<str>, V: crate::Vocabulary<Iri = I, BlankId = B>> AsRefWithContext<str, V>
	for Term<I, B, L>
{
	fn as_ref_with<'a>(&'a self, vocabulary: &'a V) -> &'a str {
		match self {
			Self::Id(id) => id.as_ref_with(vocabulary),
			Self::Literal(l) => l.as_ref(),
		}
	}
}

impl<I, B, L> AsTerm for Term<I, B, L> {
	type Iri = I;
	type BlankId = B;
	type Literal = L;

	fn as_term(&self) -> Term<&I, &B, &L> {
		match self {
			Self::Id(Id::Iri(iri)) => Term::Id(Id::Iri(iri)),
			Self::Id(Id::Blank(id)) => Term::Id(Id::Blank(id)),
			Self::Literal(lit) => Term::Literal(lit),
		}
	}
}

impl<I, B, L> IntoTerm for Term<I, B, L> {
	type Iri = I;
	type BlankId = B;
	type Literal = L;

	fn into_term(self) -> Term<I, B, L> {
		match self {
			Self::Id(Id::Iri(iri)) => Term::Id(Id::Iri(iri)),
			Self::Id(Id::Blank(id)) => Term::Id(Id::Blank(id)),
			Self::Literal(lit) => Term::Literal(lit),
		}
	}
}

/// gRDF term reference.
pub type TermRef<'a> = Term<Iri<'a>, &'a BlankId, &'a Literal>;

impl<'a> TermRef<'a> {
	pub fn into_owned(self) -> Term {
		match self {
			Self::Id(id) => Term::Id(id.into_owned()),
			Self::Literal(l) => Term::Literal(l.clone()),
		}
	}
}

impl<'a> From<&'a Term> for TermRef<'a> {
	fn from(t: &'a Term) -> Self {
		t.as_term_ref()
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

	pub fn into_term<L>(self) -> Term<I, B, L> {
		Term::Id(self)
	}

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
}

impl Id {
	pub fn as_id_ref(&self) -> IdRef {
		match self {
			Self::Blank(id) => IdRef::Blank(id),
			Self::Iri(iri) => IdRef::Iri(iri.as_iri()),
		}
	}

	/// Alias for [`as_id_ref`](Self::as_id_ref).
	pub fn as_subject_ref(&self) -> IdRef {
		self.as_id_ref()
	}

	/// Alias for [`as_id_ref`](Self::as_id_ref).
	pub fn as_graph_label_ref(&self) -> GraphLabelRef {
		self.as_id_ref()
	}

	pub fn as_term_ref(&self) -> TermRef {
		TermRef::Id(self.as_id_ref())
	}

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

pub type IdRef<'a> = Id<Iri<'a>, &'a BlankId>;

impl<'a> IdRef<'a> {
	pub fn into_owned(self) -> Id {
		match self {
			Self::Iri(iri) => Id::Iri(iri.to_owned()),
			Self::Blank(b) => Id::Blank(b.to_owned()),
		}
	}
}

impl<'a> From<&'a Id> for IdRef<'a> {
	fn from(t: &'a Id) -> Self {
		t.as_subject_ref()
	}
}

impl<I, B> AsTerm for Id<I, B> {
	type Iri = I;
	type BlankId = B;
	type Literal = std::convert::Infallible;

	fn as_term(&self) -> Term<&I, &B, &Self::Literal> {
		match self {
			Self::Iri(iri) => Term::Id(Id::Iri(iri)),
			Self::Blank(id) => Term::Id(Id::Blank(id)),
		}
	}
}

impl<I, B> IntoTerm for Id<I, B> {
	type Iri = I;
	type BlankId = B;
	type Literal = std::convert::Infallible;

	fn into_term(self) -> Term<I, B, Self::Literal> {
		match self {
			Self::Iri(iri) => Term::Id(Id::Iri(iri)),
			Self::Blank(id) => Term::Id(Id::Blank(id)),
		}
	}
}

/// RDF triple/quad subject.
pub type Subject<I = IriBuf, B = BlankIdBuf> = Id<I, B>;

/// RDF triple/quad subject reference.
pub type SubjectRef<'a> = IdRef<'a>;

/// RDF triple/quad object.
pub type Object<I = IriBuf, B = BlankIdBuf, L = Literal> = Term<I, B, L>;

/// RDF triple/quad object reference.
pub type ObjectRef<'a> = TermRef<'a>;

/// RDF quad graph Label.
pub type GraphLabel<I = IriBuf, B = BlankIdBuf> = Id<I, B>;

/// RDF quad graph label reference.
pub type GraphLabelRef<'a> = IdRef<'a>;

pub trait AsTerm {
	type Iri;
	type BlankId;
	type Literal;

	fn as_term(&self) -> Term<&Self::Iri, &Self::BlankId, &Self::Literal>;
}

pub trait IntoTerm {
	type Iri;
	type BlankId;
	type Literal;

	fn into_term(self) -> Term<Self::Iri, Self::BlankId, Self::Literal>;
}
