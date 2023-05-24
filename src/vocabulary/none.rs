use super::{BlankIdVocabulary, BlankIdVocabularyMut, IriVocabulary, IriVocabularyMut};
use crate::{
	literal, BlankId, BlankIdBuf, LanguageTagVocabulary, LanguageTagVocabularyMut, Literal,
	LiteralVocabulary, LiteralVocabularyMut,
};
use iref::{Iri, IriBuf};
use langtag::{LanguageTag, LanguageTagBuf};

/// No vocabulary.
///
/// This is an alias to the unit type.
/// This vocabulary does not store anything.
pub type NoVocabulary = ();

static mut NO_VOCABULARY: NoVocabulary = ();

/// Returns a static reference to unit (no vocabulary).
#[inline(always)]
pub fn no_vocabulary() -> &'static NoVocabulary {
	unsafe { &NO_VOCABULARY }
}

/// Returns a static mutable reference to unit (no vocabulary).
#[inline(always)]
pub fn no_vocabulary_mut() -> &'static mut NoVocabulary {
	unsafe { &mut NO_VOCABULARY }
}

impl IriVocabulary for NoVocabulary {
	type Iri = IriBuf;

	fn iri<'i>(&'i self, id: &'i IriBuf) -> Option<Iri<'i>> {
		Some(id.as_iri())
	}

	fn owned_iri(&self, id: Self::Iri) -> Result<IriBuf, Self::Iri> {
		Ok(id)
	}

	fn get(&self, iri: Iri) -> Option<IriBuf> {
		Some(iri.into())
	}
}

impl IriVocabularyMut for NoVocabulary {
	fn insert(&mut self, iri: Iri) -> IriBuf {
		iri.into()
	}

	fn insert_owned(&mut self, iri: IriBuf) -> Self::Iri {
		iri
	}
}

impl BlankIdVocabulary for NoVocabulary {
	type BlankId = BlankIdBuf;

	fn blank_id<'b>(&'b self, id: &'b BlankIdBuf) -> Option<&'b BlankId> {
		Some(id.as_blank_id_ref())
	}

	fn owned_blank_id(&self, id: Self::BlankId) -> Result<BlankIdBuf, Self::BlankId> {
		Ok(id)
	}

	fn get_blank_id(&self, id: &BlankId) -> Option<BlankIdBuf> {
		Some(id.to_owned())
	}
}

impl BlankIdVocabularyMut for NoVocabulary {
	fn insert_blank_id(&mut self, id: &BlankId) -> BlankIdBuf {
		id.to_owned()
	}

	fn insert_owned_blank_id(&mut self, id: BlankIdBuf) -> Self::BlankId {
		id
	}
}

impl LiteralVocabulary for NoVocabulary {
	type Literal = Literal<Self::Type, Self::Value>;

	type Type = literal::Type<IriBuf, LanguageTagBuf>;

	type Value = String;

	fn literal<'l>(
		&'l self,
		id: &'l Self::Literal,
	) -> Option<&'l Literal<Self::Type, Self::Value>> {
		Some(id)
	}

	fn owned_literal(
		&self,
		id: Self::Literal,
	) -> Result<Literal<Self::Type, Self::Value>, Self::Literal> {
		Ok(id)
	}

	fn get_literal(&self, id: &Literal<Self::Type, Self::Value>) -> Option<Self::Literal> {
		Some(id.to_owned())
	}
}

impl LiteralVocabularyMut for NoVocabulary {
	fn insert_literal(&mut self, value: &Literal<Self::Type, Self::Value>) -> Self::Literal {
		value.to_owned()
	}

	fn insert_owned_literal(&mut self, value: Literal<Self::Type, Self::Value>) -> Self::Literal {
		value
	}
}

impl LanguageTagVocabulary for NoVocabulary {
	type LanguageTag = LanguageTagBuf;

	fn language_tag<'l>(&'l self, id: &'l Self::LanguageTag) -> Option<LanguageTag<'l>> {
		Some(id.as_ref())
	}

	fn owned_language_tag(
		&self,
		id: Self::LanguageTag,
	) -> Result<LanguageTagBuf, Self::LanguageTag> {
		Ok(id)
	}

	fn get_language_tag(&self, id: LanguageTag) -> Option<Self::LanguageTag> {
		Some(id.cloned())
	}
}

impl LanguageTagVocabularyMut for NoVocabulary {
	fn insert_language_tag(&mut self, value: LanguageTag) -> Self::LanguageTag {
		value.cloned()
	}

	fn insert_owned_language_tag(&mut self, value: LanguageTagBuf) -> Self::LanguageTag {
		value
	}
}
