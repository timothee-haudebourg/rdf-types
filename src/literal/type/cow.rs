use core::fmt;
use std::{borrow::Cow, fmt::Write};

use educe::Educe;
use iref::{Iri, IriBuf};
use langtag::{LangTag, LangTagBuf};

use crate::{RdfDisplay, RDF_LANG_STRING};

use super::{LiteralType, LiteralTypeRef};

/// Owned or referenced RDF literal type.
#[derive(Educe, PartialEq, Eq, Hash, PartialOrd, Ord, Debug)]
#[educe(Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub enum CowLiteralType<'a> {
	/// Any type.
	Any(Cow<'a, Iri>),

	/// Language string.
	LangString(Cow<'a, LangTag>),
}

impl CowLiteralType<'_> {
	pub fn is_any(&self) -> bool {
		matches!(self, Self::Any(_))
	}

	pub fn is_lang_string(&self) -> bool {
		matches!(self, Self::LangString(_))
	}

	pub fn as_any(&self) -> Option<&Iri> {
		match self {
			Self::Any(iri) => Some(iri),
			_ => None,
		}
	}

	pub fn as_lang_string(&self) -> Option<&LangTag> {
		match self {
			Self::LangString(tag) => Some(tag),
			_ => None,
		}
	}

	pub fn lang_tag(&self) -> Option<&LangTag> {
		self.as_lang_string()
	}

	pub fn is_xsd_string(&self) -> bool {
		self.is_iri(crate::XSD_STRING)
	}

	pub fn is_iri(&self, iri: &Iri) -> bool {
		match self {
			Self::Any(i) => i.as_ref() == iri,
			Self::LangString(_) => false,
		}
	}

	pub fn as_ref(&self) -> LiteralTypeRef<'_> {
		match self {
			Self::Any(i) => LiteralTypeRef::Any(i),
			Self::LangString(l) => LiteralTypeRef::LangString(l),
		}
	}

	pub fn into_owned(self) -> LiteralType {
		match self {
			Self::Any(iri) => LiteralType::Any(iri.into_owned()),
			Self::LangString(lang) => LiteralType::LangString(lang.into_owned()),
		}
	}
}

impl<'a> From<CowLiteralType<'a>> for LiteralType {
	fn from(value: CowLiteralType<'a>) -> Self {
		value.into_owned()
	}
}

impl<'a> From<LiteralTypeRef<'a>> for CowLiteralType<'a> {
	fn from(value: LiteralTypeRef<'a>) -> Self {
		match value {
			LiteralTypeRef::Any(i) => Self::Any(Cow::Borrowed(i)),
			LiteralTypeRef::LangString(t) => Self::LangString(Cow::Borrowed(t)),
		}
	}
}

impl From<LiteralType> for CowLiteralType<'_> {
	fn from(value: LiteralType) -> Self {
		value.into_cow()
	}
}

impl<'a> From<&'a LiteralType> for CowLiteralType<'a> {
	fn from(value: &'a LiteralType) -> Self {
		value.as_cow()
	}
}

impl<'a> From<&'a Iri> for CowLiteralType<'a> {
	fn from(value: &'a Iri) -> Self {
		Self::Any(Cow::Borrowed(value))
	}
}

impl From<IriBuf> for CowLiteralType<'_> {
	fn from(value: IriBuf) -> Self {
		Self::Any(Cow::Owned(value))
	}
}

impl<'a> From<Cow<'a, Iri>> for CowLiteralType<'a> {
	fn from(value: Cow<'a, Iri>) -> Self {
		Self::Any(value)
	}
}

impl<'a> From<&'a LangTag> for CowLiteralType<'a> {
	fn from(value: &'a LangTag) -> Self {
		Self::LangString(Cow::Borrowed(value))
	}
}

impl From<LangTagBuf> for CowLiteralType<'_> {
	fn from(value: LangTagBuf) -> Self {
		Self::LangString(Cow::Owned(value))
	}
}

impl<'a> From<Cow<'a, LangTag>> for CowLiteralType<'a> {
	fn from(value: Cow<'a, LangTag>) -> Self {
		Self::LangString(value)
	}
}

impl PartialEq<Iri> for CowLiteralType<'_> {
	fn eq(&self, other: &Iri) -> bool {
		match self {
			Self::Any(ty) => ty.as_ref() == other,
			Self::LangString(_) => RDF_LANG_STRING == other,
		}
	}
}

impl<'a> PartialEq<&'a Iri> for CowLiteralType<'_> {
	fn eq(&self, other: &&'a Iri) -> bool {
		match self {
			Self::Any(ty) => ty.as_ref() == other,
			Self::LangString(_) => RDF_LANG_STRING == other,
		}
	}
}

impl fmt::Display for CowLiteralType<'_> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			Self::Any(ty) => {
				f.write_str("^^")?;
				ty.rdf_fmt(f)
			}
			Self::LangString(tag) => {
				f.write_char('@')?;
				tag.rdf_fmt(f)
			}
		}
	}
}

impl RdfDisplay for CowLiteralType<'_> {
	fn rdf_fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		fmt::Display::fmt(self, f)
	}
}
