use crate::vocabulary::{ExportFromVocabulary, ExportedFromVocabulary};
use crate::{
	InsertIntoVocabulary, InsertedIntoVocabulary, IriVocabulary, LanguageTagVocabulary,
	LiteralVocabulary, LiteralVocabularyMut, RdfDisplay, XSD_STRING,
};
use iref::{Iri, IriBuf};
use langtag::LanguageTagBuf;
use std::borrow::{Borrow, BorrowMut};
use std::fmt;

#[cfg(feature = "contextual")]
use contextual::DisplayWithContext;

#[cfg(feature = "meta")]
use locspan_derive::{
	StrippedEq, StrippedHash, StrippedOrd, StrippedPartialEq, StrippedPartialOrd,
};

mod map;

pub use map::*;

/// RDF Literal.
#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(
	feature = "meta",
	derive(
		StrippedPartialEq,
		StrippedEq,
		StrippedPartialOrd,
		StrippedOrd,
		StrippedHash,
	)
)]
pub struct Literal<T = Type<IriBuf, LanguageTagBuf>, S = String> {
	/// Literal value.
	value: S,

	/// Literal type.
	type_: T,
}

impl<T, S> Literal<T, S> {
	pub fn new(value: S, type_: T) -> Self {
		Self { value, type_ }
	}

	pub fn type_(&self) -> &T {
		&self.type_
	}

	pub fn type_mut(&mut self) -> &mut T {
		&mut self.type_
	}

	pub fn into_type_(self) -> T {
		self.type_
	}

	pub fn value(&self) -> &S {
		&self.value
	}

	pub fn value_mut(&mut self) -> &mut S {
		&mut self.value
	}

	pub fn into_value(self) -> S {
		self.value
	}

	pub fn into_parts(self) -> (S, T) {
		(self.value, self.type_)
	}

	pub fn as_str(&self) -> &str
	where
		S: AsRef<str>,
	{
		self.value.as_ref()
	}

	pub fn as_str_mut(&mut self) -> &mut str
	where
		S: AsMut<str>,
	{
		self.value.as_mut()
	}

	pub fn as_bytes(&self) -> &[u8]
	where
		S: AsRef<[u8]>,
	{
		self.value.as_ref()
	}

	pub fn as_bytes_mut(&mut self) -> &mut [u8]
	where
		S: AsMut<[u8]>,
	{
		self.value.as_mut()
	}

	pub fn insert_type_into_vocabulary<V>(self, vocabulary: &mut V) -> Literal<T::Inserted, S>
	where
		T: InsertIntoVocabulary<V>,
	{
		Literal {
			value: self.value,
			type_: self.type_.insert_into_vocabulary(vocabulary),
		}
	}

	pub fn inserted_type_into_vocabulary<V>(&self, vocabulary: &mut V) -> Literal<T::Inserted, S>
	where
		T: InsertedIntoVocabulary<V>,
		S: Clone,
	{
		Literal {
			value: self.value.clone(),
			type_: self.type_.inserted_into_vocabulary(vocabulary),
		}
	}
}

impl<I, L, S> Literal<Type<I, L>, S> {
	pub fn is_lang_string(&self) -> bool {
		self.type_.is_lang_string()
	}

	pub fn lang_tag(&self) -> Option<&L> {
		self.type_.lang_tag()
	}
}

impl<V: LiteralVocabularyMut> InsertIntoVocabulary<V> for Literal<V::Type, V::Value> {
	type Inserted = V::Literal;

	fn insert_into_vocabulary(self, vocabulary: &mut V) -> Self::Inserted {
		vocabulary.insert_literal(&self)
	}
}

impl<V: LiteralVocabularyMut> InsertedIntoVocabulary<V> for Literal<V::Type, V::Value> {
	type Inserted = V::Literal;

	fn inserted_into_vocabulary(&self, vocabulary: &mut V) -> Self::Inserted {
		vocabulary.insert_literal(self)
	}
}

impl<V: LiteralVocabulary> ExportFromVocabulary<V> for Literal<V::Type, V::Value>
where
	V::Type: ExportFromVocabulary<V>,
{
	type Output = Literal<<V::Type as ExportFromVocabulary<V>>::Output, V::Value>;

	fn export_from_vocabulary(self, vocabulary: &V) -> Self::Output {
		let (value, type_) = self.into_parts();
		Literal::new(value, type_.export_from_vocabulary(vocabulary))
	}
}

impl<V: LiteralVocabulary> ExportedFromVocabulary<V> for Literal<V::Type, V::Value>
where
	V::Type: ExportedFromVocabulary<V>,
	V::Value: Clone,
{
	type Output = Literal<<V::Type as ExportedFromVocabulary<V>>::Output, V::Value>;

	fn exported_from_vocabulary(&self, vocabulary: &V) -> Self::Output {
		Literal::new(
			self.value.clone(),
			self.type_.exported_from_vocabulary(vocabulary),
		)
	}
}

impl<T, S: AsRef<str>> Borrow<str> for Literal<T, S> {
	fn borrow(&self) -> &str {
		self.as_str()
	}
}

impl<T, S: AsRef<str> + AsMut<str>> BorrowMut<str> for Literal<T, S> {
	fn borrow_mut(&mut self) -> &mut str {
		self.as_str_mut()
	}
}

impl<T, S: AsRef<str>> AsRef<str> for Literal<T, S> {
	fn as_ref(&self) -> &str {
		self.as_str()
	}
}

impl<T, S: AsMut<str>> AsMut<str> for Literal<T, S> {
	fn as_mut(&mut self) -> &mut str {
		self.as_str_mut()
	}
}

impl<T: RdfDisplay + RdfDisplayType, S: RdfDisplay> fmt::Display for Literal<T, S> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		self.value.rdf_fmt(f)?;
		if self.type_.omit() {
			Ok(())
		} else {
			self.type_.rdf_fmt_type_separator(f)?;
			self.type_.rdf_fmt(f)
		}
	}
}

impl<T: RdfDisplay + RdfDisplayType, S: RdfDisplay> RdfDisplay for Literal<T, S> {
	fn rdf_fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		self.value.rdf_fmt(f)?;
		if self.type_.omit() {
			Ok(())
		} else {
			self.type_.rdf_fmt_type_separator(f)?;
			self.type_.rdf_fmt(f)
		}
	}
}

#[cfg(feature = "contextual")]
impl<T: crate::RdfDisplayWithContext<V> + RdfDisplayTypeWithContext<V>, S: RdfDisplay, V>
	DisplayWithContext<V> for Literal<T, S>
{
	fn fmt_with(&self, vocabulary: &V, f: &mut fmt::Formatter) -> fmt::Result {
		self.value.rdf_fmt(f)?;
		if self.type_.omit_with(vocabulary) {
			Ok(())
		} else {
			self.type_.rdf_fmt_type_separator_with(vocabulary, f)?;
			self.type_.rdf_fmt_with(vocabulary, f)
		}
	}
}

#[cfg(feature = "contextual")]
impl<T: crate::RdfDisplayWithContext<V> + RdfDisplayTypeWithContext<V>, S: RdfDisplay, V>
	crate::RdfDisplayWithContext<V> for Literal<T, S>
{
	fn rdf_fmt_with(&self, vocabulary: &V, f: &mut fmt::Formatter) -> fmt::Result {
		self.value.rdf_fmt(f)?;
		if self.type_.omit_with(vocabulary) {
			Ok(())
		} else {
			self.type_.rdf_fmt_type_separator_with(vocabulary, f)?;
			self.type_.rdf_fmt_with(vocabulary, f)
		}
	}
}

/// RDF Literal type.
#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(
	feature = "meta",
	derive(
		StrippedPartialEq,
		StrippedEq,
		StrippedPartialOrd,
		StrippedOrd,
		StrippedHash,
	)
)]
pub enum Type<I = IriBuf, L = LanguageTagBuf> {
	/// Any type.
	Any(I),

	/// Language string.
	LangString(L),
}

impl<I, L> Type<I, L> {
	pub fn is_lang_string(&self) -> bool {
		matches!(self, Self::LangString(_))
	}

	pub fn lang_tag(&self) -> Option<&L> {
		match self {
			Self::LangString(tag) => Some(tag),
			_ => None,
		}
	}
}

impl<V, I: InsertIntoVocabulary<V>, L: InsertIntoVocabulary<V>> InsertIntoVocabulary<V>
	for Type<I, L>
{
	type Inserted = Type<I::Inserted, L::Inserted>;

	fn insert_into_vocabulary(self, vocabulary: &mut V) -> Self::Inserted {
		match self {
			Self::Any(i) => Type::Any(i.insert_into_vocabulary(vocabulary)),
			Self::LangString(l) => Type::LangString(l.insert_into_vocabulary(vocabulary)),
		}
	}
}

impl<V: IriVocabulary + LanguageTagVocabulary> ExportFromVocabulary<V>
	for Type<V::Iri, V::LanguageTag>
{
	type Output = Type;

	fn export_from_vocabulary(self, vocabulary: &V) -> Self::Output {
		match self {
			Self::Any(t) => Type::Any(vocabulary.owned_iri(t).ok().unwrap()),
			Self::LangString(t) => Type::LangString(vocabulary.owned_language_tag(t).ok().unwrap()),
		}
	}
}

impl<V: IriVocabulary + LanguageTagVocabulary> ExportedFromVocabulary<V>
	for Type<V::Iri, V::LanguageTag>
{
	type Output = Type;

	fn exported_from_vocabulary(&self, vocabulary: &V) -> Self::Output {
		match self {
			Self::Any(t) => Type::Any(vocabulary.iri(t).unwrap().to_owned()),
			Self::LangString(t) => Type::LangString(vocabulary.language_tag(t).unwrap().cloned()),
		}
	}
}

pub trait RdfTypeIri {
	fn is_xsd_string(&self) -> bool;
}

impl RdfTypeIri for IriBuf {
	fn is_xsd_string(&self) -> bool {
		self == XSD_STRING
	}
}

impl RdfTypeIri for Iri {
	fn is_xsd_string(&self) -> bool {
		self == XSD_STRING
	}
}

impl<'a, T: RdfTypeIri> RdfTypeIri for &'a T {
	fn is_xsd_string(&self) -> bool {
		T::is_xsd_string(self)
	}
}

pub trait RdfTypeIriWithContext<C> {
	fn is_xsd_string_with(&self, context: &C) -> bool;
}

pub trait RdfDisplayType {
	fn omit(&self) -> bool;

	fn rdf_fmt_type_separator(&self, f: &mut fmt::Formatter) -> fmt::Result;
}

impl<T: RdfTypeIri, L> RdfDisplayType for Type<T, L> {
	fn omit(&self) -> bool {
		match self {
			Self::Any(t) => t.is_xsd_string(),
			Self::LangString(_) => false,
		}
	}

	fn rdf_fmt_type_separator(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			Self::Any(_) => write!(f, "^^"),
			Self::LangString(_) => write!(f, "@"),
		}
	}
}

pub trait RdfDisplayTypeWithContext<C> {
	fn omit_with(&self, context: &C) -> bool;

	fn rdf_fmt_type_separator_with(&self, context: &C, f: &mut fmt::Formatter) -> fmt::Result;
}

impl<T: RdfDisplay, L: RdfDisplay> RdfDisplay for Type<T, L> {
	fn rdf_fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			Self::Any(ty) => ty.rdf_fmt(f),
			Self::LangString(tag) => tag.rdf_fmt(f),
		}
	}
}

#[cfg(feature = "contextual")]
impl<T: crate::RdfDisplayWithContext<V>, L: crate::RdfDisplayWithContext<V>, V>
	crate::RdfDisplayWithContext<V> for Type<T, L>
{
	fn rdf_fmt_with(&self, vocabulary: &V, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			Self::Any(ty) => ty.rdf_fmt_with(vocabulary, f),
			Self::LangString(tag) => tag.rdf_fmt_with(vocabulary, f),
		}
	}
}

pub trait AsLiteral {
	/// Literal value type.
	type Value;

	/// Literal type value type.
	type Type;

	/// Turns the reference into a `Literal` referencing its components.
	fn as_literal(&self) -> Literal<&Self::Type, &Self::Value>;
}

impl<T, S> AsLiteral for Literal<T, S> {
	type Value = S;

	type Type = T;

	fn as_literal(&self) -> Literal<&T, &S> {
		Literal {
			type_: &self.type_,
			value: &self.value,
		}
	}
}

/// Type that can be converted into a [`Literal`].
pub trait IntoLiteral: AsLiteral {
	/// Turns the value into a `Literal`.
	fn into_literal(self) -> Literal<Self::Type, Self::Value>;
}

impl<T, S> IntoLiteral for Literal<T, S> {
	fn into_literal(self) -> Self {
		self
	}
}

/// Type that can turn a value into a `Literal`.
pub trait TryExportLiteral<V> {
	type Error;

	fn try_export_literal(self, vocabulary: &V) -> Result<Literal, Self::Error>;
}

#[derive(Debug, thiserror::Error)]
#[error("unknown literal type {0}")]
pub struct UnknownType<I>(pub I);

impl<T: TryExportLiteralType<V>, S: Into<String>, V> TryExportLiteral<V> for Literal<T, S> {
	type Error = T::Error;

	fn try_export_literal(self, vocabulary: &V) -> Result<Literal, Self::Error> {
		Ok(Literal {
			value: self.value.into(),
			type_: self.type_.try_export_literal_type(vocabulary)?,
		})
	}
}

pub trait TryExportLiteralType<V> {
	type Error;

	fn try_export_literal_type(self, vocabulary: &V) -> Result<Type, Self::Error>;
}

impl<V: IriVocabulary + LanguageTagVocabulary> TryExportLiteralType<V>
	for Type<V::Iri, V::LanguageTag>
{
	type Error = ExportError<V::Iri, V::LanguageTag>;

	fn try_export_literal_type(self, vocabulary: &V) -> Result<Type, Self::Error> {
		match self {
			Self::Any(ty) => Ok(Type::Any(
				vocabulary.owned_iri(ty).map_err(ExportError::Iri)?,
			)),
			Self::LangString(tag) => Ok(Type::LangString(
				vocabulary
					.owned_language_tag(tag)
					.map_err(ExportError::LangTag)?,
			)),
		}
	}
}

pub enum ExportError<I, L> {
	Iri(I),
	LangTag(L),
}
