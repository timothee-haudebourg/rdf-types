use crate::{Literal, Term};

/// Types that may represent a literal value.
pub trait MaybeLiteral {
	/// Inner literal type.
	type Literal;
}

impl<I> MaybeLiteral for Literal<I> {
	type Literal = Self;
}

impl<I, L> MaybeLiteral for Term<I, L> {
	type Literal = L;
}

/// Types that may have a literal representation that can be borrowed.
pub trait TryAsLiteral: MaybeLiteral {
	/// Returns a reference to the literal value, if any.
	fn try_as_literal(&self) -> Option<&Self::Literal>;

	fn is_literal(&self) -> bool {
		self.try_as_literal().is_some()
	}
}

impl<I, L> TryAsLiteral for Term<I, L> {
	fn try_as_literal(&self) -> Option<&L> {
		self.as_literal()
	}
}

/// Types that can be turned into a literal.
pub trait TryIntoLiteral: MaybeLiteral + Sized {
	fn try_into_literal(self) -> Result<Self::Literal, Self>;
}

impl<I, L> TryIntoLiteral for Term<I, L> {
	fn try_into_literal(self) -> Result<L, Self> {
		self.try_into_literal().map_err(Self::Id)
	}
}

/// Types that can be constructed from a literal.
pub trait FromLiteral: MaybeLiteral {
	/// Builds a value from a literal.
	fn from_literal(l: Self::Literal) -> Self;
}

impl<I, L> FromLiteral for Term<I, L> {
	fn from_literal(l: L) -> Self {
		Self::Literal(l)
	}
}
