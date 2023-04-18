use super::{BlankIdVocabulary, BlankIdVocabularyMut, IriVocabulary, IriVocabularyMut};
use crate::{BlankId, BlankIdBuf};
use iref::{Iri, IriBuf};

/// No vocabulary.
///
/// This is an alias to the unit type.
/// This vocabulary does not store anything.
pub type NoVocabulary = ();

static mut NO_NAMESPACE: NoVocabulary = ();

/// Returns a static reference to unit (no vocabulary).
#[inline(always)]
pub fn no_vocabulary() -> &'static NoVocabulary {
	unsafe { &NO_NAMESPACE }
}

/// Returns a static mutable reference to unit (no vocabulary).
#[inline(always)]
pub fn no_vocabulary_mut() -> &'static mut NoVocabulary {
	unsafe { &mut NO_NAMESPACE }
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
}
