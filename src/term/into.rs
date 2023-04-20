use crate::Term;

/// Type that can be turned into a [`Term`].
pub trait IntoTerm {
	/// Node identifier type.
	type Id;

	/// Literal type.
	type Literal;

	/// Turns the value into a [`Term`].
	fn into_term(self) -> Term<Self::Id, Self::Literal>;
}

impl<I, L> IntoTerm for Term<I, L> {
	type Id = I;

	type Literal = L;

	fn into_term(self) -> Term<I, L> {
		self
	}
}
