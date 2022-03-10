use crate::{BlankId, BlankIdBuf, Literal};
use iref::{Iri, IriBuf};
use std::fmt;

/// gRDF term.
///
/// Either a blank node identifier, IRI or literal value.
#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Debug)]
pub enum Term<I = IriBuf, B = BlankIdBuf, L = Literal> {
	/// Blank node identifier.
	Blank(B),

	/// IRI.
	Iri(I),

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

impl<I: fmt::Display, B: fmt::Display, L: fmt::Display> fmt::Display for Term<I, B, L> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			Self::Blank(id) => id.fmt(f),
			Self::Iri(iri) => write!(f, "<{}>", iri),
			Self::Literal(lit) => lit.fmt(f),
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
#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Debug)]
pub enum Subject<I = IriBuf, B = BlankIdBuf> {
	/// Blank node identifier.
	Blank(B),

	/// IRI.
	Iri(I),
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

	pub fn into_term(self) -> Term<I, B> {
		match self {
			Self::Blank(id) => Term::Blank(id),
			Self::Iri(iri) => Term::Iri(iri),
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

impl<I: fmt::Display, B: fmt::Display> fmt::Display for Subject<I, B> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			Self::Blank(id) => id.fmt(f),
			Self::Iri(iri) => write!(f, "<{}>", iri),
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
