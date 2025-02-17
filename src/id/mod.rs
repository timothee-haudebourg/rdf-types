use std::borrow::Cow;

use crate::BlankIdBuf;
use iref::{Iri, IriBuf};

mod r#ref;
pub use r#ref::*;

mod cow;
pub use cow::*;

pub enum Id {
	BlankId(BlankIdBuf),
	Iri(IriBuf),
}

impl Id {
	pub fn as_iri(&self) -> Option<&Iri> {
		match self {
			Self::Iri(iri) => Some(iri),
			_ => None,
		}
	}

	pub fn as_ref(&self) -> IdRef {
		match self {
			Self::BlankId(blank_id) => IdRef::BlankId(blank_id),
			Self::Iri(iri) => IdRef::Iri(iri),
		}
	}

	pub fn as_cow(&self) -> CowId {
		match self {
			Self::BlankId(blank_id) => CowId::BlankId(Cow::Borrowed(blank_id)),
			Self::Iri(iri) => CowId::Iri(Cow::Borrowed(iri)),
		}
	}
}
