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

pub trait ResourceTraversableDataset: Dataset {
	type Resources<'a>: Iterator<Item = &'a Self::Resource>
	where
		Self: 'a;

	fn resources(&self) -> Self::Resources<'_>;

	fn resource_count(&self) -> usize {
		self.resources().count()
	}
}

impl<G: ResourceTraversableGraph> ResourceTraversableDataset for G {
	type Resources<'a> = G::GraphResources<'a> where Self: 'a;

	fn resources(&self) -> Self::Resources<'_> {
		self.graph_resources()
	}

	fn resource_count(&self) -> usize {
		self.graph_resource_count()
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

	/// Returns an iterator over all the predicates `p` matching any quad
	/// `subject p o graph` present in the dataset, for any object `o`.
	fn quad_predicates_objects<'p>(
		&self,
		graph: Option<&'p Self::Resource>,
		subject: &'p Self::Resource,
	) -> QuadPredicatesObjects<'_, 'p, Self>
	where
		Self: ResourceTraversableDataset,
	{
		QuadPredicatesObjects {
			graph,
			subject,
			predicates: self.resources(),
			dataset: self,
		}
	}

	/// Returns an iterator over all the objects `o` matching the quad `subject predicate o graph`.
	fn quad_objects<'p>(
		&self,
		graph: Option<&'p Self::Resource>,
		subject: &'p Self::Resource,
		predicate: &'p Self::Resource,
	) -> QuadObjects<'_, 'p, Self> {
		QuadObjects {
			first: None,
			inner: self.quad_pattern_matching(CanonicalQuadPattern::from_option_quad(Quad(
				Some(subject),
				Some(predicate),
				None,
				Some(graph),
			))),
		}
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

pub struct QuadPredicatesObjects<
	'a,
	'p,
	D: 'a + ?Sized + ResourceTraversableDataset + PatternMatchingDataset,
> {
	graph: Option<&'p D::Resource>,
	subject: &'p D::Resource,
	predicates: D::Resources<'a>,
	dataset: &'a D,
}

impl<'a: 'p, 'p, D: 'a + ?Sized + ResourceTraversableDataset + PatternMatchingDataset> Iterator
	for QuadPredicatesObjects<'a, 'p, D>
where
	D::Resource: 'p,
{
	type Item = (&'a D::Resource, QuadObjects<'p, 'p, D>);

	fn next(&mut self) -> Option<Self::Item> {
		for predicate in &mut self.predicates {
			use crate::pattern::quad::canonical::{
				GivenSubject, GivenSubjectGivenPredicate, GivenSubjectGivenPredicateAnyObject,
			};
			let pattern = CanonicalQuadPattern::GivenSubject(
				self.subject,
				GivenSubject::GivenPredicate(
					predicate,
					GivenSubjectGivenPredicate::AnyObject(
						GivenSubjectGivenPredicateAnyObject::GivenGraph(self.graph),
					),
				),
			);

			let mut iter = self.dataset.quad_pattern_matching(pattern);
			if let Some(Quad(_, _, o, _)) = iter.next() {
				return Some((
					predicate,
					QuadObjects {
						first: Some(o),
						inner: iter,
					},
				));
			}
		}

		None
	}
}

pub struct QuadObjects<'a, 'p, D: 'a + ?Sized + PatternMatchingDataset>
where
	D::Resource: 'p,
{
	first: Option<&'a D::Resource>,
	inner: D::QuadPatternMatching<'a, 'p>,
}

impl<'a, 'p, D: 'a + ?Sized + PatternMatchingDataset> Iterator for QuadObjects<'a, 'p, D>
where
	D::Resource: 'p,
{
	type Item = &'a D::Resource;

	fn next(&mut self) -> Option<Self::Item> {
		self.first
			.take()
			.or_else(|| self.inner.next().map(Quad::into_object))
	}
}

/// Mutable dataset.
pub trait DatasetMut: Dataset {
	/// Inserts the given quad in the dataset.
	fn insert(&mut self, quad: Quad<Self::Resource>);

	/// Removes the given quad from the dataset.
	fn remove(&mut self, quad: Quad<&Self::Resource>);
}

/// Dataset view focusing on a given resource.
pub struct DatasetView<'a, D: Dataset> {
	pub dataset: &'a D,
	pub resource: &'a D::Resource,
}

/// Dataset view focusing on a given resource and restricted to the given graph.
pub struct DatasetGraphView<'a, D: Dataset> {
	pub dataset: &'a D,
	pub graph: Option<&'a D::Resource>,
	pub resource: &'a D::Resource,
}
