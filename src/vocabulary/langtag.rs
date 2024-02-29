/// language tag vocabulary.
pub trait LanguageTagVocabulary {
	/// Language tag type.
	type LanguageTag;

	fn language_tag<'l>(&'l self, id: &'l Self::LanguageTag) -> Option<LanguageTag<'l>>;

	fn owned_language_tag(
		&self,
		id: Self::LanguageTag,
	) -> Result<LanguageTagBuf, Self::LanguageTag> {
		self.language_tag(&id).map(|t| t.cloned()).ok_or(id)
	}

	/// Returns the vocabulary id of the given language tag identifier, if any.
	fn get_language_tag(&self, id: LanguageTag) -> Option<Self::LanguageTag>;
}

impl<'a, V: LanguageTagVocabulary> LanguageTagVocabulary for &'a V {
	type LanguageTag = V::LanguageTag;

	fn language_tag<'l>(&'l self, id: &'l Self::LanguageTag) -> Option<LanguageTag<'l>> {
		V::language_tag(*self, id)
	}

	fn owned_language_tag(
		&self,
		id: Self::LanguageTag,
	) -> Result<LanguageTagBuf, Self::LanguageTag> {
		V::owned_language_tag(*self, id)
	}

	fn get_language_tag(&self, id: LanguageTag) -> Option<Self::LanguageTag> {
		V::get_language_tag(*self, id)
	}
}

impl<'a, V: LanguageTagVocabulary> LanguageTagVocabulary for &'a mut V {
	type LanguageTag = V::LanguageTag;

	fn language_tag<'l>(&'l self, id: &'l Self::LanguageTag) -> Option<LanguageTag<'l>> {
		V::language_tag(*self, id)
	}

	fn owned_language_tag(
		&self,
		id: Self::LanguageTag,
	) -> Result<LanguageTagBuf, Self::LanguageTag> {
		V::owned_language_tag(*self, id)
	}

	fn get_language_tag(&self, id: LanguageTag) -> Option<Self::LanguageTag> {
		V::get_language_tag(*self, id)
	}
}

/// Mutable literal value vocabulary.
pub trait LanguageTagVocabularyMut: LanguageTagVocabulary {
	fn insert_language_tag(&mut self, value: LanguageTag) -> Self::LanguageTag;

	fn insert_owned_language_tag(&mut self, value: LanguageTagBuf) -> Self::LanguageTag {
		self.insert_language_tag(value.as_ref())
	}
}

impl<'a, V: LanguageTagVocabularyMut> LanguageTagVocabularyMut for &'a mut V {
	fn insert_language_tag(&mut self, value: LanguageTag) -> Self::LanguageTag {
		V::insert_language_tag(*self, value)
	}

	fn insert_owned_language_tag(&mut self, value: LanguageTagBuf) -> Self::LanguageTag {
		V::insert_owned_language_tag(*self, value)
	}
}

impl<'a, V: LanguageTagVocabularyMut> InsertIntoVocabulary<V> for LanguageTag<'a> {
	type Inserted = V::LanguageTag;

	fn insert_into_vocabulary(self, vocabulary: &mut V) -> Self::Inserted {
		vocabulary.insert_language_tag(self)
	}
}

impl<V: LanguageTagVocabularyMut> InsertIntoVocabulary<V> for LanguageTagBuf {
	type Inserted = V::LanguageTag;

	fn insert_into_vocabulary(self, vocabulary: &mut V) -> Self::Inserted {
		vocabulary.insert_owned_language_tag(self)
	}
}

impl<'a, V: LanguageTagVocabularyMut> InsertedIntoVocabulary<V> for LanguageTag<'a> {
	type Inserted = V::LanguageTag;

	fn inserted_into_vocabulary(&self, vocabulary: &mut V) -> Self::Inserted {
		vocabulary.insert_language_tag(*self)
	}
}

impl<V: LanguageTagVocabularyMut> InsertedIntoVocabulary<V> for LanguageTagBuf {
	type Inserted = V::LanguageTag;

	fn inserted_into_vocabulary(&self, vocabulary: &mut V) -> Self::Inserted {
		vocabulary.insert_language_tag(self.as_ref())
	}
}