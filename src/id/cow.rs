use std::borrow::Cow;

use iref::Iri;

use crate::{BlankId, Id, IdRef};

pub enum CowId<'a> {
	BlankId(Cow<'a, BlankId>),
	Iri(Cow<'a, Iri>),
}

impl CowId<'_> {
	pub fn is_blank_id(&self) -> bool {
		matches!(self, Self::BlankId(_))
	}

	pub fn is_iri(&self) -> bool {
		matches!(self, Self::Iri(_))
	}

	pub fn as_blank_id(&self) -> Option<&BlankId> {
		match self {
			Self::BlankId(b) => Some(b),
			_ => None,
		}
	}

	pub fn as_iri(&self) -> Option<&Iri> {
		match self {
			Self::Iri(iri) => Some(iri),
			_ => None,
		}
	}

	pub fn as_ref(&self) -> IdRef<'_> {
		match self {
			Self::BlankId(b) => IdRef::BlankId(b),
			Self::Iri(iri) => IdRef::Iri(iri),
		}
	}

	pub fn into_owned(self) -> Id {
		match self {
			Self::BlankId(b) => Id::BlankId(b.into_owned()),
			Self::Iri(iri) => Id::Iri(iri.into_owned()),
		}
	}
}

impl From<Id> for CowId<'_> {
	fn from(value: Id) -> Self {
		value.into_cow()
	}
}

impl<'a> From<&'a Id> for CowId<'a> {
	fn from(value: &'a Id) -> Self {
		value.as_cow()
	}
}

impl<'a> From<IdRef<'a>> for CowId<'a> {
	fn from(value: IdRef<'a>) -> Self {
		value.into_cow()
	}
}

impl From<CowId<'_>> for Id {
	fn from(value: CowId<'_>) -> Self {
		value.into_owned()
	}
}
