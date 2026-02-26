use std::borrow::Cow;

use iref::Iri;

use crate::BlankId;

use super::{CowId, Id};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
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
