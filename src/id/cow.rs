use core::fmt;
use std::borrow::Cow;

use iref::{Iri, IriBuf};

use crate::{BlankId, BlankIdBuf, Id, IdRef, RdfDisplay};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
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

impl PartialEq<Iri> for CowId<'_> {
	fn eq(&self, other: &Iri) -> bool {
		match self {
			Self::Iri(iri) => iri.as_ref() == other,
			_ => false,
		}
	}
}

impl PartialEq<&Iri> for CowId<'_> {
	fn eq(&self, other: &&Iri) -> bool {
		self.eq(*other)
	}
}

impl PartialEq<IriBuf> for CowId<'_> {
	fn eq(&self, other: &IriBuf) -> bool {
		match self {
			Self::Iri(iri) => iri.as_ref() == other.as_iri(),
			_ => false,
		}
	}
}

impl PartialEq<&IriBuf> for CowId<'_> {
	fn eq(&self, other: &&IriBuf) -> bool {
		self.eq(*other)
	}
}

impl PartialEq<BlankId> for CowId<'_> {
	fn eq(&self, other: &BlankId) -> bool {
		match self {
			Self::BlankId(b) => b.as_ref() == other,
			_ => false,
		}
	}
}

impl PartialEq<BlankIdBuf> for CowId<'_> {
	fn eq(&self, other: &BlankIdBuf) -> bool {
		match self {
			Self::BlankId(b) => b.as_ref() == other.as_blank_id(),
			_ => false,
		}
	}
}

impl PartialEq<&BlankId> for CowId<'_> {
	fn eq(&self, other: &&BlankId) -> bool {
		self.eq(*other)
	}
}

impl PartialEq<&BlankIdBuf> for CowId<'_> {
	fn eq(&self, other: &&BlankIdBuf) -> bool {
		self.eq(*other)
	}
}

impl fmt::Display for CowId<'_> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			Self::BlankId(b) => b.fmt(f),
			Self::Iri(i) => i.fmt(f),
		}
	}
}

impl RdfDisplay for CowId<'_> {
	fn rdf_fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			Self::BlankId(b) => b.rdf_fmt(f),
			Self::Iri(i) => i.rdf_fmt(f),
		}
	}
}
