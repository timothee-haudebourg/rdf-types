use std::marker::PhantomData;

use crate::{Quad, Triple};

pub struct InfallibleIterator<I>(pub I);

impl<I: Iterator> Iterator for InfallibleIterator<I> {
	type Item = Result<I::Item, std::convert::Infallible>;

	fn next(&mut self) -> Option<Self::Item> {
		self.0.next().map(Ok)
	}
}

pub struct TripleToQuadIterator<I, G>(I, PhantomData<G>);

impl<I, G> TripleToQuadIterator<I, G> {
	pub fn new(inner: I) -> Self {
		Self(inner, PhantomData)
	}
}

impl<S, P, O, G, I: Iterator<Item = Triple<S, P, O>>> Iterator for TripleToQuadIterator<I, G> {
	type Item = Quad<S, P, O, G>;

	fn next(&mut self) -> Option<Self::Item> {
		self.0.next().map(|t| t.into_quad(None))
	}
}

/// Wraps an iterator over `&'a R` into one over `MaybeOwned<'a, R>`.
pub struct BorrowedResources<I>(pub I);

impl<'a, R: 'a, I: Iterator<Item = &'a R>> Iterator for BorrowedResources<I> {
	type Item = maybe_owned::MaybeOwned<'a, R>;

	fn next(&mut self) -> Option<Self::Item> {
		self.0.next().map(maybe_owned::MaybeOwned::Borrowed)
	}
}

/// Wraps an iterator over `Triple<&'a R>` into one over `Triple<MaybeOwned<'a, R>>`.
pub struct BorrowedTriples<I>(pub I);

impl<'a, R: 'a, I: Iterator<Item = Triple<&'a R>>> Iterator for BorrowedTriples<I> {
	type Item = Triple<maybe_owned::MaybeOwned<'a, R>>;

	fn next(&mut self) -> Option<Self::Item> {
		self.0
			.next()
			.map(|t| t.map(maybe_owned::MaybeOwned::Borrowed))
	}
}

/// Wraps an iterator over `Quad<&'a R>` into one over `Quad<MaybeOwned<'a, R>>`.
pub struct BorrowedQuads<I>(pub I);

impl<'a, R: 'a, I: Iterator<Item = Quad<&'a R>>> Iterator for BorrowedQuads<I> {
	type Item = Quad<maybe_owned::MaybeOwned<'a, R>>;

	fn next(&mut self) -> Option<Self::Item> {
		self.0
			.next()
			.map(|q| q.map(maybe_owned::MaybeOwned::Borrowed))
	}
}

pub struct OptionIterator<I>(pub Option<I>);

impl<I: Iterator> Iterator for OptionIterator<I> {
	type Item = I::Item;

	fn next(&mut self) -> Option<Self::Item> {
		self.0.as_mut().and_then(I::next)
	}
}
