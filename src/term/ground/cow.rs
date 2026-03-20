use core::fmt;
use std::borrow::Cow;
use std::cmp::Ordering;

use iref::{Iri, IriBuf};

use crate::{CowLiteral, CowLiteralType, Literal, LiteralRef, RdfDisplay, XSD_STRING};

use super::{GroundTerm, GroundTermRef};

/// Copy-on-write ground term.
///
/// A [`GroundTerm`] that is either borrowed or owned.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum CowGroundTerm<'a> {
	/// IRI.
	Iri(Cow<'a, Iri>),

	/// Literal value.
	Literal(CowLiteral<'a>),
}

impl CowGroundTerm<'_> {
	pub fn is_iri(&self) -> bool {
		matches!(self, Self::Iri(_))
	}

	pub fn is_literal(&self) -> bool {
		matches!(self, Self::Literal(_))
	}

	pub fn as_iri(&self) -> Option<&Iri> {
		match self {
			Self::Iri(iri) => Some(iri),
			_ => None,
		}
	}

	pub fn as_literal(&self) -> Option<LiteralRef<'_>> {
		match self {
			Self::Literal(l) => Some(l.as_ref()),
			_ => None,
		}
	}

	pub fn as_ref(&self) -> GroundTermRef<'_> {
		match self {
			Self::Iri(iri) => GroundTermRef::Iri(iri),
			Self::Literal(l) => GroundTermRef::Literal(l.as_ref()),
		}
	}

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

impl<'a> From<&'a GroundTerm> for CowGroundTerm<'a> {
	fn from(value: &'a GroundTerm) -> Self {
		value.as_cow()
	}
}

impl<'a> From<GroundTermRef<'a>> for CowGroundTerm<'a> {
	fn from(value: GroundTermRef<'a>) -> Self {
		value.into_cow()
	}
}

impl From<CowGroundTerm<'_>> for GroundTerm {
	fn from(value: CowGroundTerm<'_>) -> Self {
		value.into_owned()
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

impl PartialEq<GroundTerm> for CowGroundTerm<'_> {
	fn eq(&self, other: &GroundTerm) -> bool {
		self.as_ref() == *other
	}
}

impl PartialEq<CowGroundTerm<'_>> for GroundTerm {
	fn eq(&self, other: &CowGroundTerm<'_>) -> bool {
		*self == other.as_ref()
	}
}

impl<'a> PartialEq<GroundTermRef<'a>> for CowGroundTerm<'_> {
	fn eq(&self, other: &GroundTermRef<'a>) -> bool {
		self.as_ref() == *other
	}
}

impl PartialEq<CowGroundTerm<'_>> for GroundTermRef<'_> {
	fn eq(&self, other: &CowGroundTerm<'_>) -> bool {
		*self == other.as_ref()
	}
}

impl PartialOrd<GroundTerm> for CowGroundTerm<'_> {
	fn partial_cmp(&self, other: &GroundTerm) -> Option<Ordering> {
		self.as_ref().partial_cmp(other)
	}
}

impl PartialOrd<CowGroundTerm<'_>> for GroundTerm {
	fn partial_cmp(&self, other: &CowGroundTerm<'_>) -> Option<Ordering> {
		self.partial_cmp(&other.as_ref())
	}
}

impl<'a> PartialOrd<GroundTermRef<'a>> for CowGroundTerm<'_> {
	fn partial_cmp(&self, other: &GroundTermRef<'a>) -> Option<Ordering> {
		self.as_ref().partial_cmp(other)
	}
}

impl PartialOrd<CowGroundTerm<'_>> for GroundTermRef<'_> {
	fn partial_cmp(&self, other: &CowGroundTerm<'_>) -> Option<Ordering> {
		self.partial_cmp(&other.as_ref())
	}
}

impl From<GroundTermRef<'_>> for GroundTerm {
	fn from(value: GroundTermRef<'_>) -> Self {
		value.to_owned()
	}
}

impl fmt::Display for CowGroundTerm<'_> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			Self::Iri(iri) => iri.fmt(f),
			Self::Literal(lit) => lit.fmt(f),
		}
	}
}

impl RdfDisplay for CowGroundTerm<'_> {
	fn rdf_fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			Self::Iri(iri) => iri.rdf_fmt(f),
			Self::Literal(lit) => lit.rdf_fmt(f),
		}
	}
}
