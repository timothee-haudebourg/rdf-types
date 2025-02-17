use iref::Iri;

use crate::BlankId;

use super::Id;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum IdRef<'a> {
	BlankId(&'a BlankId),
	Iri(&'a Iri),
}

impl<'a> IdRef<'a> {
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
