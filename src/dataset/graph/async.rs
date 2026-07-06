use std::pin::pin;

use futures_lite::{Stream, StreamExt, stream};

use crate::{
	Triple,
	dataset::{TryFiniteGraph, TryGraph, TryGraphMut, TryPatternMatchingGraph},
	pattern::CanonicalTriplePattern,
};

/// Async finite graph.
pub trait AsyncFiniteGraph: TryGraph {
	type AsyncTriples<'a>: Stream<Item = Result<Triple<Self::Resource>, Self::Error>>
	where
		Self: 'a;

	async fn async_triples(&self) -> Result<Self::AsyncTriples<'_>, Self::Error>;
}

impl<D: TryFiniteGraph> AsyncFiniteGraph for D {
	type AsyncTriples<'a>
		= stream::Iter<D::TryTriples<'a>>
	where
		Self: 'a;

	async fn async_triples(&self) -> Result<Self::AsyncTriples<'_>, Self::Error> {
		self.try_triples().map(stream::iter)
	}
}

/// Async pattern-matching-capable graph.
pub trait AsyncPatternMatchingGraph: TryGraph {
	type AsyncTriplePatternMatching<'a, 'p>: Stream<
		Item = Result<Triple<Self::Resource>, Self::Error>,
	>
	where
		Self: 'a,
		Self::Resource: 'p;

	async fn async_triple_pattern_matching<'p>(
		&self,
		pattern: CanonicalTriplePattern<&'p Self::Resource>,
	) -> Result<Self::AsyncTriplePatternMatching<'_, 'p>, Self::Error>;

	async fn async_contains_triple(
		&self,
		triple: Triple<&Self::Resource>,
	) -> Result<bool, Self::Error> {
		let mut stream = pin!(self.async_triple_pattern_matching(triple.into()).await?);
		Ok(stream.next().await.transpose()?.is_some())
	}
}

impl<D: TryPatternMatchingGraph> AsyncPatternMatchingGraph for D {
	type AsyncTriplePatternMatching<'a, 'p>
		= stream::Iter<D::TryTriplePatternMatching<'a, 'p>>
	where
		Self: 'a,
		Self::Resource: 'p;

	async fn async_triple_pattern_matching<'p>(
		&self,
		pattern: CanonicalTriplePattern<&'p Self::Resource>,
	) -> Result<Self::AsyncTriplePatternMatching<'_, 'p>, Self::Error> {
		self.try_triple_pattern_matching(pattern).map(stream::iter)
	}
}

/// Async mutable graph.
pub trait AsyncGraphMut: TryGraph {
	async fn async_insert(&mut self, triple: Triple<Self::Resource>) -> Result<(), Self::Error>;
}

impl<D: TryGraphMut> AsyncGraphMut for D {
	async fn async_insert(&mut self, triple: Triple<Self::Resource>) -> Result<(), Self::Error> {
		self.try_insert(triple)
	}
}
