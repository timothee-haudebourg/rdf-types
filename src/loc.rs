use crate::{BlankIdBuf, StringLiteral};
use iref::IriRefBuf;
use langtag::LanguageTagBuf;
use locspan::Loc;

/// Located RDF Literal.
#[derive(Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Debug)]
pub enum Literal<F> {
	/// Untyped string literal.
	String(Loc<StringLiteral, F>),

	/// Typed string literal.
	TypedString(Loc<StringLiteral, F>, Loc<IriRefBuf, F>),

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

/// Located gRDF term.
#[derive(Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Debug)]
pub enum Term<F> {
	/// Blank node identifier.
	Blank(BlankIdBuf),

	/// IRI reference.
	IriRef(IriRefBuf),

	/// Literal value.
	Literal(Literal<F>),
}

impl<F> Term<F> {
	pub fn strip(self) -> super::Term {
		match self {
			Self::Blank(id) => super::Term::Blank(id),
			Self::IriRef(iri_ref) => super::Term::IriRef(iri_ref),
			Self::Literal(lit) => super::Term::Literal(lit.strip()),
		}
	}
}

pub type Object<F> = Term<F>;
