use crate::{
	literal,
	vocabulary::{IriVocabularyMut, LiteralVocabulary, LiteralVocabularyMut},
	Interpretation, Literal, LiteralRef,
};

/// Literal value interpretation.
pub trait LiteralInterpretation<L>: Interpretation {
	/// Returns the interpretation of the given literal value, if any.
	fn literal_interpretation(&self, literal: &L) -> Option<Self::Resource>;

	fn lexical_literal_interpretation<V: LiteralVocabulary<Literal = L>>(
		&self,
		vocabulary: &V,
		literal: LiteralRef<V::Iri>,
	) -> Option<Self::Resource> {
		vocabulary
			.get_literal(literal)
			.and_then(|l| self.literal_interpretation(&l))
	}
}

impl<'t, L, T: LiteralInterpretation<L>> LiteralInterpretation<L> for &'t T {
	fn literal_interpretation(&self, literal: &L) -> Option<Self::Resource> {
		T::literal_interpretation(*self, literal)
	}

	fn lexical_literal_interpretation<V: LiteralVocabulary<Literal = L>>(
		&self,
		vocabulary: &V,
		literal: LiteralRef<V::Iri>,
	) -> Option<Self::Resource> {
		T::lexical_literal_interpretation(*self, vocabulary, literal)
	}
}

impl<'t, L, T: LiteralInterpretation<L>> LiteralInterpretation<L> for &'t mut T {
	fn literal_interpretation(&self, literal: &L) -> Option<Self::Resource> {
		T::literal_interpretation(*self, literal)
	}

	fn lexical_literal_interpretation<V: LiteralVocabulary<Literal = L>>(
		&self,
		vocabulary: &V,
		literal: LiteralRef<V::Iri>,
	) -> Option<Self::Resource> {
		T::lexical_literal_interpretation(*self, vocabulary, literal)
	}
}

/// Mutable literal value interpretation.
pub trait LiteralInterpretationMut<L = Literal>: Interpretation {
	/// Interprets the given literal value.
	fn interpret_literal(&mut self, literal: L) -> Self::Resource;

	fn interpret_lexical_literal<V: LiteralVocabularyMut<Literal = L>>(
		&mut self,
		vocabulary: &mut V,
		literal: LiteralRef<V::Iri>,
	) -> Self::Resource {
		self.interpret_literal(vocabulary.insert_literal(literal))
	}

	fn interpret_owned_lexical_literal<V: LiteralVocabularyMut<Literal = L>>(
		&mut self,
		vocabulary: &mut V,
		literal: Literal<V::Iri>,
	) -> Self::Resource {
		self.interpret_literal(vocabulary.insert_owned_literal(literal))
	}

	fn interpret_full_lexical_literal(
		&mut self,
		vocabulary: &mut (impl IriVocabularyMut + LiteralVocabularyMut<Literal = L>),
		literal: Literal,
	) -> Self::Resource {
		let (value, type_) = literal.into_parts();
		let type_ = match type_ {
			literal::LiteralType::Any(ty) => literal::LiteralType::Any(vocabulary.insert_owned(ty)),
			literal::LiteralType::LangString(tag) => literal::LiteralType::LangString(tag),
		};

		self.interpret_literal(vocabulary.insert_owned_literal(Literal::new(value, type_)))
	}
}

pub trait ReverseLiteralInterpretation: Interpretation {
	type Literal;

	type Literals<'a>: Clone + Iterator<Item = &'a Self::Literal>
	where
		Self: 'a;

	fn literals_of<'a>(&'a self, id: &'a Self::Resource) -> Self::Literals<'a>;
}

impl<'t, T: ReverseLiteralInterpretation> ReverseLiteralInterpretation for &'t T {
	type Literal = T::Literal;
	type Literals<'a> = T::Literals<'a> where Self: 'a;

	fn literals_of<'a>(&'a self, id: &'a Self::Resource) -> Self::Literals<'a> {
		T::literals_of(*self, id)
	}
}

impl<'t, T: ReverseLiteralInterpretation> ReverseLiteralInterpretation for &'t mut T {
	type Literal = T::Literal;
	type Literals<'a> = T::Literals<'a> where Self: 'a;

	fn literals_of<'a>(&'a self, id: &'a Self::Resource) -> Self::Literals<'a> {
		T::literals_of(*self, id)
	}
}
