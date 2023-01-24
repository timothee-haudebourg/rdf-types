use std::collections::HashMap;

use iref::Iri;

use crate::{
	BlankId, BlankIdBuf, BlankIdVocabulary, BlankIdVocabularyMut, IriVocabulary, IriVocabularyMut,
	Vocabulary,
};

/// Vocabulary wrapper that helps avoid blank id collisions.
///
/// This is a wrapper behind a vocabulary that can be used to prevent collisions
/// between blank node identifiers added from different sources.
/// Upon insertion, blank node identifier will be prepended by a prefix
/// representing the "scope" of the blank node identifier.
///
/// # Example
///
/// Take the situation where we have to graphs locally defining the
/// blank node `_:0`. We want to insert both identifiers using
/// [`BlankIdVocabularyMut::insert_blank_id`], however doing so would provoke a
/// collision this both blank identifiers are lexically equals, even if
/// representing different nodes. This wrapper can be used to rename the blank
/// nodes upon insertion in the vocabulary to avoid collision by adding a
/// "scope" prefix.
///
/// ```
/// use rdf_types::{BlankIdBuf, BlankIdVocabulary, BlankIdVocabularyMut};
/// use rdf_types::vocabulary::{Index, IndexVocabulary, Scoped};
///
/// let mut vocab: IndexVocabulary = IndexVocabulary::new();
///
/// let a = BlankIdBuf::from_suffix("0").unwrap();
/// let b = BlankIdBuf::from_suffix("0").unwrap();
///
/// // Without `Scoped` both identifiers are associated to the same
/// // index, they are equals.
/// let a_index = vocab.insert_blank_id(&a);
/// let b_index = vocab.insert_blank_id(&b);
/// assert_eq!(a_index, b_index);
///
/// // Using `Scoped` we can separate both identifiers.
/// let mut scoped_vocab = Scoped::new(&mut vocab, "a_scope");
/// let a_scoped_index = scoped_vocab.insert_blank_id(&a);
/// let mut scoped_vocab = Scoped::new(&mut vocab, "b_scope");
/// let b_scoped_index = scoped_vocab.insert_blank_id(&b);
/// assert_ne!(a_scoped_index, b_scoped_index);
///
/// // Underneath, they are added in the wrapped vocabulary with a prefix
/// // unique to their respective scope.
/// assert_eq!(vocab.blank_id(&a_scoped_index).unwrap(), "_:a_scope:0");
/// assert_eq!(vocab.blank_id(&b_scoped_index).unwrap(), "_:b_scope:0");
/// ```
pub struct Scoped<'a, V: BlankIdVocabulary, S> {
	scope: S,
	map: HashMap<BlankIdBuf, V::BlankId>,
	pub(crate) inner: &'a mut V,
}

impl<'a, V: BlankIdVocabulary, S> Scoped<'a, V, S> {
	/// Create a new wrapper around `vocabulary` with the given `scope`.
	///
	/// The `scope` must implement [`fmt::Display`] so it can be prepended to
	/// any blank node identifier added through this wrapper.
	pub fn new(vocabulary: &'a mut V, scope: S) -> Self {
		Self {
			scope,
			map: HashMap::new(),
			inner: vocabulary,
		}
	}
}

impl<'a, V: Vocabulary, S> IriVocabulary for Scoped<'a, V, S> {
	type Iri = V::Iri;

	fn iri<'i>(&'i self, id: &'i Self::Iri) -> Option<Iri<'i>> {
		self.inner.iri(id)
	}

	fn get(&self, iri: Iri) -> Option<Self::Iri> {
		self.inner.get(iri)
	}
}

impl<'a, V: IriVocabularyMut + BlankIdVocabulary, S> IriVocabularyMut for Scoped<'a, V, S> {
	fn insert(&mut self, iri: Iri) -> Self::Iri {
		self.inner.insert(iri)
	}
}

impl<'a, V: BlankIdVocabulary, S> BlankIdVocabulary for Scoped<'a, V, S>
where
	V::BlankId: Clone,
{
	type BlankId = V::BlankId;

	fn blank_id<'b>(&'b self, id: &'b Self::BlankId) -> Option<&'b BlankId> {
		self.inner.blank_id(id)
	}

	fn get_blank_id(&self, id: &BlankId) -> Option<Self::BlankId> {
		self.map.get(id).cloned()
	}
}

impl<'a, V: BlankIdVocabularyMut, S: std::fmt::Display> BlankIdVocabularyMut for Scoped<'a, V, S>
where
	V::BlankId: Clone,
{
	fn insert_blank_id(&mut self, id: &BlankId) -> Self::BlankId {
		match self.get_blank_id(id) {
			Some(id) => id,
			None => {
				let scoped =
					BlankIdBuf::from_suffix(&format!("{}:{}", self.scope, id.suffix())).unwrap();
				let i = self.inner.insert_blank_id(&scoped);
				self.map.insert(id.to_owned(), i.clone());
				i
			}
		}
	}
}
