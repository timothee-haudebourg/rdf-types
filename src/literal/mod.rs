use crate::vocabulary::{
	EmbedIntoVocabulary, EmbeddedIntoVocabulary, ExtractFromVocabulary, ExtractedFromVocabulary,
	IriVocabulary, LiteralVocabularyMut,
};
use crate::{IsXsdStringIri, RdfDisplay};
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
			type_: self.type_.inserted_into_vocabulary(vocabulary),
		}
	}
}

impl<V: LiteralVocabularyMut> EmbedIntoVocabulary<V> for Literal<V::Iri> {
	type Embedded = V::Literal;

	fn embed_into_vocabulary(self, vocabulary: &mut V) -> Self::Embedded {
		vocabulary.insert_owned_literal(self)
	}
}

impl<V: LiteralVocabularyMut> EmbeddedIntoVocabulary<V> for Literal<V::Iri> {
	type Embedded = V::Literal;

	fn inserted_into_vocabulary(&self, vocabulary: &mut V) -> Self::Embedded {
		vocabulary.insert_literal(self)
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

	fn exported_from_vocabulary(&self, vocabulary: &V) -> Self::Extracted {
		Literal::new(
			self.value.clone(),
			self.type_.exported_from_vocabulary(vocabulary),
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
