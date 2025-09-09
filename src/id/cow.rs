use std::borrow::Cow;

use iref::Iri;

use crate::{BlankId, Id};

pub enum CowId<'a> {
	BlankId(Cow<'a, BlankId>),
	Iri(Cow<'a, Iri>),
}

impl From<Id> for CowId<'_> {
	fn from(value: Id) -> Self {
		match value {
			Id::BlankId(b) => Self::BlankId(Cow::Owned(b)),
			Id::Iri(i) => Self::Iri(Cow::Owned(i)),
		}
	}
}

impl<'a> From<&'a Id> for CowId<'a> {
	fn from(value: &'a Id) -> Self {
		match value {
			Id::BlankId(b) => Self::BlankId(Cow::Borrowed(b)),
			Id::Iri(i) => Self::Iri(Cow::Borrowed(i)),
		}
	}
}
