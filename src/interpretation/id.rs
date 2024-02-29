use iref::Iri;

use crate::{
	vocabulary::{BlankIdVocabulary, BlankIdVocabularyMut, IriVocabulary, IriVocabularyMut},
	BlankId, Id,
};

use super::{
	BlankIdInterpretation, BlankIdInterpretationMut, IriInterpretation, IriInterpretationMut,
	ReverseBlankIdInterpretation, ReverseBlankIdInterpretationMut, ReverseIriInterpretation,
	ReverseIriInterpretationMut,
};

/// Node identifier interpretation.
pub trait IdInterpretation<I, B>: IriInterpretation<I> + BlankIdInterpretation<B> {
	/// Returns the interpretation of the given node identifier, if any.
	fn id_interpretation(&self, id: &Id<I, B>) -> Option<Self::Resource> {
		match id {
			Id::Iri(i) => self.iri_interpretation(i),
			Id::Blank(b) => self.blank_id_interpretation(b),
		}
	}

	fn lexical_id_interpretation(
		&self,
		vocabulary: &(impl IriVocabulary<Iri = I> + BlankIdVocabulary<BlankId = B>),
		id: Id<&Iri, &BlankId>,
	) -> Option<Self::Resource> {
		match id {
			Id::Iri(i) => self.lexical_iri_interpretation(vocabulary, i),
			Id::Blank(b) => self.lexical_blank_id_interpretation(vocabulary, b),
		}
	}
}

impl<I, B, T: IriInterpretation<I> + BlankIdInterpretation<B>> IdInterpretation<I, B> for T {}

/// Node identifier interpretation.
pub trait IdInterpretationMut<I, B>: IriInterpretationMut<I> + BlankIdInterpretationMut<B> {
	/// Interprets the given identifier.
	fn interpret_id(&mut self, id: Id<I, B>) -> Self::Resource {
		match id {
			Id::Iri(i) => self.interpret_iri(i),
			Id::Blank(b) => self.interpret_blank_id(b),
		}
	}

	fn interpret_lexical_id(
		&mut self,
		vocabulary: &mut (impl IriVocabularyMut<Iri = I> + BlankIdVocabularyMut<BlankId = B>),
		id: Id<&Iri, &BlankId>,
	) -> Self::Resource {
		match id {
			Id::Iri(i) => self.interpret_lexical_iri(vocabulary, i),
			Id::Blank(b) => self.interpret_lexical_blank_id(vocabulary, b),
		}
	}

	fn interpret_owned_lexical_id(
		&mut self,
		vocabulary: &mut (impl IriVocabularyMut<Iri = I> + BlankIdVocabularyMut<BlankId = B>),
		id: Id,
	) -> Self::Resource {
		match id {
			Id::Iri(i) => self.interpret_owned_lexical_iri(vocabulary, i),
			Id::Blank(b) => self.interpret_owned_lexical_blank_id(vocabulary, b),
		}
	}
}

impl<I, B, T: IriInterpretationMut<I> + BlankIdInterpretationMut<B>> IdInterpretationMut<I, B>
	for T
{
}

/// Reverse node identifier interpretation.
///
/// Used to retrieve the node identifiers of a given resource.
pub trait ReverseIdInterpretation: ReverseIriInterpretation + ReverseBlankIdInterpretation {
	fn ids_of<'a>(&'a self, id: &'a Self::Resource) -> IdsOf<'a, Self> {
		IdsOf {
			iris: self.iris_of(id),
			blanks: self.blank_ids_of(id),
		}
	}
}

impl<I: ?Sized + ReverseIriInterpretation + ReverseBlankIdInterpretation> ReverseIdInterpretation
	for I
{
}

pub struct IdsOf<'a, I: 'a + ?Sized + ReverseIdInterpretation> {
	iris: I::Iris<'a>,
	blanks: I::BlankIds<'a>,
}

impl<'a, I: 'a + ?Sized + ReverseIdInterpretation> Clone for IdsOf<'a, I> {
	fn clone(&self) -> Self {
		Self {
			iris: self.iris.clone(),
			blanks: self.blanks.clone(),
		}
	}
}

impl<'a, I: 'a + ?Sized + ReverseIdInterpretation> Copy for IdsOf<'a, I>
where
	I::Iris<'a>: Copy,
	I::BlankIds<'a>: Copy,
{
}

impl<'a, I: 'a + ?Sized + ReverseIdInterpretation> Iterator for IdsOf<'a, I> {
	type Item = Id<&'a I::Iri, &'a I::BlankId>;

	fn next(&mut self) -> Option<Self::Item> {
		self.iris
			.next()
			.map(Id::Iri)
			.or_else(|| self.blanks.next().map(Id::Blank))
	}
}

pub trait ReverseIdInterpretationMut:
	ReverseIriInterpretationMut + ReverseBlankIdInterpretationMut
{
	fn assign_id(&mut self, r: &Self::Resource, id: Id<Self::Iri, Self::BlankId>) -> bool {
		match id {
			Id::Iri(i) => self.assign_iri(r, i),
			Id::Blank(b) => self.assign_blank_id(r, b),
		}
	}
}

impl<I: ?Sized + ReverseIriInterpretationMut + ReverseBlankIdInterpretationMut>
	ReverseIdInterpretationMut for I
{
}
