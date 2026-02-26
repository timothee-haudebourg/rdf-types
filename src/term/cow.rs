use core::fmt;
use std::borrow::Cow;

use iref::{Iri, IriBuf};

use crate::{
	BlankId, BlankIdBuf, CowLiteral, GroundTerm, Id, Literal, LiteralRef, RdfDisplay, TermRef,
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
