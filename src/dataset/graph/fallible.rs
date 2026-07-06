use crate::{
	Triple,
	pattern::CanonicalTriplePattern,
	util::{InfallibleIterator, TriplesIntoOwned},
};

use super::{FiniteGraph, Graph, GraphMut, PatternMatchingGraph};

/// Fallible graph.
pub trait TryGraph {
	type Resource;
	type Error;
}

impl<D: Graph> TryGraph for D {
	type Resource = D::Resource;
	type Error = std::convert::Infallible;
}

pub trait TryFiniteGraph: TryGraph {
	type TryTriples<'a>: Iterator<Item = Result<Triple<Self::Resource>, Self::Error>>
	where
		Self: 'a;

	fn try_triples(&self) -> Result<Self::TryTriples<'_>, Self::Error>;
}

impl<D: FiniteGraph> TryFiniteGraph for D
where
	D::Resource: Clone,
{
	type TryTriples<'a>
		= InfallibleIterator<TriplesIntoOwned<D::Triples<'a>>>
	where
		Self: 'a;

	fn try_triples(&self) -> Result<Self::TryTriples<'_>, Self::Error> {
		Ok(InfallibleIterator(TriplesIntoOwned(self.triples())))
	}
}

/// Pattern-matching-capable fallible graph.
pub trait TryPatternMatchingGraph: TryGraph {
	type TryTriplePatternMatching<'a, 'p>: Iterator<
		Item = Result<Triple<Self::Resource>, Self::Error>,
	>
	where
		Self: 'a,
		Self::Resource: 'p;

	fn try_triple_pattern_matching<'p>(
		&self,
		pattern: CanonicalTriplePattern<&'p Self::Resource>,
	) -> Result<Self::TryTriplePatternMatching<'_, 'p>, Self::Error>;

	fn try_contains_triple(&self, triple: Triple<&Self::Resource>) -> Result<bool, Self::Error> {
		Ok(self
			.try_triple_pattern_matching(triple.into())?
			.next()
			.transpose()?
			.is_some())
	}
}

impl<D: PatternMatchingGraph> TryPatternMatchingGraph for D
where
	D::Resource: Clone,
{
	type TryTriplePatternMatching<'a, 'p>
		= InfallibleIterator<TriplesIntoOwned<D::TriplePatternMatching<'a, 'p>>>
	where
		Self: 'a,
		Self::Resource: 'p;

	fn try_triple_pattern_matching<'p>(
		&self,
		pattern: CanonicalTriplePattern<&'p Self::Resource>,
	) -> Result<Self::TryTriplePatternMatching<'_, 'p>, Self::Error> {
		Ok(InfallibleIterator(TriplesIntoOwned(
			self.triple_pattern_matching(pattern),
		)))
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
