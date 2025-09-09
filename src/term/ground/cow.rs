use std::borrow::Cow;

use iref::Iri;

use crate::CowLiteral;

use super::GroundTerm;

pub enum CowGroundTerm<'a> {
	Iri(Cow<'a, Iri>),

	Literal(CowLiteral<'a>),
}

impl CowGroundTerm<'_> {
	pub fn into_owned(self) -> GroundTerm {
		match self {
			Self::Iri(iri) => GroundTerm::Iri(iri.into_owned()),
			Self::Literal(l) => GroundTerm::Literal(l.into_owned()),
		}
	}
}

impl From<GroundTerm> for CowGroundTerm<'_> {
	fn from(value: GroundTerm) -> Self {
		value.into_cow()
	}
}
