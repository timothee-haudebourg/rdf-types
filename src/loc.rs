use crate::{BlankIdBuf, StringLiteral};
use iref::IriBuf;
use langtag::LanguageTagBuf;
use locspan::Loc;
use std::fmt;

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
