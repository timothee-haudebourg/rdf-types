use iref::IriBuf;

use crate::{Id, Quad, Term, Triple};

/// Map RDF literal values.
pub trait MapLiteral<T, U> {
	type Output;

	/// Maps RDF literal values.
	fn map_literal(self, f: impl FnMut(T) -> U) -> Self::Output;
}

impl<T, U> MapLiteral<T, U> for IriBuf {
	type Output = Self;

	fn map_literal(self, _f: impl FnMut(T) -> U) -> Self::Output {
		self
	}
}

impl<I, B, T, U> MapLiteral<T, U> for Id<I, B> {
	type Output = Self;

	fn map_literal(self, _f: impl FnMut(T) -> U) -> Self::Output {
		self
	}
}

impl<V: MapLiteral<T, U>, T, U> MapLiteral<T, U> for Option<V> {
	type Output = Option<V::Output>;

	fn map_literal(self, f: impl FnMut(T) -> U) -> Self::Output {
		self.map(|v| v.map_literal(f))
	}
}

impl<T: MapLiteral<L, M>, L, M> MapLiteral<L, M> for Vec<T> {
	type Output = Vec<T::Output>;

	fn map_literal(self, mut f: impl FnMut(L) -> M) -> Self::Output {
		self.into_iter().map(|t| t.map_literal(&mut f)).collect()
	}
}

impl<I, T, U> MapLiteral<T, U> for Term<I, T> {
	type Output = Term<I, U>;

	fn map_literal(self, mut f: impl FnMut(T) -> U) -> Self::Output {
		match self {
			Self::Id(id) => Term::Id(id),
			Self::Literal(l) => Term::Literal(f(l)),
		}
	}
}

impl<S: MapLiteral<T, U>, P: MapLiteral<T, U>, O: MapLiteral<T, U>, G: MapLiteral<T, U>, T, U>
	MapLiteral<T, U> for Quad<S, P, O, G>
{
	type Output = Quad<S::Output, P::Output, O::Output, G::Output>;

	fn map_literal(self, mut f: impl FnMut(T) -> U) -> Self::Output {
		Quad(
			self.0.map_literal(&mut f),
			self.1.map_literal(&mut f),
			self.2.map_literal(&mut f),
			self.3.map_literal(f),
		)
	}
}

impl<S: MapLiteral<T, U>, P: MapLiteral<T, U>, O: MapLiteral<T, U>, T, U> MapLiteral<T, U>
	for Triple<S, P, O>
{
	type Output = Triple<S::Output, P::Output, O::Output>;

	fn map_literal(self, mut f: impl FnMut(T) -> U) -> Self::Output {
		Triple(
			self.0.map_literal(&mut f),
			self.1.map_literal(&mut f),
			self.2.map_literal(f),
		)
	}
}
