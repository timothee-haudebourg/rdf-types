use std::borrow::Cow;
use std::future::Future;

use futures_lite::{stream, Stream};

use super::{FiniteGraph, GraphMut, PatternMatchingGraph};
use crate::{
	dataset::fallible_graph::TryGraph,
	pattern::CanonicalTriplePattern,
	utils::{BorrowedTriples, InfallibleIterator},
	Triple,
};

pub mod fallible;

/// Async finite graph.
pub trait AsyncFiniteGraph: TryGraph
where
	Self::Resource: ToOwned,
{
	type AsyncTriples<'a>: Stream<Item = Result<Triple<Cow<'a, Self::Resource>>, Self::Error>>
	where
		Self: 'a;

	fn async_triples(&self) -> Self::AsyncTriples<'_>;
}

impl<D: FiniteGraph> AsyncFiniteGraph for D
where
	D::Resource: ToOwned,
{
	type AsyncTriples<'a>
		= stream::Iter<InfallibleIterator<BorrowedTriples<D::Triples<'a>>>>
	where
		Self: 'a;

	fn async_triples(&self) -> Self::AsyncTriples<'_> {
		stream::iter(InfallibleIterator(BorrowedTriples(self.triples())))
	}
}

/// Async pattern-matching-capable graph.
pub trait AsyncPatternMatchingGraph: TryGraph
where
	Self::Resource: ToOwned,
{
	type AsyncTriplePatternMatching<'a, 'p>: Stream<
		Item = Result<Triple<Cow<'a, Self::Resource>>, Self::Error>,
	>
	where
		Self: 'a,
		Self::Resource: 'p;

	fn async_triple_pattern_matching<'p>(
		&self,
		pattern: CanonicalTriplePattern<&'p Self::Resource>,
	) -> Self::AsyncTriplePatternMatching<'_, 'p>;

	fn async_contains_triple(
		&self,
		triple: Triple<&Self::Resource>,
	) -> impl Future<Output = Result<bool, Self::Error>> {
		async move {
			use futures_lite::StreamExt;
			let mut stream = std::pin::pin!(self.async_triple_pattern_matching(triple.into()));
			Ok(stream.next().await.transpose()?.is_some())
		}
	}
}

impl<D: PatternMatchingGraph> AsyncPatternMatchingGraph for D
where
	D::Resource: ToOwned,
{
	type AsyncTriplePatternMatching<'a, 'p>
		= stream::Iter<InfallibleIterator<BorrowedTriples<D::TriplePatternMatching<'a, 'p>>>>
	where
		Self: 'a,
		Self::Resource: 'p;

	fn async_triple_pattern_matching<'p>(
		&self,
		pattern: CanonicalTriplePattern<&'p Self::Resource>,
	) -> Self::AsyncTriplePatternMatching<'_, 'p> {
		stream::iter(InfallibleIterator(BorrowedTriples(
			self.triple_pattern_matching(pattern),
		)))
	}
}

/// Async mutable graph.
pub trait AsyncGraphMut: TryGraph {
	fn async_insert(
		&mut self,
		triple: Triple<Self::Resource>,
	) -> impl Future<Output = Result<(), Self::Error>>;
}

impl<D: GraphMut> AsyncGraphMut for D {
	async fn async_insert(&mut self, triple: Triple<Self::Resource>) -> Result<(), Self::Error> {
		self.insert(triple);
		Ok(())
	}
}
