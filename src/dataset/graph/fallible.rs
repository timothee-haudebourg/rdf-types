//! Fallible counterparts of the [`Graph`] traits, for graphs backed by
//! fallible storage (e.g. a database or a file).
use crate::{
	Triple,
	pattern::LinearTriplePattern,
	util::{InfallibleIterator, TriplesIntoOwned},
};

use super::{FiniteGraph, Graph, GraphMut, PatternMatchingGraph};

/// Fallible graph.
///
/// A [`Graph`] whose operations may fail with an associated [`Self::Error`]
/// type.
///
/// Every non-fallible [`Graph`] is also a `TryGraph`, with the
/// [`Infallible`](std::convert::Infallible) error type.
pub trait TryGraph {
	/// Resource type.
	type Resource;

	/// Error type.
	type Error;
}

/// Any non-fallible graph can be used as fallible, with the
/// [`Infallible`](std::convert::Infallible) error type.
impl<D: Graph> TryGraph for D {
	type Resource = D::Resource;
	type Error = std::convert::Infallible;
}

/// Fallible graph that can be traversed using a provided triple iterator.
pub trait TryFiniteGraph: TryGraph {
	/// Fallible triples iterator.
	type TryTriples<'a>: Iterator<Item = Result<Triple<Self::Resource>, Self::Error>>
	where
		Self: 'a;

	/// Returns a fallible iterator over the triples of the graph.
	fn try_triples(&self) -> Result<Self::TryTriples<'_>, Self::Error>;
}

/// Any non-fallible finite graph can be used as fallible, with the
/// [`Infallible`](std::convert::Infallible) error type.
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
	/// Fallible pattern-matching iterator.
	type TryTriplePatternMatching<'a, 'p>: Iterator<
		Item = Result<Triple<Self::Resource>, Self::Error>,
	>
	where
		Self: 'a,
		Self::Resource: 'p;

	/// Returns a fallible iterator over all the triples of the graph
	/// matching the given pattern.
	fn try_triple_pattern_matching<'p>(
		&self,
		pattern: LinearTriplePattern<&'p Self::Resource>,
	) -> Result<Self::TryTriplePatternMatching<'_, 'p>, Self::Error>;

	/// Checks if the graph contains the given triple.
	fn try_contains_triple(&self, triple: Triple<&Self::Resource>) -> Result<bool, Self::Error> {
		Ok(self
			.try_triple_pattern_matching(triple.into())?
			.next()
			.transpose()?
			.is_some())
	}
}

/// Any non-fallible pattern-matching-capable graph can be used as fallible,
/// with the [`Infallible`](std::convert::Infallible) error type.
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
		pattern: LinearTriplePattern<&'p Self::Resource>,
	) -> Result<Self::TryTriplePatternMatching<'_, 'p>, Self::Error> {
		Ok(InfallibleIterator(TriplesIntoOwned(
			self.triple_pattern_matching(pattern),
		)))
	}
}

/// Fallible mutable graph.
pub trait TryGraphMut: TryGraph {
	/// Tries to insert the given triple in the graph.
	fn try_insert(&mut self, triple: Triple<Self::Resource>) -> Result<(), Self::Error>;
}

/// Any non-fallible mutable graph can be used as fallible, with the
/// [`Infallible`](std::convert::Infallible) error type.
impl<D: GraphMut> TryGraphMut for D {
	fn try_insert(&mut self, triple: Triple<Self::Resource>) -> Result<(), Self::Error> {
		self.insert(triple);
		Ok(())
	}
}
