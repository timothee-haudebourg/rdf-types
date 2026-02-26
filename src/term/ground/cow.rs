use std::borrow::Cow;

use iref::{Iri, IriBuf};

use crate::{CowLiteral, CowLiteralType, Literal, LiteralRef, XSD_STRING};

use super::GroundTerm;

/// Copy-on-write ground term.
///
/// A [`GroundTerm`] that is either borrowed or owned.
pub enum CowGroundTerm<'a> {
	/// IRI.
	Iri(Cow<'a, Iri>),

	/// Literal value.
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

impl<'a> From<CowLiteral<'a>> for CowGroundTerm<'a> {
	fn from(value: CowLiteral<'a>) -> Self {
		Self::Literal(value)
	}
}

impl<'a> From<&'a Literal> for CowGroundTerm<'a> {
	fn from(value: &'a Literal) -> Self {
		Self::Literal(value.into())
	}
}

impl<'a> From<LiteralRef<'a>> for CowGroundTerm<'a> {
	fn from(value: LiteralRef<'a>) -> Self {
		Self::Literal(value.into())
	}
}

impl<'a> From<&'a Iri> for CowGroundTerm<'a> {
	fn from(value: &'a Iri) -> Self {
		Self::Iri(Cow::Borrowed(value))
	}
}

impl<'a> From<&'a IriBuf> for CowGroundTerm<'a> {
	fn from(value: &'a IriBuf) -> Self {
		Self::Iri(Cow::Borrowed(value))
	}
}

impl<'a> From<&'a str> for CowGroundTerm<'a> {
	fn from(value: &'a str) -> Self {
		Self::Literal(CowLiteral::new(
			value,
			CowLiteralType::Any(Cow::Borrowed(XSD_STRING)),
		))
	}
}

impl<'a> From<&'a String> for CowGroundTerm<'a> {
	fn from(value: &'a String) -> Self {
		value.as_str().into()
	}
}
