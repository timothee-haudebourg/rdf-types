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
