use crate::{BlankId, BlankIdBuf, Literal, LiteralRef, RdfDisplay};
use std::borrow::Cow;
use std::fmt;
use std::hash::Hash;

mod r#ref;
use iref::{Iri, IriBuf};
pub use r#ref::*;

mod cow;
pub use cow::*;

use super::Term;

/// Lexical representation of an RDF resource.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum LocalTerm {
	Anonymous(BlankIdBuf),
	Named(Term),
}

impl LocalTerm {
	pub fn iri(iri: IriBuf) -> Self {
		Self::Named(Term::Iri(iri))
	}

	pub fn literal(literal: Literal) -> Self {
		Self::Named(Term::Literal(literal))
	}

	pub fn is_anonymous(&self) -> bool {
		matches!(self, Self::Anonymous(_))
	}

	pub fn is_blank_id(&self) -> bool {
		self.is_anonymous()
	}

	pub fn as_anonymous(&self) -> Option<&BlankId> {
		match self {
			Self::Anonymous(b) => Some(b),
			Self::Named(_) => None,
		}
	}

	pub fn as_blank_id(&self) -> Option<&BlankId> {
		self.as_anonymous()
	}

	pub fn as_iri(&self) -> Option<&Iri> {
		match self {
			Self::Named(t) => t.as_iri(),
			Self::Anonymous(_) => None,
		}
	}

	pub fn as_literal(&self) -> Option<LiteralRef> {
		match self {
			Self::Named(t) => t.as_literal(),
			Self::Anonymous(_) => None,
		}
	}

	pub fn as_ref(&self) -> LocalTermRef {
		match self {
			Self::Anonymous(blank_id) => LocalTermRef::Anonymous(blank_id),
			Self::Named(named) => LocalTermRef::Named(named.as_ref()),
		}
	}

	pub fn as_cow(&self) -> CowLocalTerm {
		match self {
			Self::Anonymous(blank_id) => CowLocalTerm::Anonymous(Cow::Borrowed(blank_id)),
			Self::Named(named) => CowLocalTerm::Named(named.as_cow()),
		}
	}

	pub fn into_cow(self) -> CowLocalTerm<'static> {
		match self {
			Self::Anonymous(blank_id) => CowLocalTerm::Anonymous(Cow::Owned(blank_id)),
			Self::Named(named) => CowLocalTerm::Named(named.into_cow()),
		}
	}
}

impl From<IriBuf> for LocalTerm {
	fn from(value: IriBuf) -> Self {
		Self::iri(value)
	}
}

impl From<Literal> for LocalTerm {
	fn from(value: Literal) -> Self {
		Self::literal(value)
	}
}

impl From<Term> for LocalTerm {
	fn from(value: Term) -> Self {
		Self::Named(value)
	}
}

impl From<BlankIdBuf> for LocalTerm {
	fn from(value: BlankIdBuf) -> Self {
		Self::Anonymous(value)
	}
}

impl Hash for LocalTerm {
	fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
		match self {
			Self::Anonymous(id) => id.hash(state),
			Self::Named(l) => l.hash(state),
		}
	}
}

impl fmt::Display for LocalTerm {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			Self::Anonymous(id) => id.fmt(f),
			Self::Named(lit) => lit.fmt(f),
		}
	}
}

impl RdfDisplay for LocalTerm {
	fn rdf_fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			Self::Anonymous(id) => id.rdf_fmt(f),
			Self::Named(lit) => lit.rdf_fmt(f),
		}
	}
}
