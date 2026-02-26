use core::fmt;
use std::borrow::Cow;

use educe::Educe;
use langtag::LangTag;

use crate::RdfDisplay;

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

	pub fn as_type(&self) -> &CowLiteralType<'a> {
		&self.type_
	}

	pub fn as_value(&self) -> &str {
		&self.value
	}

	pub fn into_parts(self) -> (Cow<'a, str>, CowLiteralType<'a>) {
		(self.value, self.type_)
	}

	pub fn as_str(&self) -> &str {
		&self.value
	}

	pub fn as_bytes(&self) -> &[u8] {
		self.value.as_bytes()
	}

	pub fn is_lang_string(&self) -> bool {
		self.type_.is_lang_string()
	}

	pub fn lang_tag(&self) -> Option<&LangTag> {
		self.type_.lang_tag()
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

impl fmt::Display for CowLiteral<'_> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		self.value.rdf_fmt(f)?;
		if self.type_.is_xsd_string() {
			Ok(())
		} else {
			self.type_.rdf_fmt(f)
		}
	}
}

impl RdfDisplay for CowLiteral<'_> {
	fn rdf_fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		self.value.rdf_fmt(f)?;
		if self.type_.is_xsd_string() {
			Ok(())
		} else {
			self.type_.rdf_fmt(f)
		}
	}
}
