use std::ops::Deref;

use crate::{Quad, Triple};

pub mod quad;
pub mod triple;

use into_owned_trait::IntoOwned;
pub use quad::CanonicalQuadPattern;
pub use triple::{CanonicalTriplePattern, TriplePatternMap};

/// Triple pattern.
pub type TriplePattern<T, X> = Triple<Pattern<T, X>>;

/// Triple pattern.
pub type QuadPattern<T, X> = Quad<Pattern<T, X>>;

/// Resource or variable.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum Pattern<T, X> {
	Ground(T),
	Var(X),
}

impl<T, X> Pattern<T, X> {
	pub fn as_ref(&self) -> Pattern<&T, &X> {
		match self {
			Self::Ground(t) => Pattern::Ground(t),
			Self::Var(x) => Pattern::Var(x),
		}
	}

	pub fn as_deref(&self) -> Pattern<&T::Target, &X::Target>
	where
		T: Deref,
		X: Deref,
	{
		match self {
			Self::Ground(t) => Pattern::Ground(t),
			Self::Var(x) => Pattern::Var(x),
		}
	}

	/// Checks if this pattern is a ground value (as opposed to a variable).
	pub fn is_ground(&self) -> bool {
		matches!(self, Self::Ground(_))
	}

	pub fn is_ground_or(&self, f: impl FnOnce(&T) -> bool) -> bool {
		match self {
			Self::Ground(t) => f(t),
			Self::Var(_) => false,
		}
	}

	pub fn is_var(&self) -> bool {
		matches!(self, Self::Var(_))
	}

	pub fn is_var_or(&self, f: impl FnOnce(&X) -> bool) -> bool {
		match self {
			Self::Ground(_) => true,
			Self::Var(x) => f(x),
		}
	}

	pub fn map<U>(self, f: impl Fn(T) -> U) -> Pattern<U, X> {
		match self {
			Self::Ground(t) => Pattern::Ground(f(t)),
			Self::Var(x) => Pattern::Var(x),
		}
	}
}

impl<T, X> From<T> for Pattern<T, X> {
	fn from(value: T) -> Self {
		Self::Ground(value)
	}
}

impl<T, X> IntoOwned for Pattern<T, X>
where
	T: IntoOwned,
	X: IntoOwned,
{
	type Owned = Pattern<T::Owned, X::Owned>;

	fn into_owned(self) -> Self::Owned {
		match self {
			Self::Ground(t) => Pattern::Ground(t.into_owned()),
			Self::Var(x) => Pattern::Var(x.into_owned()),
		}
	}
}

impl<T, X> AsPattern for Pattern<T, X> {
	type Ground = T;
	type Var = X;

	fn as_pattern(&self) -> Pattern<&T, &X> {
		self.as_ref()
	}
}

pub trait AsPattern {
	type Ground;
	type Var;

	fn as_pattern(&self) -> Pattern<&Self::Ground, &Self::Var>;

	fn is_ground(&self) -> bool {
		self.as_pattern().is_ground()
	}

	fn is_var(&self) -> bool {
		self.as_pattern().is_var()
	}
}
