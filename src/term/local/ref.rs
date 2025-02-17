use crate::{BlankId, TermRef};
use std::cmp::Ordering;

use super::LocalTerm;

/// Lexical RDF term reference.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum LocalTermRef<'a> {
	Anonymous(&'a BlankId),

	Named(TermRef<'a>),
}

impl LocalTermRef<'_> {
	pub fn to_owned(self) -> LocalTerm {
		match self {
			Self::Anonymous(blank_id) => LocalTerm::Anonymous(blank_id.to_owned()),
			Self::Named(named) => LocalTerm::Named(named.to_owned()),
		}
	}
}

impl PartialEq<LocalTerm> for LocalTermRef<'_> {
	fn eq(&self, other: &LocalTerm) -> bool {
		match (self, other) {
			(Self::Anonymous(a), LocalTerm::Anonymous(b)) => *a == b,
			(Self::Named(a), LocalTerm::Named(b)) => a == b,
			_ => false,
		}
	}
}

impl PartialOrd<LocalTerm> for LocalTermRef<'_> {
	fn partial_cmp(&self, other: &LocalTerm) -> Option<Ordering> {
		match (self, other) {
			(Self::Anonymous(a), LocalTerm::Anonymous(b)) => (*a).partial_cmp(b),
			(Self::Anonymous(_), LocalTerm::Named(_)) => Some(Ordering::Less),
			(Self::Named(_), LocalTerm::Anonymous(_)) => Some(Ordering::Greater),
			(Self::Named(a), LocalTerm::Named(b)) => (*a).partial_cmp(b),
		}
	}
}
