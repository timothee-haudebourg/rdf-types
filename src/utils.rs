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

pub struct OptionIterator<I>(pub Option<I>);

impl<I: Iterator> Iterator for OptionIterator<I> {
	type Item = I::Item;

	fn next(&mut self) -> Option<Self::Item> {
		self.0.as_mut().and_then(I::next)
	}
}
