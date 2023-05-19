use crate::{IriVocabulary, IriVocabularyMut, RdfDisplay};
use iref::IriBuf;
use langtag::LanguageTagBuf;
use std::borrow::{Borrow, BorrowMut};
use std::fmt;

#[cfg(feature = "contextual")]
use contextual::DisplayWithContext;

/// RDF Literal.
#[derive(Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Literal<S = String, I = IriBuf, L = LanguageTagBuf> {
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

impl<S: RdfDisplay, I: RdfDisplay, L: fmt::Display> fmt::Display for Literal<S, I, L> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			Self::String(s) => s.rdf_fmt(f),
			Self::TypedString(s, ty) => write!(f, "{}^^{}", s.rdf_display(), ty.rdf_display()),
			Self::LangString(s, tag) => write!(f, "{}@{tag}", s.rdf_display()),
		}
	}
}

impl<S: RdfDisplay, I: RdfDisplay, L: fmt::Display> RdfDisplay for Literal<S, I, L> {
	fn rdf_fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			Self::String(s) => s.rdf_fmt(f),
			Self::TypedString(s, ty) => write!(f, "{}^^{}", s.rdf_display(), ty.rdf_display()),
			Self::LangString(s, tag) => write!(f, "{}@{tag}", s.rdf_display()),
		}
	}
}

#[cfg(feature = "contextual")]
impl<S: RdfDisplay, I, L: fmt::Display, V: crate::IriVocabulary<Iri = I>> DisplayWithContext<V>
	for Literal<S, I, L>
{
	fn fmt_with(&self, vocabulary: &V, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			Self::String(s) => s.rdf_fmt(f),
			Self::TypedString(s, ty) => {
				write!(f, "{}^^<{}>", s.rdf_display(), vocabulary.iri(ty).unwrap())
			}
			Self::LangString(s, tag) => write!(f, "{}@{tag}", s.rdf_display()),
		}
	}
}

#[cfg(feature = "contextual")]
impl<S: RdfDisplay, I, L: fmt::Display, V: crate::IriVocabulary<Iri = I>>
	crate::RdfDisplayWithContext<V> for Literal<S, I, L>
{
	fn rdf_fmt_with(&self, vocabulary: &V, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			Self::String(s) => s.rdf_fmt(f),
			Self::TypedString(s, ty) => {
				write!(f, "{}^^<{}>", s.rdf_display(), vocabulary.iri(ty).unwrap())
			}
			Self::LangString(s, tag) => write!(f, "{}@{tag}", s.rdf_display()),
		}
	}
}

/// Type that can be converted into a [`Literal`].
pub trait IntoLiteral {
	/// String data type.
	type String;

	/// Literal type value type.
	type Type;

	/// Language tag type.
	type LanguageTag;

	/// Turns the value into a `Literal`.
	fn into_literal(self) -> Literal<Self::String, Self::Type, Self::LanguageTag>;
}

impl<S, T, L> IntoLiteral for Literal<S, T, L> {
	type String = S;

	type Type = T;

	type LanguageTag = L;

	fn into_literal(self) -> Self {
		self
	}
}

/// Type that can turn a `Literal<S, T, L>` into a `Literal`.
pub trait TryExportLiteral<S, T, L> {
	type Error;

	fn try_export_literal(&self, literal: Literal<S, T, L>) -> Result<Literal, Self::Error>;
}

#[derive(Debug, thiserror::Error)]
#[error("unknown literal type {0}")]
pub struct UnknownType<I>(pub I);

impl<V: IriVocabulary> TryExportLiteral<String, V::Iri, LanguageTagBuf> for V {
	type Error = UnknownType<V::Iri>;

	fn try_export_literal(
		&self,
		literal: Literal<String, V::Iri, LanguageTagBuf>,
	) -> Result<Literal, Self::Error> {
		match literal {
			Literal::String(s) => Ok(Literal::String(s)),
			Literal::TypedString(s, t) => Ok(Literal::TypedString(
				s,
				self.owned_iri(t).map_err(UnknownType)?,
			)),
			Literal::LangString(s, t) => Ok(Literal::LangString(s, t)),
		}
	}
}
