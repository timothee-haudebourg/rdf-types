use core::fmt;
use std::{cmp::Ordering, fmt::Write};

use educe::Educe;
use iref::Iri;
use langtag::LangTag;

use crate::{RdfDisplay, XSD_STRING};

use super::LiteralType;

/// RDF literal type reference.
#[derive(Educe, PartialEq, Eq, Hash, PartialOrd, Ord, Debug)]
#[educe(Clone, Copy)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub enum LiteralTypeRef<'a> {
	/// Any type.
	Any(&'a Iri),

	/// Language string.
	LangString(&'a LangTag),
}

impl<'a> LiteralTypeRef<'a> {
	pub fn is_lang_string(&self) -> bool {
		matches!(self, Self::LangString(_))
	}

	pub fn lang_tag(&self) -> Option<&'a LangTag> {
		match self {
			Self::LangString(tag) => Some(tag),
			_ => None,
		}
	}

	pub fn is_xsd_string(&self) -> bool {
		self.is_iri(XSD_STRING)
	}

	pub fn is_iri(&self, iri: &Iri) -> bool {
		match self {
			Self::Any(i) => *i == iri,
			Self::LangString(_) => false,
		}
	}
}

impl<'a> LiteralTypeRef<'a> {
	pub fn into_owned(self) -> LiteralType {
		match self {
			Self::Any(i) => LiteralType::Any(i.to_owned()),
			Self::LangString(l) => LiteralType::LangString(l.to_owned()),
		}
	}
}

impl<'a> PartialEq<LiteralType> for LiteralTypeRef<'a> {
	fn eq(&self, other: &LiteralType) -> bool {
		match (*self, other) {
			(Self::Any(a), LiteralType::Any(b)) => a == b,
			(Self::LangString(a), LiteralType::LangString(b)) => a == b.as_lang_tag(),
			_ => false,
		}
	}
}

impl<'a> PartialEq<LiteralTypeRef<'a>> for LiteralType {
	fn eq(&self, other: &LiteralTypeRef<'a>) -> bool {
		match (self, other) {
			(Self::Any(a), LiteralTypeRef::Any(b)) => a == *b,
			(Self::LangString(a), LiteralTypeRef::LangString(b)) => a == *b,
			_ => false,
		}
	}
}

impl<'a> PartialOrd<LiteralType> for LiteralTypeRef<'a> {
	fn partial_cmp(&self, other: &LiteralType) -> Option<Ordering> {
		match (self, other) {
			(Self::Any(a), LiteralType::Any(b)) => (*a).partial_cmp(b),
			(Self::Any(_), LiteralType::LangString(_)) => Some(Ordering::Less),
			(Self::LangString(_), LiteralType::Any(_)) => Some(Ordering::Greater),
			(Self::LangString(a), LiteralType::LangString(b)) => (*a).partial_cmp(b),
		}
	}
}

impl<'a> PartialOrd<LiteralTypeRef<'a>> for LiteralType {
	fn partial_cmp(&self, other: &LiteralTypeRef<'a>) -> Option<Ordering> {
		match (self, other) {
			(Self::Any(a), LiteralTypeRef::Any(b)) => (*a).partial_cmp(b),
			(Self::Any(_), LiteralTypeRef::LangString(_)) => Some(Ordering::Less),
			(Self::LangString(_), LiteralTypeRef::Any(_)) => Some(Ordering::Greater),
			(Self::LangString(a), LiteralTypeRef::LangString(b)) => (*a).partial_cmp(b),
		}
	}
}

impl<'a> RdfDisplay for LiteralTypeRef<'a> {
	fn rdf_fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
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
