use crate::vocabulary::{
	EmbedIntoVocabulary, EmbeddedIntoVocabulary, ExtractFromVocabulary, ExtractedFromVocabulary,
	IriVocabulary, IriVocabularyMut, LiteralVocabularyMut,
};
use crate::{IsXsdStringIri, RdfDisplay};
use educe::Educe;
use iref::IriBuf;
use langtag::LangTag;
use std::borrow::Borrow;
use std::fmt;

#[cfg(feature = "contextual")]
use contextual::DisplayWithContext;

mod r#type;
pub use r#type::*;

/// RDF Literal.
#[derive(Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Literal<I = IriBuf> {
	/// Literal value.
	pub value: String,

	/// Literal type.
	pub type_: LiteralType<I>,
}

impl<I> Literal<I> {
	pub fn new(value: String, type_: LiteralType<I>) -> Self {
		Self { value, type_ }
	}

	pub fn as_type(&self) -> &LiteralType<I> {
		&self.type_
	}

	pub fn as_type_mut(&mut self) -> &mut LiteralType<I> {
		&mut self.type_
	}

	pub fn into_type(self) -> LiteralType<I> {
		self.type_
	}

	pub fn as_value(&self) -> &String {
		&self.value
	}

	pub fn as_value_mut(&mut self) -> &mut String {
		&mut self.value
	}

	pub fn into_value(self) -> String {
		self.value
	}

	pub fn into_parts(self) -> (String, LiteralType<I>) {
		(self.value, self.type_)
	}

	pub fn as_str(&self) -> &str {
		self.value.as_ref()
	}

	pub fn as_bytes(&self) -> &[u8] {
		self.value.as_ref()
	}

	pub fn is_lang_string(&self) -> bool {
		self.type_.is_lang_string()
	}

	pub fn lang_tag(&self) -> Option<&LangTag> {
		self.type_.lang_tag()
	}

	pub fn insert_type_into_vocabulary<V>(self, vocabulary: &mut V) -> Literal<I::Embedded>
	where
		I: EmbedIntoVocabulary<V>,
	{
		Literal {
			value: self.value,
			type_: self.type_.embed_into_vocabulary(vocabulary),
		}
	}

	pub fn inserted_type_into_vocabulary<V>(&self, vocabulary: &mut V) -> Literal<I::Embedded>
	where
		I: EmbeddedIntoVocabulary<V>,
	{
		Literal {
			value: self.value.clone(),
			type_: self.type_.embedded_into_vocabulary(vocabulary),
		}
	}

	pub fn as_ref(&self) -> LiteralRef<I> {
		LiteralRef::new(&self.value, self.type_.as_ref())
	}
}

impl<'a, I: PartialEq> PartialEq<LiteralRef<'a, I>> for Literal<I> {
	fn eq(&self, other: &LiteralRef<'a, I>) -> bool {
		self.type_ == other.type_ && self.value == other.value
	}
}

impl<V: IriVocabularyMut + LiteralVocabularyMut> EmbedIntoVocabulary<V> for Literal {
	type Embedded = V::Literal;

	fn embed_into_vocabulary(self, vocabulary: &mut V) -> Self::Embedded {
		let l = self.insert_type_into_vocabulary(vocabulary);
		vocabulary.insert_owned_literal(l)
	}
}

impl<V: IriVocabularyMut + LiteralVocabularyMut> EmbeddedIntoVocabulary<V> for Literal {
	type Embedded = V::Literal;

	fn embedded_into_vocabulary(&self, vocabulary: &mut V) -> Self::Embedded {
		let l = self.inserted_type_into_vocabulary(vocabulary);
		vocabulary.insert_owned_literal(l)
	}
}

impl<V: IriVocabulary> ExtractFromVocabulary<V> for Literal<V::Iri> {
	type Extracted = Literal;

	fn extract_from_vocabulary(self, vocabulary: &V) -> Self::Extracted {
		let (value, type_) = self.into_parts();
		Literal::new(value, type_.extract_from_vocabulary(vocabulary))
	}
}

impl<V: IriVocabulary> ExtractedFromVocabulary<V> for Literal<V::Iri> {
	type Extracted = Literal;

	fn extracted_from_vocabulary(&self, vocabulary: &V) -> Self::Extracted {
		Literal::new(
			self.value.clone(),
			self.type_.extracted_from_vocabulary(vocabulary),
		)
	}
}

impl<I> Borrow<str> for Literal<I> {
	fn borrow(&self) -> &str {
		self.as_str()
	}
}

impl<I> AsRef<str> for Literal<I> {
	fn as_ref(&self) -> &str {
		self.as_str()
	}
}

impl fmt::Display for Literal {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		self.value.rdf_fmt(f)?;
		if self.type_.is_xsd_string() {
			Ok(())
		} else {
			self.type_.rdf_fmt(f)
		}
	}
}

impl<I: RdfDisplay + IsXsdStringIri> RdfDisplay for Literal<I> {
	fn rdf_fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		self.value.rdf_fmt(f)?;
		if self.type_.is_xsd_string() {
			Ok(())
		} else {
			self.type_.rdf_fmt(f)
		}
	}
}

#[cfg(feature = "contextual")]
impl<V: crate::vocabulary::IriVocabulary> DisplayWithContext<V> for Literal<V::Iri>
where
	V::Iri: crate::RdfDisplayWithContext<V>,
{
	fn fmt_with(&self, vocabulary: &V, f: &mut fmt::Formatter) -> fmt::Result {
		use crate::RdfDisplayWithContext;
		self.value.rdf_fmt(f)?;
		if self.type_.is_xsd_string_with(vocabulary) {
			Ok(())
		} else {
			self.type_.rdf_fmt_with(vocabulary, f)
		}
	}
}

#[cfg(feature = "contextual")]
impl<V: crate::vocabulary::IriVocabulary> crate::RdfDisplayWithContext<V> for Literal<V::Iri>
where
	V::Iri: crate::RdfDisplayWithContext<V>,
{
	fn rdf_fmt_with(&self, vocabulary: &V, f: &mut fmt::Formatter) -> fmt::Result {
		self.value.rdf_fmt(f)?;
		if self.type_.is_xsd_string_with(vocabulary) {
			Ok(())
		} else {
			self.type_.rdf_fmt_with(vocabulary, f)
		}
	}
}

/// RDF Literal reference.
#[derive(Educe, PartialEq, Eq, Hash, PartialOrd, Ord, Debug)]
#[educe(Clone, Copy)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct LiteralRef<'a, I = IriBuf> {
	/// Literal value.
	pub value: &'a str,

	/// Literal type.
	pub type_: LiteralTypeRef<'a, I>,
}

impl<'a, I> LiteralRef<'a, I> {
	pub fn new(value: &'a str, type_: LiteralTypeRef<'a, I>) -> Self {
		Self { value, type_ }
	}

	pub fn as_type(&self) -> LiteralTypeRef<'a, I> {
		self.type_
	}

	pub fn as_type_mut(&mut self) -> &mut LiteralTypeRef<'a, I> {
		&mut self.type_
	}

	pub fn into_type(self) -> LiteralTypeRef<'a, I> {
		self.type_
	}

	pub fn as_value(&self) -> &'a str {
		self.value
	}

	pub fn into_value(self) -> &'a str {
		self.value
	}

	pub fn into_parts(self) -> (&'a str, LiteralTypeRef<'a, I>) {
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

	pub fn insert_type_into_vocabulary<V>(self, vocabulary: &mut V) -> Literal<I::Embedded>
	where
		I: EmbeddedIntoVocabulary<V>,
	{
		Literal {
			value: self.value.to_owned(),
			type_: self.type_.embed_into_vocabulary(vocabulary),
		}
	}

	pub fn inserted_type_into_vocabulary<V>(&self, vocabulary: &mut V) -> Literal<I::Embedded>
	where
		I: EmbeddedIntoVocabulary<V>,
	{
		Literal {
			value: self.value.to_owned(),
			type_: self.type_.embedded_into_vocabulary(vocabulary),
		}
	}
}

impl<'a, I: ToOwned> LiteralRef<'a, I> {
	pub fn into_owned(self) -> Literal<I::Owned> {
		Literal::new(self.value.to_owned(), self.type_.into_owned())
	}
}

impl<'a, I> LiteralRef<'a, I> {
	pub fn cast_into_owned<J>(self) -> Literal<J>
	where
		&'a I: Into<J>,
	{
		Literal::new(self.value.to_owned(), self.type_.cast_into_owned())
	}
}

impl<'a, I: PartialEq> PartialEq<Literal<I>> for LiteralRef<'a, I> {
	fn eq(&self, other: &Literal<I>) -> bool {
		self.type_ == other.type_ && self.value == other.value
	}
}

impl<'a, V: LiteralVocabularyMut> EmbedIntoVocabulary<V> for LiteralRef<'a, V::Iri>
where
	V::Iri: Clone,
{
	type Embedded = V::Literal;

	fn embed_into_vocabulary(self, vocabulary: &mut V) -> Self::Embedded {
		vocabulary.insert_literal(self)
	}
}

impl<'a, V: LiteralVocabularyMut> EmbeddedIntoVocabulary<V> for LiteralRef<'a, V::Iri>
where
	V::Iri: Clone,
{
	type Embedded = V::Literal;

	fn embedded_into_vocabulary(&self, vocabulary: &mut V) -> Self::Embedded {
		vocabulary.insert_literal(*self)
	}
}

impl<'a, V: IriVocabulary> ExtractFromVocabulary<V> for LiteralRef<'a, V::Iri> {
	type Extracted = Literal;

	fn extract_from_vocabulary(self, vocabulary: &V) -> Self::Extracted {
		let (value, type_) = self.into_parts();
		Literal::new(value.to_owned(), type_.extract_from_vocabulary(vocabulary))
	}
}

impl<'a, V: IriVocabulary> ExtractedFromVocabulary<V> for LiteralRef<'a, V::Iri> {
	type Extracted = Literal;

	fn extracted_from_vocabulary(&self, vocabulary: &V) -> Self::Extracted {
		Literal::new(
			self.value.to_owned(),
			self.type_.extracted_from_vocabulary(vocabulary),
		)
	}
}

impl<'a, I> Borrow<str> for LiteralRef<'a, I> {
	fn borrow(&self) -> &str {
		self.as_str()
	}
}

impl<'a, I> AsRef<str> for LiteralRef<'a, I> {
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

impl<'a, I: RdfDisplay + IsXsdStringIri> RdfDisplay for LiteralRef<'a, I> {
	fn rdf_fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		self.value.rdf_fmt(f)?;
		if self.type_.is_xsd_string() {
			Ok(())
		} else {
			self.type_.rdf_fmt(f)
		}
	}
}

#[cfg(feature = "contextual")]
impl<'a, V: crate::vocabulary::IriVocabulary> DisplayWithContext<V> for LiteralRef<'a, V::Iri>
where
	V::Iri: crate::RdfDisplayWithContext<V>,
{
	fn fmt_with(&self, vocabulary: &V, f: &mut fmt::Formatter) -> fmt::Result {
		use crate::RdfDisplayWithContext;
		self.value.rdf_fmt(f)?;
		if self.type_.is_xsd_string_with(vocabulary) {
			Ok(())
		} else {
			self.type_.rdf_fmt_with(vocabulary, f)
		}
	}
}

#[cfg(feature = "contextual")]
impl<'a, V: crate::vocabulary::IriVocabulary> crate::RdfDisplayWithContext<V>
	for LiteralRef<'a, V::Iri>
where
	V::Iri: crate::RdfDisplayWithContext<V>,
{
	fn rdf_fmt_with(&self, vocabulary: &V, f: &mut fmt::Formatter) -> fmt::Result {
		self.value.rdf_fmt(f)?;
		if self.type_.is_xsd_string_with(vocabulary) {
			Ok(())
		} else {
			self.type_.rdf_fmt_with(vocabulary, f)
		}
	}
}
