//! Small iterator adapters shared by the fallible/owned trait
//! implementations of this crate.
use std::marker::PhantomData;

use into_owned_trait::IntoOwned;

use crate::{Quad, Triple};

/// Wraps a infallible iterator into one yielding
/// [`Result<I::Item, Infallible>`](std::convert::Infallible), so it can be
/// used where a fallible iterator is expected.
pub struct InfallibleIterator<I>(pub I);

impl<I: Iterator> Iterator for InfallibleIterator<I> {
	type Item = Result<I::Item, std::convert::Infallible>;

	fn next(&mut self) -> Option<Self::Item> {
		self.0.next().map(Ok)
	}
}

/// Wraps an iterator over `Triple<S, P, O>` into one over `Quad<S, P, O, G>`,
/// leaving the graph component of every quad to `None`.
pub struct TriplesIntoQuads<I, G>(I, PhantomData<G>);

impl<I, G> TriplesIntoQuads<I, G> {
	/// Wraps the given triples iterator.
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

/// Turns an `Option<I>` into an iterator, yielding the items of `I` if
/// there is one, or nothing (`None`) otherwise.
pub struct OptionIterator<I>(pub Option<I>);

impl<I: Iterator> Iterator for OptionIterator<I> {
	type Item = I::Item;

	fn next(&mut self) -> Option<Self::Item> {
		self.0.as_mut().and_then(I::next)
	}
}
