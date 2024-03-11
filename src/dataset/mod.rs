//! Dataset traits and implementations.
use crate::{
	pattern::{quad::canonical::PatternGraph, CanonicalQuadPattern},
	utils::{OptionIterator, TripleToQuadIterator},
	Quad,
};

pub mod fallible;
pub use fallible::FallibleDataset;

mod graph;
pub use graph::*;

mod r#impl;
pub use r#impl::*;

pub mod isomorphism;

/// RDF dataset.
pub trait Dataset {
	/// Resource type.
	type Resource;
}

impl<G: Graph> Dataset for G {
	type Resource = G::Resource;
}

/// Dataset that can be traversed using a provided quad iterator.
pub trait TraversableDataset: Dataset {
	/// Quads iterator.
	type Quads<'a>: Iterator<Item = Quad<&'a Self::Resource>>
	where
		Self: 'a;

	/// Returns an iterator over the quads of the dataset.
	fn quads(&self) -> Self::Quads<'_>;

	fn quads_count(&self) -> usize {
		self.quads().count()
	}
}

impl<G: TraversableGraph> TraversableDataset for G {
	type Quads<'a> = TripleToQuadIterator<G::Triples<'a>, &'a G::Resource> where Self: 'a;

	fn quads(&self) -> Self::Quads<'_> {
		TripleToQuadIterator::new(self.triples())
	}

	fn quads_count(&self) -> usize {
		TraversableGraph::triples_count(self)
	}
}

/// Pattern-matching-capable dataset.
pub trait PatternMatchingDataset: Dataset {
	/// Pattern-matching iterator.
	type QuadPatternMatching<'a, 'p>: Iterator<Item = Quad<&'a Self::Resource>>
	where
		Self: 'a,
		Self::Resource: 'p;

	/// Returns an iterator over all the quads of the dataset matching the given
	/// pattern.
	fn quad_pattern_matching<'p>(
		&self,
		pattern: CanonicalQuadPattern<&'p Self::Resource>,
	) -> Self::QuadPatternMatching<'_, 'p>;

	/// Checks if the dataset contains the given quad.
	fn contains_quad(&self, quad: Quad<&Self::Resource>) -> bool {
		self.quad_pattern_matching(quad.into()).next().is_some()
	}
}

impl<G: PatternMatchingGraph> PatternMatchingDataset for G {
	type QuadPatternMatching<'a, 'p> = OptionIterator<TripleToQuadIterator<G::TriplePatternMatching<'a, 'p>, &'a G::Resource>> where Self: 'a, Self::Resource: 'p;

	fn quad_pattern_matching<'p>(
		&self,
		pattern: CanonicalQuadPattern<&'p Self::Resource>,
	) -> Self::QuadPatternMatching<'_, 'p> {
		let (pattern, g) = pattern.into_triple();
		match g {
			PatternGraph::Given(None) | PatternGraph::Any => OptionIterator(Some(
				TripleToQuadIterator::new(self.triple_pattern_matching(pattern)),
			)),
			_ => OptionIterator(None),
		}
	}
}

/// Mutable dataset.
pub trait DatasetMut: Dataset {
	/// Inserts the given quad in the dataset.
	fn insert(&mut self, quad: Quad<Self::Resource>);

	/// Removes the given quad from the dataset.
	fn remove(&mut self, quad: Quad<&Self::Resource>);
}
