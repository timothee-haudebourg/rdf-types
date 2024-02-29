use crate::{
	vocabulary::{BlankIdVocabulary, BlankIdVocabularyMut},
	BlankId, BlankIdBuf, Interpretation,
};

/// Blank node identifier interpretation.
pub trait BlankIdInterpretation<B: ?Sized>: Interpretation {
	/// Returns the interpretation of the given blank node identifier, if any.
	fn blank_id_interpretation(&self, blank_id: &B) -> Option<Self::Resource>;

	fn lexical_blank_id_interpretation(
		&self,
		vocabulary: &impl BlankIdVocabulary<BlankId = B>,
		blank_id: &BlankId,
	) -> Option<Self::Resource>
	where
		B: Sized,
	{
		vocabulary
			.get_blank_id(blank_id)
			.and_then(|blank_id| self.blank_id_interpretation(&blank_id))
	}
}

impl<'t, B, T: BlankIdInterpretation<B>> BlankIdInterpretation<B> for &'t T {
	fn blank_id_interpretation(&self, blank_id: &B) -> Option<Self::Resource> {
		T::blank_id_interpretation(*self, blank_id)
	}

	fn lexical_blank_id_interpretation(
		&self,
		vocabulary: &impl BlankIdVocabulary<BlankId = B>,
		blank_id: &BlankId,
	) -> Option<Self::Resource>
	where
		B: Sized,
	{
		T::lexical_blank_id_interpretation(*self, vocabulary, blank_id)
	}
}

impl<'t, B, T: BlankIdInterpretation<B>> BlankIdInterpretation<B> for &'t mut T {
	fn blank_id_interpretation(&self, blank_id: &B) -> Option<Self::Resource> {
		T::blank_id_interpretation(*self, blank_id)
	}

	fn lexical_blank_id_interpretation(
		&self,
		vocabulary: &impl BlankIdVocabulary<BlankId = B>,
		blank_id: &BlankId,
	) -> Option<Self::Resource>
	where
		B: Sized,
	{
		T::lexical_blank_id_interpretation(*self, vocabulary, blank_id)
	}
}

/// Blank node identifier interpretation.
pub trait BlankIdInterpretationMut<B = BlankIdBuf>: Interpretation {
	/// Interprets the given blank node identifier.
	fn interpret_blank_id(&mut self, blank_id: B) -> Self::Resource;

	fn interpret_lexical_blank_id(
		&mut self,
		vocabulary: &mut impl BlankIdVocabularyMut<BlankId = B>,
		blank_id: &BlankId,
	) -> Self::Resource {
		self.interpret_blank_id(vocabulary.insert_blank_id(blank_id))
	}

	fn interpret_owned_lexical_blank_id(
		&mut self,
		vocabulary: &mut impl BlankIdVocabularyMut<BlankId = B>,
		blank_id: BlankIdBuf,
	) -> Self::Resource {
		self.interpret_blank_id(vocabulary.insert_owned_blank_id(blank_id))
	}
}

impl<'t, B, T: BlankIdInterpretationMut<B>> BlankIdInterpretationMut<B> for &'t mut T {
	fn interpret_blank_id(&mut self, blank_id: B) -> Self::Resource {
		T::interpret_blank_id(*self, blank_id)
	}

	fn interpret_lexical_blank_id(
		&mut self,
		vocabulary: &mut impl BlankIdVocabularyMut<BlankId = B>,
		blank_id: &BlankId,
	) -> Self::Resource {
		T::interpret_lexical_blank_id(*self, vocabulary, blank_id)
	}

	fn interpret_owned_lexical_blank_id(
		&mut self,
		vocabulary: &mut impl BlankIdVocabularyMut<BlankId = B>,
		blank_id: BlankIdBuf,
	) -> Self::Resource {
		T::interpret_owned_lexical_blank_id(*self, vocabulary, blank_id)
	}
}

pub trait ReverseBlankIdInterpretation: Interpretation {
	type BlankId;
	type BlankIds<'a>: Clone + Iterator<Item = &'a Self::BlankId>
	where
		Self: 'a;

	fn blank_ids_of<'a>(&'a self, id: &'a Self::Resource) -> Self::BlankIds<'a>;
}

impl<'t, T: ReverseBlankIdInterpretation> ReverseBlankIdInterpretation for &'t T {
	type BlankId = T::BlankId;
	type BlankIds<'a> = T::BlankIds<'a> where Self: 'a;

	fn blank_ids_of<'a>(&'a self, id: &'a Self::Resource) -> Self::BlankIds<'a> {
		T::blank_ids_of(*self, id)
	}
}

impl<'t, T: ReverseBlankIdInterpretation> ReverseBlankIdInterpretation for &'t mut T {
	type BlankId = T::BlankId;
	type BlankIds<'a> = T::BlankIds<'a> where Self: 'a;

	fn blank_ids_of<'a>(&'a self, id: &'a Self::Resource) -> Self::BlankIds<'a> {
		T::blank_ids_of(*self, id)
	}
}

pub trait ReverseBlankIdInterpretationMut: ReverseBlankIdInterpretation {
	fn assign_blank_id(&mut self, id: &Self::Resource, blank_id: Self::BlankId) -> bool;
}

impl<'t, T: ReverseBlankIdInterpretationMut> ReverseBlankIdInterpretationMut for &'t mut T {
	fn assign_blank_id(&mut self, id: &Self::Resource, blank_id: Self::BlankId) -> bool {
		T::assign_blank_id(*self, id, blank_id)
	}
}
