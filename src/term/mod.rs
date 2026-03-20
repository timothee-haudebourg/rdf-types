use std::borrow::Cow;
use std::cmp::Ordering;
use std::fmt;
use std::hash::Hash;

use iref::{Iri, IriBuf};

use crate::{BlankId, BlankIdBuf, Id, IdRef, Literal, LiteralRef, RdfDisplay};

mod cow;
pub mod generator;
mod ground;
mod r#ref;

pub use cow::*;
pub use generator::Generator;
pub use ground::*;
pub use r#ref::*;

/// Term.
///
/// Lexical representation of an RDF resource.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum Term {
	/// Blank identifier.
	BlankId(BlankIdBuf),

	/// Ground term.
	Ground(GroundTerm),
}

impl Term {
	pub fn iri(iri: IriBuf) -> Self {
		Self::Ground(GroundTerm::Iri(iri))
	}

	pub fn literal(literal: Literal) -> Self {
		Self::Ground(GroundTerm::Literal(literal))
	}

	pub fn id(id: Id) -> Self {
		match id {
			Id::Iri(iri) => Self::iri(iri),
			Id::BlankId(b) => Self::BlankId(b),
		}
	}

	pub fn is_id(&self) -> bool {
		matches!(self, Self::BlankId(_) | Self::Ground(GroundTerm::Iri(_)))
	}

	pub fn as_id(&self) -> Option<IdRef<'_>> {
		match self {
			Self::BlankId(b) => Some(IdRef::BlankId(b)),
			Self::Ground(GroundTerm::Iri(iri)) => Some(IdRef::Iri(iri)),
			_ => None,
		}
	}

	pub fn is_blank_id(&self) -> bool {
		matches!(self, Self::BlankId(_))
	}

	pub fn is_ground(&self) -> bool {
		matches!(self, Self::Ground(_))
	}

	pub fn as_blank_id(&self) -> Option<&BlankId> {
		match self {
			Self::BlankId(b) => Some(b),
			Self::Ground(_) => None,
		}
	}

	pub fn as_ground(&self) -> Option<&GroundTerm> {
		match self {
			Self::Ground(g) => Some(g),
			_ => None,
		}
	}

	pub fn into_ground(self) -> Option<GroundTerm> {
		match self {
			Self::Ground(g) => Some(g),
			_ => None,
		}
	}

	pub fn is_iri(&self) -> bool {
		matches!(self, Self::Ground(GroundTerm::Iri(_)))
	}

	pub fn as_iri(&self) -> Option<&Iri> {
		match self {
			Self::Ground(t) => t.as_iri(),
			Self::BlankId(_) => None,
		}
	}

	pub fn into_iri(self) -> Result<IriBuf, Self> {
		match self {
			Self::Ground(GroundTerm::Iri(iri)) => Ok(iri),
			other => Err(other),
		}
	}

	pub fn is_literal(&self) -> bool {
		matches!(self, Self::Ground(GroundTerm::Literal(_)))
	}

	pub fn as_literal(&self) -> Option<LiteralRef<'_>> {
		match self {
			Self::Ground(t) => t.as_literal(),
			Self::BlankId(_) => None,
		}
	}

	pub fn as_ref(&self) -> TermRef<'_> {
		match self {
			Self::BlankId(blank_id) => TermRef::BlankId(blank_id),
			Self::Ground(named) => TermRef::Ground(named.as_ref()),
		}
	}

	pub fn as_cow(&self) -> CowTerm<'_> {
		match self {
			Self::BlankId(blank_id) => CowTerm::BlankId(Cow::Borrowed(blank_id)),
			Self::Ground(named) => CowTerm::Ground(named.as_cow()),
		}
	}

	pub fn into_cow(self) -> CowTerm<'static> {
		match self {
			Self::BlankId(blank_id) => CowTerm::BlankId(Cow::Owned(blank_id)),
			Self::Ground(named) => CowTerm::Ground(named.into_cow()),
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

impl<'a> PartialEq<TermRef<'a>> for Term {
	fn eq(&self, other: &TermRef<'a>) -> bool {
		match (self, other) {
			(Self::BlankId(a), TermRef::BlankId(b)) => a == *b,
			(Self::Ground(a), TermRef::Ground(b)) => a == b,
			_ => false,
		}
	}
}

impl<'a> PartialOrd<TermRef<'a>> for Term {
	fn partial_cmp(&self, other: &TermRef<'a>) -> Option<Ordering> {
		match (self, other) {
			(Self::BlankId(a), TermRef::BlankId(b)) => a.as_blank_id().partial_cmp(b),
			(Self::BlankId(_), TermRef::Ground(_)) => Some(Ordering::Less),
			(Self::Ground(_), TermRef::BlankId(_)) => Some(Ordering::Greater),
			(Self::Ground(a), TermRef::Ground(b)) => a.partial_cmp(b),
		}
	}
}
