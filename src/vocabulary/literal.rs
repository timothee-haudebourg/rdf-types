use super::IriVocabulary;
use crate::{Literal, LiteralRef};

/// Literal value vocabulary.
pub trait LiteralVocabulary: IriVocabulary {
	/// Literal identifier type.
	type Literal;

	fn literal<'l>(&'l self, id: &'l Self::Literal) -> Option<LiteralRef<'l, Self::Iri>>;

	fn owned_literal(&self, id: Self::Literal) -> Result<Literal<Self::Iri>, Self::Literal>;

	/// Returns the vocabulary id of the given literal identifier, if any.
	fn get_literal(&self, id: LiteralRef<Self::Iri>) -> Option<Self::Literal>;
}

impl<'a, V: LiteralVocabulary> LiteralVocabulary for &'a V {
	type Literal = V::Literal;

	fn literal<'l>(&'l self, id: &'l Self::Literal) -> Option<LiteralRef<'l, V::Iri>> {
		V::literal(*self, id)
	}

	fn owned_literal(&self, id: Self::Literal) -> Result<Literal<V::Iri>, Self::Literal> {
		V::owned_literal(*self, id)
	}

	fn get_literal(&self, id: LiteralRef<Self::Iri>) -> Option<Self::Literal> {
		V::get_literal(*self, id)
	}
}

impl<'a, V: LiteralVocabulary> LiteralVocabulary for &'a mut V {
	type Literal = V::Literal;

	fn literal<'l>(&'l self, id: &'l Self::Literal) -> Option<LiteralRef<'l, V::Iri>> {
		V::literal(*self, id)
	}

	fn owned_literal(&self, id: Self::Literal) -> Result<Literal<V::Iri>, Self::Literal> {
		V::owned_literal(*self, id)
	}

	fn get_literal(&self, id: LiteralRef<Self::Iri>) -> Option<Self::Literal> {
		V::get_literal(*self, id)
	}
}

/// Mutable literal value vocabulary.
pub trait LiteralVocabularyMut: LiteralVocabulary {
	fn insert_literal(&mut self, value: LiteralRef<Self::Iri>) -> Self::Literal;

	fn insert_owned_literal(&mut self, value: Literal<Self::Iri>) -> Self::Literal {
		self.insert_literal(value.as_ref())
	}
}

impl<'a, V: LiteralVocabularyMut> LiteralVocabularyMut for &'a mut V {
	fn insert_literal(&mut self, value: LiteralRef<V::Iri>) -> Self::Literal {
		V::insert_literal(*self, value)
	}

	fn insert_owned_literal(&mut self, value: Literal<V::Iri>) -> Self::Literal {
		V::insert_owned_literal(*self, value)
	}
}
