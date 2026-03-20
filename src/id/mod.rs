use core::fmt;
use std::borrow::Cow;
use std::cmp::Ordering;

use crate::{BlankId, BlankIdBuf, RdfDisplay};
use iref::{Iri, IriBuf};

mod r#ref;
pub use r#ref::*;

mod cow;
pub use cow::*;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum Id {
	BlankId(BlankIdBuf),
	Iri(IriBuf),
}

impl Id {
	pub fn is_blank_id(&self) -> bool {
		matches!(self, Self::BlankId(_))
	}

	pub fn as_blank_id(&self) -> Option<&BlankId> {
		match self {
			Self::BlankId(b) => Some(b),
			_ => None,
		}
	}

	pub fn is_iri(&self) -> bool {
		matches!(self, Self::Iri(_))
	}

	pub fn as_iri(&self) -> Option<&Iri> {
		match self {
			Self::Iri(iri) => Some(iri),
			_ => None,
		}
	}

	pub fn as_ref(&self) -> IdRef<'_> {
		match self {
			Self::BlankId(blank_id) => IdRef::BlankId(blank_id),
			Self::Iri(iri) => IdRef::Iri(iri),
		}
	}

	pub fn into_blank_id(self) -> Option<BlankIdBuf> {
		match self {
			Self::BlankId(b) => Some(b),
			_ => None,
		}
	}

	pub fn into_iri(self) -> Option<IriBuf> {
		match self {
			Self::Iri(iri) => Some(iri),
			_ => None,
		}
	}

	pub fn as_cow(&self) -> CowId<'_> {
		match self {
			Self::BlankId(blank_id) => CowId::BlankId(Cow::Borrowed(blank_id)),
			Self::Iri(iri) => CowId::Iri(Cow::Borrowed(iri)),
		}
	}

	pub fn into_cow(self) -> CowId<'static> {
		match self {
			Self::BlankId(blank_id) => CowId::BlankId(Cow::Owned(blank_id)),
			Self::Iri(iri) => CowId::Iri(Cow::Owned(iri)),
		}
	}
}

impl fmt::Display for Id {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Self::BlankId(b) => b.fmt(f),
			Self::Iri(i) => i.fmt(f),
		}
	}
}

impl RdfDisplay for Id {
	fn rdf_fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			Self::BlankId(b) => b.rdf_fmt(f),
			Self::Iri(i) => i.rdf_fmt(f),
		}
	}
}

impl PartialEq<Iri> for Id {
	fn eq(&self, other: &Iri) -> bool {
		match self {
			Self::Iri(iri) => iri.as_iri() == other,
			_ => false,
		}
	}
}

impl PartialEq<&Iri> for Id {
	fn eq(&self, other: &&Iri) -> bool {
		self.eq(*other)
	}
}

impl PartialEq<IriBuf> for Id {
	fn eq(&self, other: &IriBuf) -> bool {
		match self {
			Self::Iri(iri) => iri == other,
			_ => false,
		}
	}
}

impl PartialEq<&IriBuf> for Id {
	fn eq(&self, other: &&IriBuf) -> bool {
		self.eq(*other)
	}
}

impl PartialEq<BlankId> for Id {
	fn eq(&self, other: &BlankId) -> bool {
		match self {
			Self::BlankId(b) => b.as_blank_id() == other,
			_ => false,
		}
	}
}

impl PartialEq<BlankIdBuf> for Id {
	fn eq(&self, other: &BlankIdBuf) -> bool {
		match self {
			Self::BlankId(b) => b == other,
			_ => false,
		}
	}
}

impl PartialEq<&BlankId> for Id {
	fn eq(&self, other: &&BlankId) -> bool {
		self.eq(*other)
	}
}

impl PartialEq<&BlankIdBuf> for Id {
	fn eq(&self, other: &&BlankIdBuf) -> bool {
		self.eq(*other)
	}
}

impl<'a> PartialEq<IdRef<'a>> for Id {
	fn eq(&self, other: &IdRef<'a>) -> bool {
		match (self, other) {
			(Self::BlankId(a), IdRef::BlankId(b)) => a == *b,
			(Self::Iri(a), IdRef::Iri(b)) => a == *b,
			_ => false,
		}
	}
}

impl<'a> PartialOrd<IdRef<'a>> for Id {
	fn partial_cmp(&self, other: &IdRef<'a>) -> Option<Ordering> {
		match (self, other) {
			(Self::BlankId(a), IdRef::BlankId(b)) => a.as_blank_id().partial_cmp(b),
			(Self::BlankId(_), IdRef::Iri(_)) => Some(Ordering::Less),
			(Self::Iri(_), IdRef::BlankId(_)) => Some(Ordering::Greater),
			(Self::Iri(a), IdRef::Iri(b)) => a.as_iri().partial_cmp(b),
		}
	}
}
