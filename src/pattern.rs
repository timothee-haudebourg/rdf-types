//! Triple/quad patterns, used to describe a set of triples/quads by leaving
//! some of their components unspecified (variables).
use std::ops::Deref;

use into_owned_trait::IntoOwned;

use crate::{Quad, Triple};

/// Triple pattern, with each component either a ground resource or a
/// variable.
pub type TriplePattern<T, X> = Triple<Pattern<T, X>>;

/// Quad pattern, with each component either a ground resource or a
/// variable.
pub type QuadPattern<T, X> = Quad<Pattern<T, X>>;

/// Linear triple pattern, with each component either a ground resource
/// (`Some`) or unconstrained (`None`).
///
/// Unlike [`TriplePattern`], a linear pattern cannot bind the same variable
/// to more than one component, hence the name: it does not encode a general
/// [`Pattern`] graph, only affine constraints on each component
/// independently.
pub type LinearTriplePattern<T> = Triple<Option<T>>;

impl<T> From<Triple<T>> for LinearTriplePattern<T> {
	/// Turns a triple into the linear pattern matching only that triple.
	fn from(value: Triple<T>) -> Self {
		value.map(Some)
	}
}

/// Linear quad pattern, with each component either a ground resource
/// (`Some`) or unconstrained (`None`).
///
/// See [`LinearTriplePattern`] for details on what makes a pattern "linear".
pub type LinearQuadPattern<T> = Quad<Option<T>>;

impl<T> From<Quad<T>> for LinearQuadPattern<T> {
	/// Turns a quad into the linear pattern matching only that quad.
	fn from(value: Quad<T>) -> Self {
		value.map(Some)
	}
}

/// Resource or variable.
///
/// Used as a triple/quad component to represent either a fixed
/// (`Ground`) resource, or a `Var`iable that may be substituted for any
/// resource.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Pattern<T, X> {
	/// Ground (fixed) resource.
	Ground(T),

	/// Variable, matching any resource.
	Var(X),
}

impl<T, X> Pattern<T, X> {
	/// Borrows the ground value or variable of this pattern.
	pub fn as_ref(&self) -> Pattern<&T, &X> {
		match self {
			Self::Ground(t) => Pattern::Ground(t),
			Self::Var(x) => Pattern::Var(x),
		}
	}

	/// Dereferences the ground value or variable of this pattern.
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

	/// Returns `true` if this pattern is a ground value satisfying the given
	/// predicate, and `false` if it is a variable.
	pub fn is_ground_and(&self, f: impl FnOnce(&T) -> bool) -> bool {
		match self {
			Self::Ground(t) => f(t),
			Self::Var(_) => false,
		}
	}

	/// Checks if this pattern is a variable (as opposed to a ground value).
	pub fn is_var(&self) -> bool {
		matches!(self, Self::Var(_))
	}

	/// Returns `true` if this pattern is a ground value, or if it is a
	/// variable satisfying the given predicate.
	///
	/// Mirrors [`Option::is_none_or`], with [`Self::Ground`] playing the role
	/// of [`None`].
	pub fn is_ground_or(&self, f: impl FnOnce(&X) -> bool) -> bool {
		match self {
			Self::Ground(_) => true,
			Self::Var(x) => f(x),
		}
	}

	/// Maps the ground value with the given function, leaving a variable
	/// untouched.
	pub fn map<U>(self, f: impl FnOnce(T) -> U) -> Pattern<U, X> {
		match self {
			Self::Ground(t) => Pattern::Ground(f(t)),
			Self::Var(x) => Pattern::Var(x),
		}
	}

	/// Maps the variable with the given function, leaving a ground value
	/// untouched.
	pub fn map_var<Y>(self, f: impl FnOnce(X) -> Y) -> Pattern<T, Y> {
		match self {
			Self::Ground(t) => Pattern::Ground(t),
			Self::Var(x) => Pattern::Var(f(x)),
		}
	}
}

impl<T, X> From<T> for Pattern<T, X> {
	/// Creates a ground pattern from a resource.
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

/// Value that can be seen as a [`Pattern`], either a ground value or a
/// variable.
///
/// Implemented by [`Pattern`] itself. Resource types implementing this trait
/// can be used with [`crate::find_bijection`] to find a blank
/// node/variable-preserving bijection between two datasets.
pub trait AsPattern {
	/// Ground value type.
	type Ground: ?Sized;

	/// Variable type.
	type Var: ?Sized;

	/// Borrows this value as a [`Pattern`].
	fn as_pattern(&self) -> Pattern<&Self::Ground, &Self::Var>;

	/// Checks if this value is a ground value (as opposed to a variable).
	fn is_ground(&self) -> bool {
		self.as_pattern().is_ground()
	}

	/// Checks if this value is a variable (as opposed to a ground value).
	fn is_var(&self) -> bool {
		self.as_pattern().is_var()
	}
}
