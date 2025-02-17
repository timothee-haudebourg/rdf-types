use std::borrow::Cow;

use educe::Educe;
use iref::{Iri, IriBuf};
use langtag::{LangTag, LangTagBuf};

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
	pub fn as_ref(&self) -> LiteralTypeRef {
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
