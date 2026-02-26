use core::fmt;
use std::borrow::Cow;

use crate::{BlankId, BlankIdBuf};
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

	pub fn as_cow(&self) -> CowId<'_> {
		match self {
			Self::BlankId(blank_id) => CowId::BlankId(Cow::Borrowed(blank_id)),
			Self::Iri(iri) => CowId::Iri(Cow::Borrowed(iri)),
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
