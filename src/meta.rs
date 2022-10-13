use crate::{BlankIdBuf, GraphLabel, Quad, StringLiteral, Subject, Triple};
use iref::IriBuf;
use langtag::LanguageTagBuf;
use locspan::{Meta, Strip};
use locspan_derive::*;
use std::fmt;

#[cfg(feature = "contextual")]
use contextual::DisplayWithContext;

/// gRDF term with literal with metadata.
pub type Term<M> = crate::Term<IriBuf, BlankIdBuf, Literal<M>>;

/// RDF object with literal with metadata.
pub type Object<M> = crate::Object<IriBuf, BlankIdBuf, Literal<M>>;

/// gRDF term with metadata.
pub type MetaTerm<M> = Meta<Term<M>, M>;

/// RDF object with metadata.
pub type MetaObject<M> = Meta<Object<M>, M>;

/// Quad with metadata.
pub type MetaQuad<S, P, O, G, M> = Meta<Quad<Meta<S, M>, Meta<P, M>, Meta<O, M>, Meta<G, M>>, M>;

/// RDF quad with metadata.
pub type MetaRdfQuad<M> =
	Meta<Quad<Meta<Subject, M>, Meta<IriBuf, M>, MetaObject<M>, Meta<GraphLabel, M>>, M>;

/// gRDF quad with metadata.
pub type MetaGrdfQuad<M> = Meta<Quad<MetaTerm<M>, MetaTerm<M>, MetaTerm<M>, MetaTerm<M>>, M>;

/// RDF Literal with metadata.
#[derive(
	Clone,
	PartialEq,
	Eq,
	Hash,
	PartialOrd,
	Ord,
	Debug,
	StrippedPartialEq,
	StrippedEq,
	StrippedPartialOrd,
	StrippedOrd,
	StrippedHash,
)]
#[stripped_ignore(M)]
#[stripped(S, I, L)]
pub enum Literal<M, S = StringLiteral, I = IriBuf, L = LanguageTagBuf> {
	/// Untyped string literal.
	String(#[stripped_deref] Meta<S, M>),

	/// Typed string literal.
	TypedString(#[stripped_deref] Meta<S, M>, #[stripped_deref] Meta<I, M>),

	/// Language string.
	LangString(#[stripped_deref] Meta<S, M>, #[stripped_deref] Meta<L, M>),
}

impl<M, S, I, L> Literal<M, S, I, L> {
	pub fn is_typed(&self) -> bool {
		matches!(self, Self::TypedString(_, _))
	}

	pub fn ty(&self) -> Option<&Meta<I, M>> {
		match self {
			Self::TypedString(_, ty) => Some(ty),
			_ => None,
		}
	}

	pub fn is_lang_string(&self) -> bool {
		matches!(self, Self::LangString(_, _))
	}

	pub fn lang_tag(&self) -> Option<&Meta<L, M>> {
		match self {
			Self::LangString(_, tag) => Some(tag),
			_ => None,
		}
	}

	pub fn string_literal(&self) -> &Meta<S, M> {
		match self {
			Self::String(s) => s,
			Self::TypedString(s, _) => s,
			Self::LangString(s, _) => s,
		}
	}

	pub fn into_string_literal(self) -> Meta<S, M> {
		match self {
			Self::String(s) => s,
			Self::TypedString(s, _) => s,
			Self::LangString(s, _) => s,
		}
	}

	pub fn strip(self) -> super::Literal<S, I, L> {
		match self {
			Self::String(Meta(lit, _)) => super::Literal::String(lit),
			Self::TypedString(Meta(lit, _), Meta(iri_ref, _)) => {
				super::Literal::TypedString(lit, iri_ref)
			}
			Self::LangString(Meta(lit, _), Meta(tag, _)) => super::Literal::LangString(lit, tag),
		}
	}
}

impl<M, S, I, L> Strip for Literal<M, S, I, L> {
	type Stripped = super::Literal<S, I, L>;

	fn strip(self) -> Self::Stripped {
		self.strip()
	}
}

impl<M, S: fmt::Display, I: fmt::Display, L: fmt::Display> fmt::Display for Literal<M, S, I, L> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			Self::String(s) => s.value().fmt(f),
			Self::TypedString(s, ty) => write!(f, "{}^^<{}>", s.value(), ty.value()),
			Self::LangString(s, tag) => write!(f, "{}@{}", s.value(), tag.value()),
		}
	}
}

#[cfg(feature = "contextual")]
impl<M, S: fmt::Display, I, L: fmt::Display, V: crate::IriVocabulary<Iri = I>> DisplayWithContext<V>
	for Literal<M, S, I, L>
{
	fn fmt_with(&self, vocabulary: &V, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			Self::String(s) => s.value().fmt(f),
			Self::TypedString(s, ty) => write!(
				f,
				"{}^^<{}>",
				s.value(),
				vocabulary.iri(ty.value()).unwrap()
			),
			Self::LangString(s, tag) => write!(f, "{}@{}", s.value(), tag.value()),
		}
	}
}

impl<I, B, L: Strip> super::Term<I, B, L> {
	pub fn strip(self) -> super::Term<I, B, L::Stripped> {
		match self {
			Self::Blank(id) => super::Term::Blank(id),
			Self::Iri(iri) => super::Term::Iri(iri),
			Self::Literal(lit) => super::Term::Literal(lit.strip()),
		}
	}
}

impl<I, B, L: Strip> Strip for super::Term<I, B, L> {
	type Stripped = super::Term<I, B, L::Stripped>;

	fn strip(self) -> Self::Stripped {
		self.strip()
	}
}

impl<S: Strip, P: Strip, O: Strip> Strip for Triple<S, P, O> {
	type Stripped = Triple<S::Stripped, P::Stripped, O::Stripped>;

	fn strip(self) -> Self::Stripped {
		Triple(self.0.strip(), self.1.strip(), self.2.strip())
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
