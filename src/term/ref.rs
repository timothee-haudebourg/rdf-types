use core::fmt;
use std::borrow::Cow;
use std::cmp::Ordering;

use iref::Iri;

use crate::{BlankId, IdRef, LiteralRef, RdfDisplay};

use super::{CowTerm, GroundTermRef, Term};

/// Term reference.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum TermRef<'a> {
	/// Blank identifier.
	BlankId(&'a BlankId),

	/// Ground term.
	Ground(GroundTermRef<'a>),
}

impl<'a> TermRef<'a> {
	pub fn is_id(&self) -> bool {
		matches!(self, Self::BlankId(_) | Self::Ground(GroundTermRef::Iri(_)))
	}

	pub fn as_id(&self) -> Option<IdRef<'a>> {
		match self {
			Self::BlankId(b) => Some(IdRef::BlankId(b)),
			Self::Ground(GroundTermRef::Iri(iri)) => Some(IdRef::Iri(iri)),
			_ => None,
		}
	}

	pub fn into_id(self) -> Result<IdRef<'a>, LiteralRef<'a>> {
		match self {
			Self::BlankId(b) => Ok(IdRef::BlankId(b)),
			Self::Ground(GroundTermRef::Iri(iri)) => Ok(IdRef::Iri(iri)),
			Self::Ground(GroundTermRef::Literal(lit)) => Err(lit),
		}
	}

	pub fn is_blank_id(&self) -> bool {
		matches!(self, Self::BlankId(_))
	}

	pub fn is_ground(&self) -> bool {
		matches!(self, Self::Ground(_))
	}

	pub fn is_iri(&self) -> bool {
		matches!(self, Self::Ground(GroundTermRef::Iri(_)))
	}

	pub fn is_literal(&self) -> bool {
		matches!(self, Self::Ground(GroundTermRef::Literal(_)))
	}

	pub fn as_blank_id(&self) -> Option<&'a BlankId> {
		match self {
			Self::BlankId(b) => Some(b),
			_ => None,
		}
	}

	pub fn as_ground(&self) -> Option<GroundTermRef<'a>> {
		match self {
			Self::Ground(g) => Some(*g),
			_ => None,
		}
	}

	pub fn as_iri(&self) -> Option<&'a Iri> {
		match self {
			Self::Ground(g) => g.as_iri(),
			_ => None,
		}
	}

	pub fn as_literal(&self) -> Option<LiteralRef<'a>> {
		match self {
			Self::Ground(g) => g.as_literal(),
			_ => None,
		}
	}

	pub fn into_cow(self) -> CowTerm<'a> {
		match self {
			Self::BlankId(b) => CowTerm::BlankId(Cow::Borrowed(b)),
			Self::Ground(g) => CowTerm::Ground(g.into_cow()),
		}
	}
}

impl TermRef<'_> {
	/// Clones the referenced term.
	pub fn to_owned(self) -> Term {
		match self {
			Self::BlankId(blank_id) => Term::BlankId(blank_id.to_owned()),
			Self::Ground(named) => Term::Ground(named.to_owned()),
		}
	}
}

impl From<TermRef<'_>> for Term {
	fn from(value: TermRef<'_>) -> Self {
		value.to_owned()
	}
}

impl PartialEq<Term> for TermRef<'_> {
	fn eq(&self, other: &Term) -> bool {
		match (self, other) {
			(Self::BlankId(a), Term::BlankId(b)) => *a == b,
			(Self::Ground(a), Term::Ground(b)) => a == b,
			_ => false,
		}
	}
}

impl PartialOrd<Term> for TermRef<'_> {
	fn partial_cmp(&self, other: &Term) -> Option<Ordering> {
		match (self, other) {
			(Self::BlankId(a), Term::BlankId(b)) => (*a).partial_cmp(b),
			(Self::BlankId(_), Term::Ground(_)) => Some(Ordering::Less),
			(Self::Ground(_), Term::BlankId(_)) => Some(Ordering::Greater),
			(Self::Ground(a), Term::Ground(b)) => (*a).partial_cmp(b),
		}
	}
}

impl fmt::Display for TermRef<'_> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			Self::BlankId(id) => id.fmt(f),
			Self::Ground(g) => g.fmt(f),
		}
	}
}

impl RdfDisplay for TermRef<'_> {
	fn rdf_fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			Self::BlankId(id) => id.rdf_fmt(f),
			Self::Ground(g) => g.rdf_fmt(f),
		}
	}
}
