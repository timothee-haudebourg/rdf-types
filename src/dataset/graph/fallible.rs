use crate::{pattern::CanonicalTriplePattern, utils::InfallibleIterator, Triple};
use std::borrow::Cow;

use super::{FiniteGraph, Graph, GraphMut, PatternMatchingGraph};

/// Fallible graph.
pub trait TryGraph {
	type Resource: ToOwned;
	type Error;
}

impl<D: Graph> TryGraph for D {
	type Resource = D::Resource;
	type Error = std::convert::Infallible;
}

pub trait TryFiniteGraph: TryGraph {
	type TryTriples<'a>: Iterator<Item = Result<Triple<Cow<'a, Self::Resource>>, Self::Error>>
	where
		Self: 'a;

	fn try_triples(&self) -> Self::TryTriples<'_>;
}

impl<D: FiniteGraph> TryFiniteGraph for D {
	type TryTriples<'a>
		= InfallibleIterator<D::Triples<'a>>
	where
		Self: 'a;

	fn try_triples(&self) -> Self::TryTriples<'_> {
		InfallibleIterator(self.triples())
	}
}

/// Pattern-matching-capable fallible graph.
pub trait TryPatternMatchingGraph: TryGraph {
	type TryTriplePatternMatching<'a, 'p>: Iterator<
		Item = Result<Triple<Cow<'a, Self::Resource>>, Self::Error>,
	>
	where
		Self: 'a,
		Self::Resource: 'p;

	fn try_triple_pattern_matching<'p>(
		&self,
		pattern: CanonicalTriplePattern<Cow<'p, Self::Resource>>,
	) -> Self::TryTriplePatternMatching<'_, 'p>;

	fn try_contains_triple(&self, triple: Triple<&Self::Resource>) -> Result<bool, Self::Error> {
		Ok(self
			.try_triple_pattern_matching(triple.map(Cow::Borrowed).into())
			.next()
			.transpose()?
			.is_some())
	}
}

impl<D: PatternMatchingGraph> TryPatternMatchingGraph for D {
	type TryTriplePatternMatching<'a, 'p>
		= InfallibleIterator<D::TriplePatternMatching<'a, 'p>>
	where
		Self: 'a,
		Self::Resource: 'p;

	fn try_triple_pattern_matching<'p>(
		&self,
		pattern: CanonicalTriplePattern<Cow<'p, Self::Resource>>,
	) -> Self::TryTriplePatternMatching<'_, 'p> {
		InfallibleIterator(self.triple_pattern_matching(pattern))
	}
}

/// Fallible mutable graph.
pub trait TryGraphMut: TryGraph {
	fn try_insert(&mut self, triple: Triple<Self::Resource>) -> Result<(), Self::Error>;
}

impl<D: GraphMut> TryGraphMut for D {
	fn try_insert(&mut self, triple: Triple<Self::Resource>) -> Result<(), Self::Error> {
		self.insert(triple);
		Ok(())
	}
}
