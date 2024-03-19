use iref::{Iri, IriBuf};

use super::{EmbedIntoVocabulary, EmbeddedIntoVocabulary};

/// IRI vocabulary.
pub trait IriVocabulary {
	type Iri;

	/// Returns the IRI associated to the given IRI id.
	fn iri<'i>(&'i self, id: &'i Self::Iri) -> Option<&'i Iri>;

	/// Returns a copy of the IRI associated to the given IRI id.
	fn owned_iri(&self, id: Self::Iri) -> Result<IriBuf, Self::Iri> {
		self.iri(&id).map(Iri::to_owned).ok_or(id)
	}

	/// Returns the id of the given IRI, if any.
	fn get(&self, iri: &Iri) -> Option<Self::Iri>;
}

impl<'a, V: IriVocabulary> IriVocabulary for &'a V {
	type Iri = V::Iri;

	fn iri<'i>(&'i self, id: &'i Self::Iri) -> Option<&'i Iri> {
		V::iri(*self, id)
	}

	fn owned_iri(&self, id: Self::Iri) -> Result<IriBuf, Self::Iri> {
		V::owned_iri(*self, id)
	}

	fn get(&self, iri: &Iri) -> Option<Self::Iri> {
		V::get(*self, iri)
	}
}

impl<'a, V: IriVocabulary> IriVocabulary for &'a mut V {
	type Iri = V::Iri;

	fn iri<'i>(&'i self, id: &'i Self::Iri) -> Option<&'i Iri> {
		V::iri(*self, id)
	}

	fn owned_iri(&self, id: Self::Iri) -> Result<IriBuf, Self::Iri> {
		V::owned_iri(*self, id)
	}

	fn get(&self, iri: &Iri) -> Option<Self::Iri> {
		V::get(*self, iri)
	}
}

/// Mutable IRI vocabulary.
pub trait IriVocabularyMut: IriVocabulary {
	/// Inserts an IRI to the vocabulary and returns its id.
	///
	/// If the IRI was already present in the vocabulary, no new id is created
	/// and the current one is returned.
	fn insert(&mut self, iri: &Iri) -> Self::Iri;

	fn insert_owned(&mut self, iri: IriBuf) -> Self::Iri {
		self.insert(iri.as_iri())
	}
}

impl<'a, V: IriVocabularyMut> IriVocabularyMut for &'a mut V {
	fn insert(&mut self, iri: &Iri) -> Self::Iri {
		V::insert(*self, iri)
	}

	fn insert_owned(&mut self, iri: IriBuf) -> Self::Iri {
		V::insert_owned(*self, iri)
	}
}

impl<'a, V: IriVocabularyMut> EmbedIntoVocabulary<V> for &'a Iri {
	type Embedded = V::Iri;

	fn embed_into_vocabulary(self, vocabulary: &mut V) -> Self::Embedded {
		vocabulary.insert(self)
	}
}

impl<V: IriVocabularyMut> EmbedIntoVocabulary<V> for IriBuf {
	type Embedded = V::Iri;

	fn embed_into_vocabulary(self, vocabulary: &mut V) -> Self::Embedded {
		vocabulary.insert_owned(self)
	}
}

impl<'a, V: IriVocabularyMut> EmbeddedIntoVocabulary<V> for &'a Iri {
	type Embedded = V::Iri;

	fn embedded_into_vocabulary(&self, vocabulary: &mut V) -> Self::Embedded {
		vocabulary.insert(self)
	}
}

impl<V: IriVocabularyMut> EmbeddedIntoVocabulary<V> for IriBuf {
	type Embedded = V::Iri;

	fn embedded_into_vocabulary(&self, vocabulary: &mut V) -> Self::Embedded {
		vocabulary.insert(self.as_iri())
	}
}
