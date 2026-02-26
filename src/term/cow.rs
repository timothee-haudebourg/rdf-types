use iref::{Iri, IriBuf};

use crate::{BlankId, BlankIdBuf, CowLiteral, GroundTerm, Id, Literal, LiteralRef};
use std::borrow::Cow;

use super::{CowGroundTerm, Term};

/// Copy-on-write term.
pub enum CowTerm<'a> {
	/// Blank node identifier.
	BlankId(Cow<'a, BlankId>),

	/// Ground term.
	Ground(CowGroundTerm<'a>),
}

impl<'a> CowTerm<'a> {
	pub fn into_owned(self) -> Term {
		match self {
			Self::BlankId(b) => Term::BlankId(b.into_owned()),
			Self::Ground(t) => Term::Ground(t.into_owned()),
		}
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
