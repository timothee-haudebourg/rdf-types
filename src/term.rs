use crate::{BlankId, BlankIdBuf, Literal, StringLiteral};
use iref::{Iri, IriBuf};
use std::cmp::Ordering;
use std::fmt;

#[cfg(feature = "contextual")]
use contextual::{AsRefWithContext, DisplayWithContext};

#[cfg(feature = "meta")]
use locspan_derive::*;

/// gRDF term.
///
/// Either a blank node identifier, IRI or literal value.
#[derive(Clone, Copy, Eq, Hash, Ord, Debug)]
#[cfg_attr(
	feature = "meta",
	derive(
		StrippedPartialEq,
		StrippedEq,
		StrippedPartialOrd,
		StrippedOrd,
		StrippedHash
	)
)]
#[cfg_attr(feature = "meta", stripped(B, I))]
pub enum Term<I = IriBuf, B = BlankIdBuf, L = Literal<StringLiteral, I>> {
	/// Blank node identifier.
	Blank(#[cfg_attr(feature = "meta", stripped)] B),

	/// IRI.
	Iri(#[cfg_attr(feature = "meta", stripped)] I),

	/// Literal value.
	Literal(L),
}

impl<I, B, L> Term<I, B, L> {
	pub fn is_blank(&self) -> bool {
		matches!(self, Self::Blank(_))
	}

	pub fn is_iri(&self) -> bool {
		matches!(self, Self::Iri(_))
	}

	pub fn is_literal(&self) -> bool {
		matches!(self, Self::Literal(_))
	}

	pub fn as_blank(&self) -> Option<&B> {
		match self {
			Self::Blank(id) => Some(id),
			_ => None,
		}
	}

	pub fn into_blank(self) -> Option<B> {
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

	pub fn into_iri(self) -> Option<I> {
		match self {
			Self::Iri(iri) => Some(iri),
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
			Self::Blank(id) => TermRef::Blank(id),
			Self::Iri(iri) => TermRef::Iri(iri.as_iri()),
			Self::Literal(lit) => TermRef::Literal(lit),
		}
	}

	pub fn as_object_ref(&self) -> TermRef {
		self.as_term_ref()
	}
}

impl<I1: PartialEq<I2>, B1: PartialEq<B2>, L1: PartialEq<L2>, I2, B2, L2>
	PartialEq<Term<I2, B2, L2>> for Term<I1, B1, L1>
{
	fn eq(&self, other: &Term<I2, B2, L2>) -> bool {
		match (self, other) {
			(Self::Blank(a), Term::Blank(b)) => a == b,
			(Self::Iri(a), Term::Iri(b)) => a == b,
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
			(Self::Blank(a), Term::Blank(b)) => a.partial_cmp(b),
			(Self::Blank(_), _) => Some(Ordering::Less),
			(Self::Iri(a), Term::Iri(b)) => a.partial_cmp(b),
			(Self::Iri(_), Term::Blank(_)) => Some(Ordering::Greater),
			(Self::Iri(_), _) => Some(Ordering::Less),
			(Self::Literal(a), Term::Literal(b)) => a.partial_cmp(b),
			_ => Some(Ordering::Greater),
		}
	}
}

impl<I: fmt::Display, B: fmt::Display, L: fmt::Display> fmt::Display for Term<I, B, L> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			Self::Blank(id) => id.fmt(f),
			Self::Iri(iri) => write!(f, "<{}>", iri),
			Self::Literal(lit) => lit.fmt(f),
		}
	}
}

#[cfg(feature = "contextual")]
impl<I, B, L: DisplayWithContext<V>, V: crate::Vocabulary<Iri = I, BlankId = B>>
	DisplayWithContext<V> for Term<I, B, L>
{
	fn fmt_with(&self, vocabulary: &V, f: &mut fmt::Formatter) -> fmt::Result {
		use fmt::Display;
		match self {
			Self::Blank(id) => vocabulary.blank_id(id).unwrap().fmt(f),
			Self::Iri(iri) => write!(f, "<{}>", vocabulary.iri(iri).unwrap()),
			Self::Literal(lit) => lit.fmt_with(vocabulary, f),
		}
	}
}

#[cfg(feature = "contextual")]
impl<I, B, L: AsRef<str>, V: crate::Vocabulary<Iri = I, BlankId = B>> AsRefWithContext<str, V>
	for Term<I, B, L>
{
	fn as_ref_with<'a>(&'a self, vocabulary: &'a V) -> &'a str {
		match self {
			Self::Blank(b) => vocabulary.blank_id(b).unwrap().as_str(),
			Self::Iri(i) => vocabulary.iri(i).unwrap().into_str(),
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
			Self::Iri(iri) => Term::Iri(iri),
			Self::Blank(id) => Term::Blank(id),
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
			Self::Iri(iri) => Term::Iri(iri),
			Self::Blank(id) => Term::Blank(id),
			Self::Literal(lit) => Term::Literal(lit),
		}
	}
}

/// gRDF term reference.
pub type TermRef<'a> = Term<Iri<'a>, &'a BlankId, &'a Literal>;

impl<'a> From<&'a Term> for TermRef<'a> {
	fn from(t: &'a Term) -> Self {
		t.as_term_ref()
	}
}

/// RDF Subject.
///
/// Either a blank node identifier or an IRI.
#[derive(Clone, Copy, Eq, Hash, Ord, Debug)]
#[cfg_attr(
	feature = "meta",
	derive(
		StrippedPartialEq,
		StrippedEq,
		StrippedPartialOrd,
		StrippedOrd,
		StrippedHash
	)
)]
#[cfg_attr(feature = "meta", stripped(B, I))]
pub enum Subject<I = IriBuf, B = BlankIdBuf> {
	/// Blank node identifier.
	Blank(#[cfg_attr(feature = "meta", stripped)] B),

	/// IRI.
	Iri(#[cfg_attr(feature = "meta", stripped)] I),
}

impl<I, B> Subject<I, B> {
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

	pub fn into_term<L>(self) -> Term<I, B, L> {
		match self {
			Self::Blank(id) => Term::Blank(id),
			Self::Iri(iri) => Term::Iri(iri),
		}
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

impl Subject {
	pub fn as_subject_ref(&self) -> SubjectRef {
		match self {
			Self::Blank(id) => SubjectRef::Blank(id),
			Self::Iri(iri) => SubjectRef::Iri(iri.as_iri()),
		}
	}

	pub fn as_graph_label_ref(&self) -> GraphLabelRef {
		self.as_subject_ref()
	}

	pub fn as_term_ref(&self) -> TermRef {
		match self {
			Self::Blank(id) => TermRef::Blank(id),
			Self::Iri(iri) => TermRef::Iri(iri.as_iri()),
		}
	}
}

impl<I1: PartialEq<I2>, B1: PartialEq<B2>, I2, B2> PartialEq<Subject<I2, B2>> for Subject<I1, B1> {
	fn eq(&self, other: &Subject<I2, B2>) -> bool {
		match (self, other) {
			(Self::Blank(a), Subject::Blank(b)) => a == b,
			(Self::Iri(a), Subject::Iri(b)) => a == b,
			_ => false,
		}
	}
}

impl<I1: PartialOrd<I2>, B1: PartialOrd<B2>, I2, B2> PartialOrd<Subject<I2, B2>>
	for Subject<I1, B1>
{
	fn partial_cmp(&self, other: &Subject<I2, B2>) -> Option<Ordering> {
		match (self, other) {
			(Self::Blank(a), Subject::Blank(b)) => a.partial_cmp(b),
			(Self::Blank(_), _) => Some(Ordering::Less),
			(Self::Iri(a), Subject::Iri(b)) => a.partial_cmp(b),
			_ => Some(Ordering::Greater),
		}
	}
}

impl<I: fmt::Display, B: fmt::Display> fmt::Display for Subject<I, B> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			Self::Blank(id) => id.fmt(f),
			Self::Iri(iri) => write!(f, "<{}>", iri),
		}
	}
}

#[cfg(feature = "contextual")]
impl<I, B, V: crate::Vocabulary<Iri = I, BlankId = B>> DisplayWithContext<V> for Subject<I, B> {
	fn fmt_with(&self, vocabulary: &V, f: &mut fmt::Formatter) -> fmt::Result {
		use fmt::Display;
		match self {
			Self::Blank(id) => vocabulary.blank_id(id).unwrap().fmt(f),
			Self::Iri(iri) => write!(f, "<{}>", vocabulary.iri(iri).unwrap()),
		}
	}
}

#[cfg(feature = "contextual")]
impl<I, B, V: crate::Vocabulary<Iri = I, BlankId = B>> AsRefWithContext<str, V> for Subject<I, B> {
	fn as_ref_with<'a>(&'a self, vocabulary: &'a V) -> &'a str {
		match self {
			Self::Blank(b) => vocabulary.blank_id(b).unwrap().as_str(),
			Self::Iri(i) => vocabulary.iri(i).unwrap().into_str(),
		}
	}
}

pub type SubjectRef<'a> = Subject<Iri<'a>, &'a BlankId>;

impl<'a> From<&'a Subject> for SubjectRef<'a> {
	fn from(t: &'a Subject) -> Self {
		t.as_subject_ref()
	}
}

impl<I, B> AsTerm for Subject<I, B> {
	type Iri = I;
	type BlankId = B;
	type Literal = std::convert::Infallible;

	fn as_term(&self) -> Term<&I, &B, &Self::Literal> {
		match self {
			Self::Iri(iri) => Term::Iri(iri),
			Self::Blank(id) => Term::Blank(id),
		}
	}
}

impl<I, B> IntoTerm for Subject<I, B> {
	type Iri = I;
	type BlankId = B;
	type Literal = std::convert::Infallible;

	fn into_term(self) -> Term<I, B, Self::Literal> {
		match self {
			Self::Iri(iri) => Term::Iri(iri),
			Self::Blank(id) => Term::Blank(id),
		}
	}
}

/// RDF Object.
pub type Object<I = IriBuf, B = BlankIdBuf, L = Literal> = Term<I, B, L>;

/// RDF Object reference.
pub type ObjectRef<'a> = TermRef<'a>;

/// RDF Graph Label.
pub type GraphLabel<I = IriBuf, B = BlankIdBuf> = Subject<I, B>;

/// RDF Graph Label reference.
pub type GraphLabelRef<'a> = SubjectRef<'a>;

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
