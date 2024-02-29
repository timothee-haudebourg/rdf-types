use iref::{Iri, IriBuf};

use crate::vocabulary::{IriVocabulary, IriVocabularyMut};

use super::Interpretation;

/// IRI Interpretation.
pub trait IriInterpretation<I: ?Sized>: Interpretation {
	/// Returns the interpretation of the given IRI, if any.
	fn iri_interpretation(&self, iri: &I) -> Option<Self::Resource>;

	fn lexical_iri_interpretation(
		&self,
		vocabulary: &impl IriVocabulary<Iri = I>,
		iri: &Iri,
	) -> Option<Self::Resource>
	where
		I: Sized,
	{
		vocabulary
			.get(iri)
			.and_then(|iri| self.iri_interpretation(&iri))
	}
}

impl<'t, I, T: IriInterpretation<I>> IriInterpretation<I> for &'t T {
	fn iri_interpretation(&self, iri: &I) -> Option<Self::Resource> {
		T::iri_interpretation(*self, iri)
	}
}

impl<'t, I, T: IriInterpretation<I>> IriInterpretation<I> for &'t mut T {
	fn iri_interpretation(&self, iri: &I) -> Option<Self::Resource> {
		T::iri_interpretation(*self, iri)
	}
}

/// Mutable IRI interpretation.
pub trait IriInterpretationMut<I = IriBuf>: Interpretation {
	/// Interprets the given IRI.
	fn interpret_iri(&mut self, iri: I) -> Self::Resource;

	fn interpret_lexical_iri(
		&mut self,
		vocabulary: &mut impl IriVocabularyMut<Iri = I>,
		iri: &Iri,
	) -> Self::Resource {
		self.interpret_iri(vocabulary.insert(iri))
	}

	fn interpret_owned_lexical_iri(
		&mut self,
		vocabulary: &mut impl IriVocabularyMut<Iri = I>,
		iri: IriBuf,
	) -> Self::Resource {
		self.interpret_iri(vocabulary.insert_owned(iri))
	}
}

impl<'t, I, T: IriInterpretationMut<I>> IriInterpretationMut<I> for &'t mut T {
	fn interpret_iri(&mut self, iri: I) -> Self::Resource {
		T::interpret_iri(*self, iri)
	}

	fn interpret_lexical_iri(
		&mut self,
		vocabulary: &mut impl IriVocabularyMut<Iri = I>,
		iri: &Iri,
	) -> Self::Resource {
		T::interpret_lexical_iri(*self, vocabulary, iri)
	}

	fn interpret_owned_lexical_iri(
		&mut self,
		vocabulary: &mut impl IriVocabularyMut<Iri = I>,
		iri: IriBuf,
	) -> Self::Resource {
		T::interpret_owned_lexical_iri(*self, vocabulary, iri)
	}
}

pub trait ReverseIriInterpretation: Interpretation {
	type Iri;
	type Iris<'a>: Clone + Iterator<Item = &'a Self::Iri>
	where
		Self: 'a;

	fn iris_of<'a>(&'a self, id: &'a Self::Resource) -> Self::Iris<'a>;
}

impl<'t, T: ReverseIriInterpretation> ReverseIriInterpretation for &'t T {
	type Iri = T::Iri;
	type Iris<'a> = T::Iris<'a> where Self: 'a;

	fn iris_of<'a>(&'a self, id: &'a Self::Resource) -> Self::Iris<'a> {
		T::iris_of(*self, id)
	}
}

impl<'t, T: ReverseIriInterpretation> ReverseIriInterpretation for &'t mut T {
	type Iri = T::Iri;
	type Iris<'a> = T::Iris<'a> where Self: 'a;

	fn iris_of<'a>(&'a self, id: &'a Self::Resource) -> Self::Iris<'a> {
		T::iris_of(*self, id)
	}
}

pub trait ReverseIriInterpretationMut: ReverseIriInterpretation {
	fn assign_iri(&mut self, id: &Self::Resource, iri: Self::Iri) -> bool;
}

impl<'t, T: ReverseIriInterpretationMut> ReverseIriInterpretationMut for &'t mut T {
	fn assign_iri(&mut self, id: &Self::Resource, iri: Self::Iri) -> bool {
		T::assign_iri(*self, id, iri)
	}
}
