use std::borrow::Cow;
use std::fmt;
use std::hash::Hash;

use iref::{Iri, IriBuf};

use crate::{BlankId, BlankIdBuf, Id, Literal, LiteralRef, RdfDisplay};

mod cow;
pub mod generator;
mod ground;
mod r#ref;

pub use cow::*;
pub use generator::Generator;
pub use ground::*;
pub use r#ref::*;

/// Lexical representation of an RDF resource.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum Term {
	BlankId(BlankIdBuf),
	Ground(GroundTerm),
}

impl Term {
	pub fn iri(iri: IriBuf) -> Self {
		Self::Ground(GroundTerm::Iri(iri))
	}

	pub fn literal(literal: Literal) -> Self {
		Self::Ground(GroundTerm::Literal(literal))
	}

	pub fn is_blank_id(&self) -> bool {
		matches!(self, Self::BlankId(_))
	}

	pub fn as_blank_id(&self) -> Option<&BlankId> {
		match self {
			Self::BlankId(b) => Some(b),
			Self::Ground(_) => None,
		}
	}

	pub fn as_iri(&self) -> Option<&Iri> {
		match self {
			Self::Ground(t) => t.as_iri(),
			Self::BlankId(_) => None,
		}
	}

	pub fn as_literal(&self) -> Option<LiteralRef> {
		match self {
			Self::Ground(t) => t.as_literal(),
			Self::BlankId(_) => None,
		}
	}

	pub fn as_ref(&self) -> LocalTermRef {
		match self {
			Self::BlankId(blank_id) => LocalTermRef::Anonymous(blank_id),
			Self::Ground(named) => LocalTermRef::Named(named.as_ref()),
		}
	}

	pub fn as_cow(&self) -> CowLocalTerm {
		match self {
			Self::BlankId(blank_id) => CowLocalTerm::Anonymous(Cow::Borrowed(blank_id)),
			Self::Ground(named) => CowLocalTerm::Named(named.as_cow()),
		}
	}

	pub fn into_cow(self) -> CowLocalTerm<'static> {
		match self {
			Self::BlankId(blank_id) => CowLocalTerm::Anonymous(Cow::Owned(blank_id)),
			Self::Ground(named) => CowLocalTerm::Named(named.into_cow()),
		}
	}

	pub fn into_id(self) -> Result<Id, Literal> {
		match self {
			Self::BlankId(blank) => Ok(Id::BlankId(blank)),
			Self::Ground(GroundTerm::Iri(iri)) => Ok(Id::Iri(iri)),
			Self::Ground(GroundTerm::Literal(lit)) => Err(lit),
		}
	}
}

impl From<IriBuf> for Term {
	fn from(value: IriBuf) -> Self {
		Self::iri(value)
	}
}

impl From<Literal> for Term {
	fn from(value: Literal) -> Self {
		Self::literal(value)
	}
}

impl From<Id> for Term {
	fn from(value: Id) -> Self {
		match value {
			Id::BlankId(b) => Self::BlankId(b),
			Id::Iri(i) => Self::Ground(GroundTerm::Iri(i)),
		}
	}
}

impl From<GroundTerm> for Term {
	fn from(value: GroundTerm) -> Self {
		Self::Ground(value)
	}
}

impl From<BlankIdBuf> for Term {
	fn from(value: BlankIdBuf) -> Self {
		Self::BlankId(value)
	}
}

impl Hash for Term {
	fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
		match self {
			Self::BlankId(id) => id.hash(state),
			Self::Ground(l) => l.hash(state),
		}
	}
}

impl fmt::Display for Term {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			Self::BlankId(id) => id.fmt(f),
			Self::Ground(lit) => lit.fmt(f),
		}
	}
}

impl RdfDisplay for Term {
	fn rdf_fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			Self::BlankId(id) => id.rdf_fmt(f),
			Self::Ground(lit) => lit.rdf_fmt(f),
		}
	}
}
