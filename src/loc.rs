use crate::{BlankIdBuf, GraphLabel, Quad, StringLiteral, Subject, Triple};
use iref::IriBuf;
use langtag::LanguageTagBuf;
use locspan::{Loc, Strip, StrippedPartialEq};
use std::fmt;

/// Located quad.
pub type LocQuad<S, P, O, G, F> = Loc<Quad<Loc<S, F>, Loc<P, F>, Loc<O, F>, Loc<G, F>>, F>;

/// Located RDF quad.
pub type LocRdfQuad<F> =
	Loc<Quad<Loc<Subject, F>, Loc<IriBuf, F>, Loc<Object<F>, F>, Loc<GraphLabel, F>>, F>;

/// Located gRDF quad.
pub type LocGrdfQuad<F> =
	Loc<Quad<Loc<Term<F>, F>, Loc<Term<F>, F>, Loc<Term<F>, F>, Loc<Term<F>, F>>, F>;

/// Located RDF Literal.
#[derive(Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Debug)]
pub enum Literal<F> {
	/// Untyped string literal.
	String(Loc<StringLiteral, F>),

	/// Typed string literal.
	TypedString(Loc<StringLiteral, F>, Loc<IriBuf, F>),

	/// Language string.
	LangString(Loc<StringLiteral, F>, Loc<LanguageTagBuf, F>),
}

impl<F> Literal<F> {
	pub fn strip(self) -> super::Literal {
		match self {
			Self::String(Loc(lit, _)) => super::Literal::String(lit),
			Self::TypedString(Loc(lit, _), Loc(iri_ref, _)) => {
				super::Literal::TypedString(lit, iri_ref)
			}
			Self::LangString(Loc(lit, _), Loc(tag, _)) => super::Literal::LangString(lit, tag),
		}
	}
}

impl<F> Strip for Literal<F> {
	type Stripped = super::Literal;

	fn strip(self) -> Self::Stripped {
		self.strip()
	}
}

impl<F> StrippedPartialEq for Literal<F> {
	fn stripped_eq(&self, other: &Self) -> bool {
		match (self, other) {
			(Self::String(Loc(a, _)), Self::String(Loc(b, _))) => a == b,
			(
				Self::TypedString(Loc(a, _), Loc(a_ty, _)),
				Self::TypedString(Loc(b, _), Loc(b_ty, _)),
			) => a == b && a_ty == b_ty,
			(
				Self::LangString(Loc(a, _), Loc(a_tag, _)),
				Self::LangString(Loc(b, _), Loc(b_tag, _)),
			) => a == b && a_tag == b_tag,
			_ => false,
		}
	}
}

impl<F> fmt::Display for Literal<F> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			Self::String(s) => s.value().fmt(f),
			Self::TypedString(s, ty) => write!(f, "{}^^<{}>", s.value(), ty.value()),
			Self::LangString(s, tag) => write!(f, "{}@{}", s.value(), tag.value()),
		}
	}
}

/// Located gRDF term.
#[derive(Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Debug)]
pub enum Term<F> {
	/// Blank node identifier.
	Blank(BlankIdBuf),

	/// IRI reference.
	Iri(IriBuf),

	/// Literal value.
	Literal(Literal<F>),
}

impl<F> Term<F> {
	pub fn strip(self) -> super::Term {
		match self {
			Self::Blank(id) => super::Term::Blank(id),
			Self::Iri(iri) => super::Term::Iri(iri),
			Self::Literal(lit) => super::Term::Literal(lit.strip()),
		}
	}
}

impl<F> StrippedPartialEq for Term<F> {
	fn stripped_eq(&self, other: &Self) -> bool {
		match (self, other) {
			(Self::Blank(a), Self::Blank(b)) => a == b,
			(Self::Iri(a), Self::Iri(b)) => a == b,
			(Self::Literal(a), Self::Literal(b)) => a.stripped_eq(b),
			_ => false,
		}
	}
}

impl<F> Strip for Term<F> {
	type Stripped = super::Term;

	fn strip(self) -> Self::Stripped {
		self.strip()
	}
}

impl<F> fmt::Display for Term<F> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			Self::Blank(id) => id.fmt(f),
			Self::Iri(iri) => write!(f, "<{}>", iri),
			Self::Literal(lit) => lit.fmt(f),
		}
	}
}

pub type Object<F> = Term<F>;

impl<S: Strip, P: Strip, O: Strip> Strip for Triple<S, P, O> {
	type Stripped = Triple<S::Stripped, P::Stripped, O::Stripped>;

	fn strip(self) -> Self::Stripped {
		Triple(self.0.strip(), self.1.strip(), self.2.strip())
	}
}

impl<S: StrippedPartialEq, P: StrippedPartialEq, O: StrippedPartialEq> StrippedPartialEq
	for Triple<S, P, O>
{
	fn stripped_eq(&self, other: &Self) -> bool {
		self.0.stripped_eq(&other.0) && self.1.stripped_eq(&other.1) && self.2.stripped_eq(&other.2)
	}
}

impl<S: Strip, P: Strip, O: Strip, G: Strip> Strip for Quad<S, P, O, G> {
	type Stripped = Quad<S::Stripped, P::Stripped, O::Stripped, G::Stripped>;

	fn strip(self) -> Self::Stripped {
		Quad(
			self.0.strip(),
			self.1.strip(),
			self.2.strip(),
			self.3.strip(),
		)
	}
}

impl<S: StrippedPartialEq, P: StrippedPartialEq, O: StrippedPartialEq, G: StrippedPartialEq>
	StrippedPartialEq for Quad<S, P, O, G>
{
	fn stripped_eq(&self, other: &Self) -> bool {
		self.0.stripped_eq(&other.0)
			&& self.1.stripped_eq(&other.1)
			&& self.2.stripped_eq(&other.2)
			&& self.3.stripped_eq(&other.3)
	}
}
