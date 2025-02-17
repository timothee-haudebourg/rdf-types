use std::borrow::Cow;

use iref::Iri;

use crate::CowLiteral;

use super::Term;

pub enum CowTerm<'a> {
	Iri(Cow<'a, Iri>),

	Literal(CowLiteral<'a>),
}

impl CowTerm<'_> {
	pub fn into_owned(self) -> Term {
		match self {
			Self::Iri(iri) => Term::Iri(iri.into_owned()),
			Self::Literal(l) => Term::Literal(l.into_owned()),
		}
	}
}

impl From<Term> for CowTerm<'_> {
	fn from(value: Term) -> Self {
		value.into_cow()
	}
}
