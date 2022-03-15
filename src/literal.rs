use iref::{Iri, IriBuf};
use langtag::{LanguageTag, LanguageTagBuf};
use std::borrow::{Borrow, BorrowMut};
use std::fmt;
use std::ops::{Deref, DerefMut};
use std::str::FromStr;

/// RDF Literal.
#[derive(Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Debug)]
pub enum Literal {
	/// Untyped string literal.
	String(StringLiteral),

	/// Typed string literal.
	TypedString(StringLiteral, IriBuf),

	/// Language string.
	LangString(StringLiteral, LanguageTagBuf),
}

impl Literal {
	pub fn is_typed(&self) -> bool {
		matches!(self, Self::TypedString(_, _))
	}

	pub fn ty(&self) -> Option<Iri> {
		match self {
			Self::TypedString(_, ty) => Some(ty.as_iri()),
			_ => None,
		}
	}

	pub fn is_lang_string(&self) -> bool {
		matches!(self, Self::LangString(_, _))
	}

	pub fn lang_tag(&self) -> Option<LanguageTag> {
		match self {
			Self::LangString(_, tag) => Some(tag.as_ref()),
			_ => None,
		}
	}

	pub fn string_literal(&self) -> &StringLiteral {
		match self {
			Self::String(s) => s,
			Self::TypedString(s, _) => s,
			Self::LangString(s, _) => s,
		}
	}

	pub fn into_string_literal(self) -> StringLiteral {
		match self {
			Self::String(s) => s,
			Self::TypedString(s, _) => s,
			Self::LangString(s, _) => s,
		}
	}
}

/// String literal, without type or language tag.
#[derive(Default, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct StringLiteral(String);

impl StringLiteral {
	pub fn new() -> Self {
		Self::default()
	}
}

impl PartialEq<String> for StringLiteral {
	fn eq(&self, other: &String) -> bool {
		self.as_str().eq(other.as_str())
	}
}

impl PartialEq<str> for StringLiteral {
	fn eq(&self, other: &str) -> bool {
		self.as_str().eq(other)
	}
}

impl PartialEq<StringLiteral> for String {
	fn eq(&self, other: &StringLiteral) -> bool {
		self.as_str().eq(other.as_str())
	}
}

impl PartialEq<StringLiteral> for str {
	fn eq(&self, other: &StringLiteral) -> bool {
		self.eq(other.as_str())
	}
}

impl From<String> for StringLiteral {
	fn from(s: String) -> Self {
		Self(s)
	}
}

impl From<StringLiteral> for String {
	fn from(s: StringLiteral) -> Self {
		s.0
	}
}

impl FromStr for StringLiteral {
	type Err = std::convert::Infallible;

	fn from_str(s: &str) -> Result<Self, std::convert::Infallible> {
		Ok(Self(s.to_owned()))
	}
}

impl Deref for StringLiteral {
	type Target = String;

	fn deref(&self) -> &String {
		&self.0
	}
}

impl DerefMut for StringLiteral {
	fn deref_mut(&mut self) -> &mut String {
		&mut self.0
	}
}

impl Borrow<str> for StringLiteral {
	fn borrow(&self) -> &str {
		self.0.as_str()
	}
}

impl BorrowMut<str> for StringLiteral {
	fn borrow_mut(&mut self) -> &mut str {
		self.0.as_mut_str()
	}
}

impl AsRef<str> for StringLiteral {
	fn as_ref(&self) -> &str {
		self.0.as_str()
	}
}

impl AsMut<str> for StringLiteral {
	fn as_mut(&mut self) -> &mut str {
		self.0.as_mut_str()
	}
}

impl fmt::Display for StringLiteral {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "\"")?;

		for c in self.0.chars() {
			match c {
				'"' => write!(f, "\\u0022"),
				'\\' => write!(f, "\\u005c"),
				'\n' => write!(f, "\\n"),
				'\r' => write!(f, "\\r"),
				'\t' => write!(f, "\\t"),
				'\u{08}' => write!(f, "\\b"),
				'\u{0c}' => write!(f, "\\f"),
				c => c.fmt(f),
			}?
		}

		write!(f, "\"")
	}
}

impl fmt::Display for Literal {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			Self::String(s) => s.fmt(f),
			Self::TypedString(s, ty) => write!(f, "{}^^<{}>", s, ty),
			Self::LangString(s, tag) => write!(f, "{}@{}", s, tag),
		}
	}
}
