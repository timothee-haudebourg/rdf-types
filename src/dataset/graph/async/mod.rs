use std::borrow::Cow;
use std::future::Future;

use futures_lite::{stream, Stream};

use super::{FiniteGraph, GraphMut, PatternMatchingGraph};
use crate::{
	dataset::fallible_graph::TryGraph, pattern::CanonicalTriplePattern, utils::InfallibleIterator,
	Triple,
};

pub mod fallible;

/// Async finite graph.
pub trait AsyncFiniteGraph: TryGraph {
	type AsyncTriples<'a>: Stream<Item = Result<Triple<Cow<'a, Self::Resource>>, Self::Error>>
	where
		Self: 'a;

	fn async_triples(&self) -> Self::AsyncTriples<'_>;
}

impl<D: FiniteGraph> AsyncFiniteGraph for D {
	type AsyncTriples<'a>
		= stream::Iter<InfallibleIterator<D::Triples<'a>>>
	where
		Self: 'a;

	fn async_triples(&self) -> Self::AsyncTriples<'_> {
		stream::iter(InfallibleIterator(self.triples()))
	}
}

/// Async pattern-matching-capable graph.
pub trait AsyncPatternMatchingGraph: TryGraph {
	type AsyncTriplePatternMatching<'a, 'p>: Stream<
		Item = Result<Triple<Cow<'a, Self::Resource>>, Self::Error>,
	>
	where
		Self: 'a,
		Self::Resource: 'p;

	fn async_triple_pattern_matching<'p>(
		&self,
		pattern: CanonicalTriplePattern<Cow<'p, Self::Resource>>,
	) -> Self::AsyncTriplePatternMatching<'_, 'p>;

	fn async_contains_triple(
		&self,
		triple: Triple<&Self::Resource>,
	) -> impl Future<Output = Result<bool, Self::Error>> {
		async move {
			use futures_lite::StreamExt;
			let mut stream = std::pin::pin!(
				self.async_triple_pattern_matching(triple.map(Cow::Borrowed).into())
			);
			Ok(stream.next().await.transpose()?.is_some())
		}
	}
}

impl<D: PatternMatchingGraph> AsyncPatternMatchingGraph for D {
	type AsyncTriplePatternMatching<'a, 'p>
		= stream::Iter<InfallibleIterator<D::TriplePatternMatching<'a, 'p>>>
	where
		Self: 'a,
		Self::Resource: 'p;

	fn async_triple_pattern_matching<'p>(
		&self,
		pattern: CanonicalTriplePattern<Cow<'p, Self::Resource>>,
	) -> Self::AsyncTriplePatternMatching<'_, 'p> {
		stream::iter(InfallibleIterator(self.triple_pattern_matching(pattern)))
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
