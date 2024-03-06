use core::fmt;

use iref::{Iri, IriBuf};
use langtag::{LangTag, LangTagBuf};

use crate::{
	vocabulary::{
		EmbedIntoVocabulary, EmbeddedIntoVocabulary, ExtractFromVocabulary,
		ExtractedFromVocabulary, IriVocabulary,
	},
	IsXsdStringIri, RdfDisplay, XSD_STRING,
};

/// RDF literal type.
#[derive(Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum LiteralType<I = IriBuf> {
	/// Any type.
	Any(I),

	/// Language string.
	LangString(LangTagBuf),
}

impl<I> LiteralType<I> {
	pub fn is_lang_string(&self) -> bool {
		matches!(self, Self::LangString(_))
	}

	pub fn lang_tag(&self) -> Option<&LangTag> {
		match self {
			Self::LangString(tag) => Some(tag),
			_ => None,
		}
	}

	pub fn is_xsd_string_with(&self, vocabulary: &impl IriVocabulary<Iri = I>) -> bool {
		match self {
			Self::Any(i) => vocabulary.iri(i).is_some_and(|iri| iri == XSD_STRING),
			Self::LangString(_) => false,
		}
	}

	pub fn is_xsd_string(&self) -> bool
	where
		I: IsXsdStringIri,
	{
		match self {
			Self::Any(iri) => iri.is_xsd_string_iri(),
			Self::LangString(_) => false,
		}
	}

	pub fn is_iri(&self, iri: &I) -> bool
	where
		I: PartialEq,
	{
		match self {
			Self::Any(i) => i == iri,
			Self::LangString(_) => false,
		}
	}

	pub fn as_lexical_type_ref_with<'a>(
		&'a self,
		vocabulary: &'a impl IriVocabulary<Iri = I>,
	) -> LexicalLiteralTypeRef<'a> {
		match self {
			Self::Any(i) => LexicalLiteralTypeRef::Any(vocabulary.iri(i).unwrap()),
			Self::LangString(l) => LexicalLiteralTypeRef::LangString(l),
		}
	}
}

impl LiteralType {
	pub fn as_lexical_type_ref(&self) -> LexicalLiteralTypeRef {
		match self {
			Self::Any(i) => LexicalLiteralTypeRef::Any(i),
			Self::LangString(l) => LexicalLiteralTypeRef::LangString(l),
		}
	}
}

impl<V, I: EmbedIntoVocabulary<V>> EmbedIntoVocabulary<V> for LiteralType<I> {
	type Embedded = LiteralType<I::Embedded>;

	fn embed_into_vocabulary(self, vocabulary: &mut V) -> Self::Embedded {
		match self {
			Self::Any(i) => LiteralType::Any(i.embed_into_vocabulary(vocabulary)),
			Self::LangString(l) => LiteralType::LangString(l),
		}
	}
}

impl<V, I: EmbeddedIntoVocabulary<V>> EmbeddedIntoVocabulary<V> for LiteralType<I> {
	type Embedded = LiteralType<I::Embedded>;

	fn inserted_into_vocabulary(&self, vocabulary: &mut V) -> Self::Embedded {
		match self {
			Self::Any(i) => LiteralType::Any(i.inserted_into_vocabulary(vocabulary)),
			Self::LangString(l) => LiteralType::LangString(l.clone()),
		}
	}
}

impl<V: IriVocabulary> ExtractFromVocabulary<V> for LiteralType<V::Iri> {
	type Extracted = LiteralType;

	fn extract_from_vocabulary(self, vocabulary: &V) -> Self::Extracted {
		match self {
			Self::Any(t) => LiteralType::Any(vocabulary.owned_iri(t).ok().unwrap()),
			Self::LangString(t) => LiteralType::LangString(t),
		}
	}
}

impl<V: IriVocabulary> ExtractedFromVocabulary<V> for LiteralType<V::Iri> {
	type Extracted = LiteralType;

	fn exported_from_vocabulary(&self, vocabulary: &V) -> Self::Extracted {
		match self {
			Self::Any(t) => LiteralType::Any(vocabulary.iri(t).unwrap().to_owned()),
			Self::LangString(t) => LiteralType::LangString(t.clone()),
		}
	}
}

/// Literal type IRI.
///
/// This trait is used to correctly format literal type IRIs, which can be
/// omitted when it is [`XSD_STRING`].
pub trait RdfTypeIri {
	/// Checks if the type IRI is [`XSD_STRING`].
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

/// Literal type IRI.
///
/// This trait is used to correctly format literal type IRIs, which can be
/// omitted when it is [`XSD_STRING`].
pub trait RdfTypeIriWithContext<C> {
	/// Checks if the type IRI is [`XSD_STRING`] using the given context.
	fn is_xsd_string_with(&self, context: &C) -> bool;
}

impl<C> RdfTypeIriWithContext<C> for IriBuf {
	fn is_xsd_string_with(&self, _context: &C) -> bool {
		self == XSD_STRING
	}
}

impl<C> RdfTypeIriWithContext<C> for Iri {
	fn is_xsd_string_with(&self, _context: &C) -> bool {
		self == XSD_STRING
	}
}

impl<'a, C, T: RdfTypeIriWithContext<C>> RdfTypeIriWithContext<C> for &'a T {
	fn is_xsd_string_with(&self, context: &C) -> bool {
		T::is_xsd_string_with(self, context)
	}
}

impl<I: RdfDisplay> RdfDisplay for LiteralType<I> {
	fn rdf_fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			Self::Any(ty) => {
				write!(f, "^^")?;
				ty.rdf_fmt(f)
			}
			Self::LangString(tag) => {
				write!(f, "@")?;
				tag.rdf_fmt(f)
			}
		}
	}
}

#[cfg(feature = "contextual")]
impl<T: crate::RdfDisplayWithContext<V>, V> crate::RdfDisplayWithContext<V> for LiteralType<T> {
	fn rdf_fmt_with(&self, vocabulary: &V, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			Self::Any(ty) => {
				write!(f, "^^")?;
				ty.rdf_fmt_with(vocabulary, f)
			}
			Self::LangString(tag) => {
				write!(f, "@")?;
				tag.rdf_fmt_with(vocabulary, f)
			}
		}
	}
}

/// RDF literal type.
#[derive(Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub enum LexicalLiteralTypeRef<'a> {
	/// Any type.
	Any(&'a Iri),

	/// Language string.
	LangString(&'a LangTag),
}

impl<'a> LexicalLiteralTypeRef<'a> {
	pub fn is_iri(&self, iri: &Iri) -> bool {
		match self {
			Self::Any(i) => *i == iri,
			Self::LangString(_) => false,
		}
	}
}
