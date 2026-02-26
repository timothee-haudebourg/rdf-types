use core::fmt;
use std::borrow::Cow;
use std::cmp::Ordering;

use iref::Iri;

use crate::{BlankId, RdfDisplay};

use super::{CowId, Id};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum IdRef<'a> {
	BlankId(&'a BlankId),
	Iri(&'a Iri),
}

impl<'a> IdRef<'a> {
	pub fn is_blank_id(&self) -> bool {
		matches!(self, Self::BlankId(_))
	}

	pub fn is_iri(&self) -> bool {
		matches!(self, Self::Iri(_))
	}

	pub fn as_blank_id(&self) -> Option<&'a BlankId> {
		match self {
			Self::BlankId(b) => Some(b),
			_ => None,
		}
	}

	pub fn as_iri(&self) -> Option<&'a Iri> {
		match self {
			Self::Iri(iri) => Some(iri),
			_ => None,
		}
	}

	pub fn into_cow(self) -> CowId<'a> {
		match self {
			Self::BlankId(b) => CowId::BlankId(Cow::Borrowed(b)),
			Self::Iri(iri) => CowId::Iri(Cow::Borrowed(iri)),
		}
	}
}

impl IdRef<'_> {
	pub fn to_owned(&self) -> Id {
		self.into_owned()
	}

	pub fn into_owned(self) -> Id {
		match self {
			Self::BlankId(blank_id) => Id::BlankId(blank_id.to_owned()),
			Self::Iri(iri) => Id::Iri(iri.to_owned()),
		}
	}
}

impl fmt::Display for IdRef<'_> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			Self::BlankId(b) => b.fmt(f),
			Self::Iri(i) => i.fmt(f),
		}
	}
}

impl RdfDisplay for IdRef<'_> {
	fn rdf_fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			Self::BlankId(b) => b.rdf_fmt(f),
			Self::Iri(i) => i.rdf_fmt(f),
		}
	}
}

impl PartialEq<Id> for IdRef<'_> {
	fn eq(&self, other: &Id) -> bool {
		match (self, other) {
			(Self::BlankId(a), Id::BlankId(b)) => *a == b,
			(Self::Iri(a), Id::Iri(b)) => *a == b,
			_ => false,
		}
	}
}

impl PartialOrd<Id> for IdRef<'_> {
	fn partial_cmp(&self, other: &Id) -> Option<Ordering> {
		match (self, other) {
			(Self::BlankId(a), Id::BlankId(b)) => (*a).partial_cmp(b),
			(Self::BlankId(_), Id::Iri(_)) => Some(Ordering::Less),
			(Self::Iri(_), Id::BlankId(_)) => Some(Ordering::Greater),
			(Self::Iri(a), Id::Iri(b)) => (*a).partial_cmp(b),
		}
	}
}
