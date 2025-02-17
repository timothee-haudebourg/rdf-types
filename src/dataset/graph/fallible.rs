use crate::{pattern::CanonicalTriplePattern, utils::InfallibleIterator, Triple};

use super::{Graph, GraphMut, PatternMatchingGraph, TraversableGraph};

/// Fallible graph.
pub trait FallibleGraph {
	type Resource;
	type Error;
}

impl<D: Graph> FallibleGraph for D {
	type Resource = D::Resource;
	type Error = std::convert::Infallible;
}

pub trait FallibleTraversableGraph: FallibleGraph {
	type TryTriples<'a>: Iterator<Item = Result<Triple<&'a Self::Resource>, Self::Error>>
	where
		Self: 'a;

	fn try_triples(&self) -> Self::TryTriples<'_>;
}

impl<D: TraversableGraph> FallibleTraversableGraph for D {
	type TryTriples<'a>
		= InfallibleIterator<D::Triples<'a>>
	where
		Self: 'a;

	fn try_triples(&self) -> Self::TryTriples<'_> {
		InfallibleIterator(self.triples())
	}
}

/// Pattern-matching-capable fallible dataset.
pub trait FalliblePatternMatchingGraph: FallibleGraph {
	type TryTriplePatternMatching<'a, 'p>: Iterator<
		Item = Result<Triple<&'a Self::Resource>, Self::Error>,
	>
	where
		Self: 'a,
		Self::Resource: 'p;

	fn try_triple_pattern_matching<'p>(
		&self,
		pattern: CanonicalTriplePattern<&'p Self::Resource>,
	) -> Self::TryTriplePatternMatching<'_, 'p>;

	fn try_contains_triple(&self, triple: Triple<&Self::Resource>) -> Result<bool, Self::Error> {
		Ok(self
			.try_triple_pattern_matching(triple.into())
			.next()
			.transpose()?
			.is_some())
	}
}

impl<D: PatternMatchingGraph> FalliblePatternMatchingGraph for D {
	type TryTriplePatternMatching<'a, 'p>
		= InfallibleIterator<D::TriplePatternMatching<'a, 'p>>
	where
		Self: 'a,
		Self::Resource: 'p;

	fn try_triple_pattern_matching<'p>(
		&self,
		pattern: CanonicalTriplePattern<&'p Self::Resource>,
	) -> Self::TryTriplePatternMatching<'_, 'p> {
		InfallibleIterator(self.triple_pattern_matching(pattern))
	}
}

/// Fallible mutable dataset.
pub trait FallibleGraphMut: FallibleGraph {
	fn try_insert(&mut self, triple: Triple<Self::Resource>) -> Result<(), Self::Error>;
}

impl<D: GraphMut> FallibleGraphMut for D {
	fn try_insert(&mut self, triple: Triple<Self::Resource>) -> Result<(), Self::Error> {
		self.insert(triple);
		Ok(())
	}
}
