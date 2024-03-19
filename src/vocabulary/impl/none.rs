use crate::{
	vocabulary::{
		BlankIdVocabulary, BlankIdVocabularyMut, IriVocabulary, IriVocabularyMut,
		LiteralVocabulary, LiteralVocabularyMut,
	},
	BlankId, BlankIdBuf, Literal, LiteralRef,
};
use iref::{Iri, IriBuf};

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

	fn iri<'i>(&'i self, id: &'i IriBuf) -> Option<&'i Iri> {
		Some(id.as_iri())
	}

	fn owned_iri(&self, id: Self::Iri) -> Result<IriBuf, Self::Iri> {
		Ok(id)
	}

	fn get(&self, iri: &Iri) -> Option<IriBuf> {
		Some(iri.to_owned())
	}
}

impl IriVocabularyMut for NoVocabulary {
	fn insert(&mut self, iri: &Iri) -> IriBuf {
		iri.to_owned()
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
	type Literal = Literal;

	fn literal<'l>(&'l self, id: &'l Self::Literal) -> Option<LiteralRef<'l>> {
		Some(id.as_ref())
	}

	fn owned_literal(&self, id: Self::Literal) -> Result<Literal, Self::Literal> {
		Ok(id)
	}

	fn get_literal(&self, id: LiteralRef<Self::Iri>) -> Option<Self::Literal> {
		Some(id.into_owned())
	}
}

impl LiteralVocabularyMut for NoVocabulary {
	fn insert_literal(&mut self, value: LiteralRef<Self::Iri>) -> Self::Literal {
		value.into_owned()
	}

	fn insert_owned_literal(&mut self, value: Literal) -> Self::Literal {
		value
	}
}
