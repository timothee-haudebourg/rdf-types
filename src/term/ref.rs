use std::cmp::Ordering;

use crate::LiteralRef;
use iref::Iri;

use super::Term;

/// Lexical RDF term reference.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum TermRef<'a> {
	Iri(&'a Iri),

	Literal(LiteralRef<'a>),
}

impl<'a> TermRef<'a> {
	pub fn to_owned(self) -> Term {
		match self {
			Self::Iri(iri) => Term::Iri(iri.to_owned()),
			Self::Literal(l) => Term::Literal(l.to_owned()),
		}
	}
}

impl<'a> PartialEq<Term> for TermRef<'a> {
	fn eq(&self, other: &Term) -> bool {
		match (self, other) {
			(Self::Iri(a), Term::Iri(b)) => *a == b,
			(Self::Literal(a), Term::Literal(b)) => a == b,
			_ => false,
		}
	}
}

impl<'a> PartialOrd<Term> for TermRef<'a> {
	fn partial_cmp(&self, other: &Term) -> Option<Ordering> {
		match (self, other) {
			(Self::Iri(a), Term::Iri(b)) => (*a).partial_cmp(b),
			(Self::Iri(_), Term::Literal(_)) => Some(Ordering::Less),
			(Self::Literal(_), Term::Iri(_)) => Some(Ordering::Greater),
			(Self::Literal(a), Term::Literal(b)) => (*a).partial_cmp(b),
		}
	}
}
