use std::cmp::Ordering;

use iref::Iri;

use crate::LiteralRef;

use super::GroundTerm;

/// Lexical RDF term reference.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum GroundTermRef<'a> {
	Iri(&'a Iri),

	Literal(LiteralRef<'a>),
}

impl GroundTermRef<'_> {
	pub fn to_owned(self) -> GroundTerm {
		match self {
			Self::Iri(iri) => GroundTerm::Iri(iri.to_owned()),
			Self::Literal(l) => GroundTerm::Literal(l.to_owned()),
		}
	}
}

impl PartialEq<GroundTerm> for GroundTermRef<'_> {
	fn eq(&self, other: &GroundTerm) -> bool {
		match (self, other) {
			(Self::Iri(a), GroundTerm::Iri(b)) => *a == b,
			(Self::Literal(a), GroundTerm::Literal(b)) => a == b,
			_ => false,
		}
	}
}

impl PartialOrd<GroundTerm> for GroundTermRef<'_> {
	fn partial_cmp(&self, other: &GroundTerm) -> Option<Ordering> {
		match (self, other) {
			(Self::Iri(a), GroundTerm::Iri(b)) => (*a).partial_cmp(b),
			(Self::Iri(_), GroundTerm::Literal(_)) => Some(Ordering::Less),
			(Self::Literal(_), GroundTerm::Iri(_)) => Some(Ordering::Greater),
			(Self::Literal(a), GroundTerm::Literal(b)) => (*a).partial_cmp(b),
		}
	}
}
