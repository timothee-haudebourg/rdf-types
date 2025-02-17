use crate::{Literal, LiteralRef, RdfDisplay};
use iref::{Iri, IriBuf};
use std::hash::Hash;
use std::{borrow::Cow, fmt};

mod r#ref;
pub use r#ref::*;

mod cow;
pub use cow::*;

mod local;
pub use local::*;

pub mod generator;
pub use generator::{Generator, LocalGenerator};

/// Lexical representation of an RDF resource.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum Term {
	/// IRI.
	Iri(IriBuf),

	/// Literal value.
	Literal(Literal),
}

impl Term {
	pub fn is_iri(&self) -> bool {
		matches!(self, Self::Iri(_))
	}

	pub fn is_literal(&self) -> bool {
		matches!(self, Self::Literal(_))
	}

	pub fn as_literal(&self) -> Option<LiteralRef> {
		match self {
			Self::Literal(lit) => Some(lit.as_ref()),
			_ => None,
		}
	}

	pub fn into_literal(self) -> Option<Literal> {
		match self {
			Self::Literal(lit) => Some(lit),
			_ => None,
		}
	}

	pub fn try_into_literal(self) -> Result<Literal, IriBuf> {
		match self {
			Self::Literal(lit) => Ok(lit),
			Self::Iri(id) => Err(id),
		}
	}

	pub fn as_iri(&self) -> Option<&Iri> {
		match self {
			Self::Iri(id) => Some(id),
			_ => None,
		}
	}

	pub fn try_into_iri(self) -> Result<IriBuf, Self> {
		match self {
			Self::Iri(iri) => Ok(iri),
			other => Err(other),
		}
	}

	pub fn into_iri(self) -> Option<IriBuf> {
		self.try_into_iri().ok()
	}

	pub fn as_ref(&self) -> TermRef {
		match self {
			Self::Iri(id) => TermRef::Iri(id),
			Self::Literal(l) => TermRef::Literal(l.as_ref()),
		}
	}

	pub fn as_cow(&self) -> CowTerm {
		match self {
			Self::Iri(id) => CowTerm::Iri(Cow::Borrowed(id)),
			Self::Literal(l) => CowTerm::Literal(l.as_cow()),
		}
	}

	pub fn into_cow(self) -> CowTerm<'static> {
		match self {
			Self::Iri(id) => CowTerm::Iri(Cow::Owned(id)),
			Self::Literal(l) => CowTerm::Literal(l.into_cow()),
		}
	}
}

impl Hash for Term {
	fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
		match self {
			Self::Iri(id) => id.hash(state),
			Self::Literal(l) => l.hash(state),
		}
	}
}

impl fmt::Display for Term {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			Self::Iri(id) => id.fmt(f),
			Self::Literal(lit) => lit.fmt(f),
		}
	}
}

impl RdfDisplay for Term {
	fn rdf_fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			Self::Iri(id) => id.rdf_fmt(f),
			Self::Literal(lit) => lit.rdf_fmt(f),
		}
	}
}
