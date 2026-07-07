//! Asynchronous counterparts of the [`Graph`](super::Graph) traits, for
//! graphs backed by asynchronous storage.
use std::pin::pin;

use futures_lite::{Stream, StreamExt, stream};

use crate::{
	Triple,
	dataset::{TryFiniteGraph, TryGraph, TryGraphMut, TryPatternMatchingGraph},
	pattern::LinearTriplePattern,
};

/// Async finite graph.
pub trait AsyncFiniteGraph: TryGraph {
	/// Asynchronous fallible triples stream.
	type AsyncTriples<'a>: Stream<Item = Result<Triple<Self::Resource>, Self::Error>>
	where
		Self: 'a;

	/// Returns a stream over the triples of the graph.
	async fn async_triples(&self) -> Result<Self::AsyncTriples<'_>, Self::Error>;
}

/// Any fallible finite graph can be used, synchronously, as an asynchronous
/// finite graph.
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
	/// Asynchronous fallible pattern-matching stream.
	type AsyncTriplePatternMatching<'a, 'p>: Stream<
		Item = Result<Triple<Self::Resource>, Self::Error>,
	>
	where
		Self: 'a,
		Self::Resource: 'p;

	/// Returns a stream over all the triples of the graph matching the
	/// given pattern.
	async fn async_triple_pattern_matching<'p>(
		&self,
		pattern: LinearTriplePattern<&'p Self::Resource>,
	) -> Result<Self::AsyncTriplePatternMatching<'_, 'p>, Self::Error>;

	/// Checks if the graph contains the given triple.
	async fn async_contains_triple(
		&self,
		triple: Triple<&Self::Resource>,
	) -> Result<bool, Self::Error> {
		let mut stream = pin!(self.async_triple_pattern_matching(triple.into()).await?);
		Ok(stream.next().await.transpose()?.is_some())
	}
}

/// Any fallible pattern-matching-capable graph can be used, synchronously,
/// as an asynchronous pattern-matching-capable graph.
impl<D: TryPatternMatchingGraph> AsyncPatternMatchingGraph for D {
	type AsyncTriplePatternMatching<'a, 'p>
		= stream::Iter<D::TryTriplePatternMatching<'a, 'p>>
	where
		Self: 'a,
		Self::Resource: 'p;

	async fn async_triple_pattern_matching<'p>(
		&self,
		pattern: LinearTriplePattern<&'p Self::Resource>,
	) -> Result<Self::AsyncTriplePatternMatching<'_, 'p>, Self::Error> {
		self.try_triple_pattern_matching(pattern).map(stream::iter)
	}
}

/// Async mutable graph.
pub trait AsyncGraphMut: TryGraph {
	/// Inserts the given triple in the graph.
	async fn async_insert(&mut self, triple: Triple<Self::Resource>) -> Result<(), Self::Error>;
}

/// Any fallible mutable graph can be used, synchronously, as an
/// asynchronous mutable graph.
impl<D: TryGraphMut> AsyncGraphMut for D {
	async fn async_insert(&mut self, triple: Triple<Self::Resource>) -> Result<(), Self::Error> {
		self.try_insert(triple)
	}
}
