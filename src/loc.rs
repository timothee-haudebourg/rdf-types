use crate::StringLiteral;
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

/// Located gRDF subject.
#[derive(Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Debug)]
pub enum Subject<F> {
	/// Blank node identifier.
	Blank(BlankIdBuf),

	/// IRI reference.
	IriRef(IriRefBuf),
}

pub type Object<F> = Term<F>;

pub type GraphLabel<F> = Subject<F>;
