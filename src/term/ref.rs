use std::cmp::Ordering;

use crate::BlankId;

use super::{GroundTermRef, Term};

/// Lexical RDF term reference.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum LocalTermRef<'a> {
	Anonymous(&'a BlankId),

	Named(GroundTermRef<'a>),
}

impl LocalTermRef<'_> {
	pub fn to_owned(self) -> Term {
		match self {
			Self::Anonymous(blank_id) => Term::BlankId(blank_id.to_owned()),
			Self::Named(named) => Term::Ground(named.to_owned()),
		}
	}
}

impl PartialEq<Term> for LocalTermRef<'_> {
	fn eq(&self, other: &Term) -> bool {
		match (self, other) {
			(Self::Anonymous(a), Term::BlankId(b)) => *a == b,
			(Self::Named(a), Term::Ground(b)) => a == b,
			_ => false,
		}
	}
}

impl PartialOrd<Term> for LocalTermRef<'_> {
	fn partial_cmp(&self, other: &Term) -> Option<Ordering> {
		match (self, other) {
			(Self::Anonymous(a), Term::BlankId(b)) => (*a).partial_cmp(b),
			(Self::Anonymous(_), Term::Ground(_)) => Some(Ordering::Less),
			(Self::Named(_), Term::BlankId(_)) => Some(Ordering::Greater),
			(Self::Named(a), Term::Ground(b)) => (*a).partial_cmp(b),
		}
	}
}
