use std::borrow::Cow;

use educe::Educe;

use super::{CowLiteralType, Literal, LiteralRef};

/// RDF Literal reference.
#[derive(Educe, PartialEq, Eq, Hash, PartialOrd, Ord, Debug)]
#[educe(Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct CowLiteral<'a> {
	/// Literal value.
	pub value: Cow<'a, str>,

	/// Literal type.
	pub type_: CowLiteralType<'a>,
}

impl<'a> CowLiteral<'a> {
	pub fn new(value: impl Into<Cow<'a, str>>, type_: impl Into<CowLiteralType<'a>>) -> Self {
		Self {
			value: value.into(),
			type_: type_.into(),
		}
	}

	pub fn as_ref(&self) -> LiteralRef<'_> {
		LiteralRef::new(&self.value, self.type_.as_ref())
	}

	pub fn into_owned(self) -> Literal {
		Literal::new(self.value, self.type_)
	}
}

impl<'a> From<&'a Literal> for CowLiteral<'a> {
	fn from(value: &'a Literal) -> Self {
		Self::new(&value.value, value.type_.as_ref())
	}
}

impl<'a> From<LiteralRef<'a>> for CowLiteral<'a> {
	fn from(value: LiteralRef<'a>) -> Self {
		Self::new(value.value, value.type_)
	}
}

impl From<Literal> for CowLiteral<'_> {
	fn from(value: Literal) -> Self {
		value.into_cow()
	}
}

impl From<CowLiteral<'_>> for Literal {
	fn from(value: CowLiteral<'_>) -> Self {
		value.into_owned()
	}
}
