use std::marker::PhantomData;

use into_owned_trait::IntoOwned;

use crate::{Quad, Triple};

pub struct InfallibleIterator<I>(pub I);

impl<I: Iterator> Iterator for InfallibleIterator<I> {
	type Item = Result<I::Item, std::convert::Infallible>;

	fn next(&mut self) -> Option<Self::Item> {
		self.0.next().map(Ok)
	}
}

pub struct TriplesIntoQuads<I, G>(I, PhantomData<G>);

impl<I, G> TriplesIntoQuads<I, G> {
	pub fn new(inner: I) -> Self {
		Self(inner, PhantomData)
	}
}

impl<S, P, O, G, I: Iterator<Item = Triple<S, P, O>>> Iterator for TriplesIntoQuads<I, G> {
	type Item = Quad<S, P, O, G>;

	fn next(&mut self) -> Option<Self::Item> {
		self.0.next().map(|t| t.into_quad(None))
	}
}

/// Wraps an iterator over `Triple<R>` into one over `Triple<R::Owned>` using
/// the `IntoOwned` trait.
pub struct TriplesIntoOwned<I>(pub I);

impl<R: IntoOwned, I: Iterator<Item = Triple<R>>> Iterator for TriplesIntoOwned<I> {
	type Item = Triple<R::Owned>;

	fn next(&mut self) -> Option<Self::Item> {
		self.0.next().map(|t| t.map(IntoOwned::into_owned))
	}
}

/// Wraps an iterator over `Quad<R>` into one over `Quad<R::Owned>` using the
/// `IntoOwned` trait.
pub struct QuadsIntoOwned<I>(pub I);

impl<R: IntoOwned, I: Iterator<Item = Quad<R>>> Iterator for QuadsIntoOwned<I> {
	type Item = Quad<R::Owned>;

	fn next(&mut self) -> Option<Self::Item> {
		self.0.next().map(|q| q.map(IntoOwned::into_owned))
	}
}

pub struct OptionIterator<I>(pub Option<I>);

impl<I: Iterator> Iterator for OptionIterator<I> {
	type Item = I::Item;

	fn next(&mut self) -> Option<Self::Item> {
		self.0.as_mut().and_then(I::next)
	}
}
