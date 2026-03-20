use core::fmt;
use std::borrow::Cow;
use std::cmp::Ordering;

use iref::{Iri, IriBuf};

use crate::{
	BlankId, BlankIdBuf, CowId, CowLiteral, GroundTerm, Id, IdRef, Literal, LiteralRef, RdfDisplay,
	TermRef,
};

use super::{CowGroundTerm, Term};

/// Copy-on-write term.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum CowTerm<'a> {
	/// Blank node identifier.
	BlankId(Cow<'a, BlankId>),

	/// Ground term.
	Ground(CowGroundTerm<'a>),
}

impl CowTerm<'_> {
	pub fn is_id(&self) -> bool {
		matches!(self, Self::BlankId(_) | Self::Ground(CowGroundTerm::Iri(_)))
	}

	pub fn as_id(&self) -> Option<IdRef<'_>> {
		match self {
			Self::BlankId(b) => Some(IdRef::BlankId(b)),
			Self::Ground(CowGroundTerm::Iri(iri)) => Some(IdRef::Iri(iri)),
			_ => None,
		}
	}

	pub fn is_blank_id(&self) -> bool {
		matches!(self, Self::BlankId(_))
	}

	pub fn is_ground(&self) -> bool {
		matches!(self, Self::Ground(_))
	}

	pub fn is_iri(&self) -> bool {
		matches!(self, Self::Ground(CowGroundTerm::Iri(_)))
	}

	pub fn is_literal(&self) -> bool {
		matches!(self, Self::Ground(CowGroundTerm::Literal(_)))
	}

	pub fn as_blank_id(&self) -> Option<&BlankId> {
		match self {
			Self::BlankId(b) => Some(b),
			_ => None,
		}
	}

	pub fn as_iri(&self) -> Option<&Iri> {
		match self {
			Self::Ground(g) => g.as_iri(),
			_ => None,
		}
	}

	pub fn as_ground(&self) -> Option<&CowGroundTerm<'_>> {
		match self {
			Self::Ground(g) => Some(g),
			_ => None,
		}
	}

	pub fn as_literal(&self) -> Option<LiteralRef<'_>> {
		match self {
			Self::Ground(g) => g.as_literal(),
			_ => None,
		}
	}

	pub fn as_ref(&self) -> TermRef<'_> {
		match self {
			Self::BlankId(b) => TermRef::BlankId(b),
			Self::Ground(g) => TermRef::Ground(g.as_ref()),
		}
	}

	pub fn into_owned(self) -> Term {
		match self {
			Self::BlankId(b) => Term::BlankId(b.into_owned()),
			Self::Ground(t) => Term::Ground(t.into_owned()),
		}
	}
}

impl<'a> CowTerm<'a> {
	pub fn into_ground(self) -> Result<CowGroundTerm<'a>, Cow<'a, BlankId>> {
		match self {
			Self::Ground(g) => Ok(g),
			Self::BlankId(b) => Err(b),
		}
	}

	pub fn into_id(self) -> Result<CowId<'a>, CowLiteral<'a>> {
		match self {
			Self::BlankId(b) => Ok(CowId::BlankId(b)),
			Self::Ground(CowGroundTerm::Iri(iri)) => Ok(CowId::Iri(iri)),
			Self::Ground(CowGroundTerm::Literal(lit)) => Err(lit),
		}
	}
}

impl<'a> From<TermRef<'a>> for CowTerm<'a> {
	fn from(value: TermRef<'a>) -> Self {
		value.into_cow()
	}
}

impl From<CowTerm<'_>> for Term {
	fn from(value: CowTerm<'_>) -> Self {
		value.into_owned()
	}
}

impl From<GroundTerm> for CowTerm<'_> {
	fn from(value: GroundTerm) -> Self {
		Self::Ground(value.into())
	}
}

impl From<Term> for CowTerm<'_> {
	fn from(value: Term) -> Self {
		value.into_cow()
	}
}

impl<'a> From<&'a Term> for CowTerm<'a> {
	fn from(value: &'a Term) -> Self {
		value.as_cow()
	}
}

impl<'a> From<&'a Literal> for CowTerm<'a> {
	fn from(value: &'a Literal) -> Self {
		Self::Ground(value.into())
	}
}

impl<'a> From<CowLiteral<'a>> for CowTerm<'a> {
	fn from(value: CowLiteral<'a>) -> Self {
		Self::Ground(value.into())
	}
}

impl<'a> From<LiteralRef<'a>> for CowTerm<'a> {
	fn from(value: LiteralRef<'a>) -> Self {
		Self::Ground(value.into())
	}
}

impl<'a> From<&'a Id> for CowTerm<'a> {
	fn from(value: &'a Id) -> Self {
		match value {
			Id::BlankId(b) => b.into(),
			Id::Iri(i) => i.into(),
		}
	}
}

impl<'a> From<&'a BlankId> for CowTerm<'a> {
	fn from(value: &'a BlankId) -> Self {
		Self::BlankId(Cow::Borrowed(value))
	}
}

impl<'a> From<&'a BlankIdBuf> for CowTerm<'a> {
	fn from(value: &'a BlankIdBuf) -> Self {
		Self::BlankId(Cow::Borrowed(value))
	}
}

impl<'a> From<&'a Iri> for CowTerm<'a> {
	fn from(value: &'a Iri) -> Self {
		Self::Ground(value.into())
	}
}

impl<'a> From<&'a IriBuf> for CowTerm<'a> {
	fn from(value: &'a IriBuf) -> Self {
		Self::Ground(value.into())
	}
}

impl<'a> From<&'a str> for CowTerm<'a> {
	fn from(value: &'a str) -> Self {
		Self::Ground(value.into())
	}
}

impl<'a> From<&'a String> for CowTerm<'a> {
	fn from(value: &'a String) -> Self {
		Self::Ground(value.into())
	}
}

impl PartialEq<Iri> for CowTerm<'_> {
	fn eq(&self, other: &Iri) -> bool {
		match self {
			Self::Ground(CowGroundTerm::Iri(this)) => this.as_ref() == other,
			_ => false,
		}
	}
}

impl PartialEq<&Iri> for CowTerm<'_> {
	fn eq(&self, other: &&Iri) -> bool {
		self.eq(*other)
	}
}

impl PartialEq<Term> for CowTerm<'_> {
	fn eq(&self, other: &Term) -> bool {
		self.as_ref() == *other
	}
}

impl PartialEq<CowTerm<'_>> for Term {
	fn eq(&self, other: &CowTerm<'_>) -> bool {
		*self == other.as_ref()
	}
}

impl<'a> PartialEq<TermRef<'a>> for CowTerm<'_> {
	fn eq(&self, other: &TermRef<'a>) -> bool {
		self.as_ref() == *other
	}
}

impl PartialEq<CowTerm<'_>> for TermRef<'_> {
	fn eq(&self, other: &CowTerm<'_>) -> bool {
		*self == other.as_ref()
	}
}

impl PartialOrd<Term> for CowTerm<'_> {
	fn partial_cmp(&self, other: &Term) -> Option<Ordering> {
		self.as_ref().partial_cmp(other)
	}
}

impl PartialOrd<CowTerm<'_>> for Term {
	fn partial_cmp(&self, other: &CowTerm<'_>) -> Option<Ordering> {
		self.partial_cmp(&other.as_ref())
	}
}

impl<'a> PartialOrd<TermRef<'a>> for CowTerm<'_> {
	fn partial_cmp(&self, other: &TermRef<'a>) -> Option<Ordering> {
		self.as_ref().partial_cmp(other)
	}
}

impl PartialOrd<CowTerm<'_>> for TermRef<'_> {
	fn partial_cmp(&self, other: &CowTerm<'_>) -> Option<Ordering> {
		self.partial_cmp(&other.as_ref())
	}
}

impl fmt::Display for CowTerm<'_> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			Self::BlankId(id) => id.fmt(f),
			Self::Ground(g) => g.fmt(f),
		}
	}
}

impl RdfDisplay for CowTerm<'_> {
	fn rdf_fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			Self::BlankId(id) => id.rdf_fmt(f),
			Self::Ground(g) => g.rdf_fmt(f),
		}
	}
}
