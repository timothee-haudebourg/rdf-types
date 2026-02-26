use std::cmp::Ordering;

use crate::BlankId;

use super::{GroundTermRef, Term};

/// Term reference.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum TermRef<'a> {
	/// Blank identifier.
	BlankId(&'a BlankId),

	/// Ground term.
	Ground(GroundTermRef<'a>),
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
