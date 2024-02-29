use crate::{BlankId, BlankIdBuf};

use super::{EmbedIntoVocabulary, EmbeddedIntoVocabulary};

/// Blank node identifier vocabulary.
pub trait BlankIdVocabulary {
	type BlankId;

	/// Returns the blank node identifier associated to the given id.
	fn blank_id<'b>(&'b self, id: &'b Self::BlankId) -> Option<&'b BlankId>;

	/// Returns a copy of the blank node identifier associated to the given id.
	fn owned_blank_id(&self, id: Self::BlankId) -> Result<BlankIdBuf, Self::BlankId> {
		self.blank_id(&id).map(BlankId::to_owned).ok_or(id)
	}

	/// Returns the vocabulary id of the given blank node identifier, if any.
	fn get_blank_id(&self, id: &BlankId) -> Option<Self::BlankId>;
}

impl<'a, V: BlankIdVocabulary> BlankIdVocabulary for &'a V {
	type BlankId = V::BlankId;

	fn blank_id<'b>(&'b self, id: &'b Self::BlankId) -> Option<&'b BlankId> {
		V::blank_id(*self, id)
	}

	fn owned_blank_id(&self, id: Self::BlankId) -> Result<BlankIdBuf, Self::BlankId> {
		V::owned_blank_id(*self, id)
	}

	fn get_blank_id(&self, id: &BlankId) -> Option<Self::BlankId> {
		V::get_blank_id(*self, id)
	}
}

impl<'a, V: BlankIdVocabulary> BlankIdVocabulary for &'a mut V {
	type BlankId = V::BlankId;

	fn blank_id<'b>(&'b self, id: &'b Self::BlankId) -> Option<&'b BlankId> {
		V::blank_id(*self, id)
	}

	fn owned_blank_id(&self, id: Self::BlankId) -> Result<BlankIdBuf, Self::BlankId> {
		V::owned_blank_id(*self, id)
	}

	fn get_blank_id(&self, id: &BlankId) -> Option<Self::BlankId> {
		V::get_blank_id(*self, id)
	}
}

/// Mutable blank node identifier vocabulary.
pub trait BlankIdVocabularyMut: BlankIdVocabulary {
	/// Inserts a blank node identifier to the vocabulary and returns its id.
	///
	/// If the blank id was already present in the vocabulary, no new
	/// vocabulary id is created and the current one is returned.
	fn insert_blank_id(&mut self, id: &BlankId) -> Self::BlankId;

	fn insert_owned_blank_id(&mut self, id: BlankIdBuf) -> Self::BlankId {
		self.insert_blank_id(id.as_blank_id_ref())
	}
}

impl<'a, V: BlankIdVocabularyMut> BlankIdVocabularyMut for &'a mut V {
	fn insert_blank_id(&mut self, id: &BlankId) -> Self::BlankId {
		V::insert_blank_id(*self, id)
	}

	fn insert_owned_blank_id(&mut self, id: BlankIdBuf) -> Self::BlankId {
		V::insert_owned_blank_id(*self, id)
	}
}

impl<'a, V: BlankIdVocabularyMut> EmbedIntoVocabulary<V> for &'a BlankId {
	type Embedded = V::BlankId;

	fn embed_into_vocabulary(self, vocabulary: &mut V) -> Self::Embedded {
		vocabulary.insert_blank_id(self)
	}
}

impl<V: BlankIdVocabularyMut> EmbedIntoVocabulary<V> for BlankIdBuf {
	type Embedded = V::BlankId;

	fn embed_into_vocabulary(self, vocabulary: &mut V) -> Self::Embedded {
		vocabulary.insert_owned_blank_id(self)
	}
}

impl<'a, V: BlankIdVocabularyMut> EmbeddedIntoVocabulary<V> for &'a BlankId {
	type Embedded = V::BlankId;

	fn inserted_into_vocabulary(&self, vocabulary: &mut V) -> Self::Embedded {
		vocabulary.insert_blank_id(self)
	}
}

impl<V: BlankIdVocabularyMut> EmbeddedIntoVocabulary<V> for BlankIdBuf {
	type Embedded = V::BlankId;

	fn inserted_into_vocabulary(&self, vocabulary: &mut V) -> Self::Embedded {
		vocabulary.insert_blank_id(self.as_blank_id_ref())
	}
}
