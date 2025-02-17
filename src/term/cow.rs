use std::borrow::Cow;

use iref::Iri;

use crate::CowLiteral;

use super::Term;

pub enum CowTerm<'a> {
	Iri(Cow<'a, Iri>),

	Literal(CowLiteral<'a>),
}

impl<'a> CowTerm<'a> {
	pub fn into_owned(self) -> Term {
		match self {
			Self::Iri(iri) => Term::Iri(iri.into_owned()),
			Self::Literal(l) => Term::Literal(l.into_owned()),
		}
	}
}

impl<'a> From<Term> for CowTerm<'a> {
	fn from(value: Term) -> Self {
		value.into_cow()
	}
}
