use core::fmt;
use std::{borrow::Borrow, cmp::Ordering};

use educe::Educe;
use langtag::LangTag;

use crate::RdfDisplay;

use super::{CowLiteral, Literal, LiteralTypeRef};

/// RDF Literal reference.
#[derive(Educe, PartialEq, Eq, Hash, PartialOrd, Ord, Debug)]
#[educe(Clone, Copy)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct LiteralRef<'a> {
	/// Literal value.
	pub value: &'a str,

	/// Literal type.
	pub type_: LiteralTypeRef<'a>,
}

impl<'a> LiteralRef<'a> {
	pub fn new(value: &'a str, type_: LiteralTypeRef<'a>) -> Self {
		Self { value, type_ }
	}

	pub fn as_type(&self) -> LiteralTypeRef<'a> {
		self.type_
	}

	pub fn as_type_mut(&mut self) -> &mut LiteralTypeRef<'a> {
		&mut self.type_
	}

	pub fn into_type(self) -> LiteralTypeRef<'a> {
		self.type_
	}

	pub fn as_value(&self) -> &'a str {
		self.value
	}

	pub fn into_value(self) -> &'a str {
		self.value
	}

	pub fn into_parts(self) -> (&'a str, LiteralTypeRef<'a>) {
		(self.value, self.type_)
	}

	pub fn as_str(&self) -> &'a str {
		self.value
	}

	pub fn as_bytes(&self) -> &'a [u8] {
		self.value.as_ref()
	}

	pub fn is_lang_string(&self) -> bool {
		self.type_.is_lang_string()
	}

	pub fn lang_tag(&self) -> Option<&'a LangTag> {
		self.type_.lang_tag()
	}

	pub fn into_cow(self) -> CowLiteral<'a> {
		CowLiteral::new(self.value, self.type_)
	}
}

impl<'a> LiteralRef<'a> {
	pub fn to_owned(self) -> Literal {
		Literal::new(self.value.to_owned(), self.type_.into_owned())
	}

	pub fn cloned(self) -> Literal {
		self.to_owned()
	}
}

impl<'a> PartialEq<LiteralRef<'a>> for Literal {
	fn eq(&self, other: &LiteralRef<'a>) -> bool {
		self.type_ == other.type_ && self.value == other.value
	}
}

impl<'a> PartialEq<Literal> for LiteralRef<'a> {
	fn eq(&self, other: &Literal) -> bool {
		self.type_ == other.type_ && self.value == other.value
	}
}

impl<'a> equivalent::Equivalent<Literal> for LiteralRef<'a> {
	fn equivalent(&self, key: &Literal) -> bool {
		self == key
	}
}

impl<'a> PartialOrd<LiteralRef<'a>> for Literal {
	fn partial_cmp(&self, other: &LiteralRef<'a>) -> Option<Ordering> {
		Some(
			self.value
				.as_str()
				.partial_cmp(other.value)?
				.then(self.type_.partial_cmp(&other.type_)?),
		)
	}
}

impl<'a> PartialOrd<Literal> for LiteralRef<'a> {
	fn partial_cmp(&self, other: &Literal) -> Option<Ordering> {
		Some(
			self.value
				.partial_cmp(&other.value)?
				.then(self.type_.partial_cmp(&other.type_)?),
		)
	}
}

impl<'a> Borrow<str> for LiteralRef<'a> {
	fn borrow(&self) -> &str {
		self.as_str()
	}
}

impl<'a> AsRef<str> for LiteralRef<'a> {
	fn as_ref(&self) -> &str {
		self.as_str()
	}
}

impl<'a> fmt::Display for LiteralRef<'a> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		self.value.rdf_fmt(f)?;
		if self.type_.is_xsd_string() {
			Ok(())
		} else {
			self.type_.rdf_fmt(f)
		}
	}
}

impl<'a> RdfDisplay for LiteralRef<'a> {
	fn rdf_fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		self.value.rdf_fmt(f)?;
		if self.type_.is_xsd_string() {
			Ok(())
		} else {
			self.type_.rdf_fmt(f)
		}
	}
}
