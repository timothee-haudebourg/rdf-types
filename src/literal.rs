use crate::{IriVocabularyMut, RdfDisplay};
use iref::IriBuf;
use langtag::LanguageTagBuf;
use std::borrow::{Borrow, BorrowMut};
use std::fmt;
use std::ops::{Deref, DerefMut};
use std::str::FromStr;

#[cfg(feature = "contextual")]
use contextual::DisplayWithContext;

/// RDF Literal.
#[derive(Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Debug)]
pub enum Literal<S = StringLiteral, I = IriBuf, L = LanguageTagBuf> {
	/// Untyped string literal.
	String(S),

	/// Typed string literal.
	TypedString(S, I),

	/// Language string.
	LangString(S, L),
}

impl<S, I, L> Literal<S, I, L> {
	pub fn is_typed(&self) -> bool {
		matches!(self, Self::TypedString(_, _))
	}

	pub fn ty(&self) -> Option<&I> {
		match self {
			Self::TypedString(_, ty) => Some(ty),
			_ => None,
		}
	}

	pub fn is_lang_string(&self) -> bool {
		matches!(self, Self::LangString(_, _))
	}

	pub fn lang_tag(&self) -> Option<&L> {
		match self {
			Self::LangString(_, tag) => Some(tag),
			_ => None,
		}
	}

	pub fn string_literal(&self) -> &S {
		match self {
			Self::String(s) => s,
			Self::TypedString(s, _) => s,
			Self::LangString(s, _) => s,
		}
	}

	pub fn string_literal_mut(&mut self) -> &mut S {
		match self {
			Self::String(s) => s,
			Self::TypedString(s, _) => s,
			Self::LangString(s, _) => s,
		}
	}

	pub fn into_string_literal(self) -> S {
		match self {
			Self::String(s) => s,
			Self::TypedString(s, _) => s,
			Self::LangString(s, _) => s,
		}
	}
}

impl<S, L> Literal<S, IriBuf, L> {
	pub fn inserted_into<V: IriVocabularyMut>(&self, vocabulary: &mut V) -> Literal<S, V::Iri, L>
	where
		S: Clone,
		L: Clone,
	{
		match self {
			Self::String(s) => Literal::String(s.clone()),
			Self::TypedString(s, t) => {
				Literal::TypedString(s.clone(), vocabulary.insert(t.as_iri()))
			}
			Self::LangString(s, l) => Literal::LangString(s.clone(), l.clone()),
		}
	}

	pub fn insert_into<V: IriVocabularyMut>(self, vocabulary: &mut V) -> Literal<S, V::Iri, L> {
		match self {
			Self::String(s) => Literal::String(s),
			Self::TypedString(s, t) => Literal::TypedString(s, vocabulary.insert(t.as_iri())),
			Self::LangString(s, l) => Literal::LangString(s, l),
		}
	}
}

impl<S: Borrow<str>, I, L> Borrow<str> for Literal<S, I, L> {
	fn borrow(&self) -> &str {
		self.string_literal().borrow()
	}
}

impl<S: BorrowMut<str>, I, L> BorrowMut<str> for Literal<S, I, L> {
	fn borrow_mut(&mut self) -> &mut str {
		self.string_literal_mut().borrow_mut()
	}
}

impl<S: AsRef<str>, I, L> AsRef<str> for Literal<S, I, L> {
	fn as_ref(&self) -> &str {
		self.string_literal().as_ref()
	}
}

impl<S: AsMut<str>, I, L> AsMut<str> for Literal<S, I, L> {
	fn as_mut(&mut self) -> &mut str {
		self.string_literal_mut().as_mut()
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
				'"' => write!(f, "\\\""),
				'\\' => write!(f, "\\\\"),
				'\n' => write!(f, "\\n"),
				'\r' => write!(f, "\\r"),
				// '\t' => write!(f, "\\t"),
				// '\u{08}' => write!(f, "\\b"),
				// '\u{0c}' => write!(f, "\\f"),
				c => c.fmt(f),
			}?
		}

		write!(f, "\"")
	}
}

impl<S: fmt::Display, I: RdfDisplay, L: fmt::Display> fmt::Display for Literal<S, I, L> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			Self::String(s) => s.fmt(f),
			Self::TypedString(s, ty) => write!(f, "{s}^^{}", ty.rdf_display()),
			Self::LangString(s, tag) => write!(f, "{s}@{tag}"),
		}
	}
}

impl<S: fmt::Display, I: RdfDisplay, L: fmt::Display> RdfDisplay for Literal<S, I, L> {
	fn rdf_fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			Self::String(s) => s.fmt(f),
			Self::TypedString(s, ty) => write!(f, "{s}^^{}", ty.rdf_display()),
			Self::LangString(s, tag) => write!(f, "{s}@{tag}"),
		}
	}
}

#[cfg(feature = "contextual")]
impl<S: fmt::Display, I, L: fmt::Display, V: crate::IriVocabulary<Iri = I>> DisplayWithContext<V>
	for Literal<S, I, L>
{
	fn fmt_with(&self, vocabulary: &V, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			Self::String(s) => s.fmt(f),
			Self::TypedString(s, ty) => write!(f, "{s}^^<{}>", vocabulary.iri(ty).unwrap()),
			Self::LangString(s, tag) => write!(f, "{s}@{tag}"),
		}
	}
}

#[cfg(feature = "contextual")]
impl<S: fmt::Display, I, L: fmt::Display, V: crate::IriVocabulary<Iri = I>>
	crate::RdfDisplayWithContext<V> for Literal<S, I, L>
{
	fn rdf_fmt_with(&self, vocabulary: &V, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			Self::String(s) => s.fmt(f),
			Self::TypedString(s, ty) => write!(f, "{s}^^<{}>", vocabulary.iri(ty).unwrap()),
			Self::LangString(s, tag) => write!(f, "{s}@{tag}"),
		}
	}
}
