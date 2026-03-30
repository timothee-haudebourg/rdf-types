use maybe_owned::MaybeOwned;
use std::future::Future;

use futures_lite::{stream, Stream};

use crate::{
	dataset::{
		fallible_graph::{TryFiniteGraph, TryGraphMut, TryPatternMatchingGraph},
		TryGraph,
	},
	pattern::CanonicalTriplePattern,
	Triple,
};

/// Async fallible finite graph.
pub trait AsyncTryFiniteGraph: TryGraph {
	type AsyncTryTriples<'a>: Stream<
		Item = Result<Triple<MaybeOwned<'a, Self::Resource>>, Self::Error>,
	>
	where
		Self: 'a;

	fn async_try_triples(&self) -> Self::AsyncTryTriples<'_>;
}

impl<D: TryFiniteGraph> AsyncTryFiniteGraph for D {
	type AsyncTryTriples<'a>
		= stream::Iter<D::TryTriples<'a>>
	where
		Self: 'a;

	fn async_try_triples(&self) -> Self::AsyncTryTriples<'_> {
		stream::iter(self.try_triples())
	}
}

/// Async fallible pattern-matching-capable graph.
pub trait AsyncTryPatternMatchingGraph: TryGraph {
	type AsyncTryTriplePatternMatching<'a, 'p>: Stream<
		Item = Result<Triple<MaybeOwned<'a, Self::Resource>>, Self::Error>,
	>
	where
		Self: 'a,
		Self::Resource: 'p;

	fn async_try_triple_pattern_matching<'p>(
		&self,
		pattern: CanonicalTriplePattern<MaybeOwned<'p, Self::Resource>>,
	) -> Self::AsyncTryTriplePatternMatching<'_, 'p>;

	fn async_try_contains_triple(
		&self,
		triple: Triple<&Self::Resource>,
	) -> impl Future<Output = Result<bool, Self::Error>> {
		async move {
			use futures_lite::StreamExt;
			let mut stream = std::pin::pin!(
				self.async_try_triple_pattern_matching(triple.map(MaybeOwned::Borrowed).into())
			);
			Ok(stream.next().await.transpose()?.is_some())
		}
	}
}

impl<D: TryPatternMatchingGraph> AsyncTryPatternMatchingGraph for D {
	type AsyncTryTriplePatternMatching<'a, 'p>
		= stream::Iter<D::TryTriplePatternMatching<'a, 'p>>
	where
		Self: 'a,
		Self::Resource: 'p;

	fn async_try_triple_pattern_matching<'p>(
		&self,
		pattern: CanonicalTriplePattern<MaybeOwned<'p, Self::Resource>>,
	) -> Self::AsyncTryTriplePatternMatching<'_, 'p> {
		stream::iter(self.try_triple_pattern_matching(pattern))
	}
}

/// Async fallible mutable graph.
pub trait AsyncTryGraphMut: TryGraph {
	fn async_try_insert(
		&mut self,
		triple: Triple<Self::Resource>,
	) -> impl Future<Output = Result<(), Self::Error>>;
}

impl<D: TryGraphMut> AsyncTryGraphMut for D {
	async fn async_try_insert(
		&mut self,
		triple: Triple<Self::Resource>,
	) -> Result<(), Self::Error> {
		self.try_insert(triple)
	}
}
