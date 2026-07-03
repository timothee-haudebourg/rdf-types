//! Dataset traits and implementations.
use crate::{
	pattern::{quad::canonical::PatternGraph, CanonicalQuadPattern},
	utils::{OptionIterator, TripleToQuadIterator},
	Quad,
};
pub mod r#async;

pub mod fallible;
pub use fallible::TryDataset;

mod graph;
pub use graph::{fallible as fallible_graph, r#async as async_graph, *};

mod r#impl;
pub use r#impl::*;

/// RDF dataset.
pub trait Dataset {
	/// Resource type.
	type Resource;
}

impl<G: Graph> Dataset for G {
	type Resource = G::Resource;
}

/// Dataset that can be traversed using a provided quad iterator.
pub trait FiniteDataset: Dataset {
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

impl<G: FiniteGraph> FiniteDataset for G {
	type Quads<'a>
		= TripleToQuadIterator<G::Triples<'a>, &'a G::Resource>
	where
		Self: 'a;

	fn quads(&self) -> Self::Quads<'_> {
		TripleToQuadIterator::new(self.triples())
	}

	fn quads_count(&self) -> usize {
		FiniteGraph::triples_count(self)
	}
}

pub trait ResourceFiniteDataset: Dataset {
	type Resources<'a>: Iterator<Item = &'a Self::Resource>
	where
		Self: 'a;

	fn resources(&self) -> Self::Resources<'_>;

	fn resource_count(&self) -> usize {
		self.resources().count()
	}
}

impl<G: ResourceFiniteGraph> ResourceFiniteDataset for G {
	type Resources<'a>
		= G::GraphResources<'a>
	where
		Self: 'a;

	fn resources(&self) -> Self::Resources<'_> {
		self.graph_resources()
	}

	fn resource_count(&self) -> usize {
		self.graph_resource_count()
	}
}

pub trait SubjectFiniteDataset: Dataset {
	type Subjects<'a>: Iterator<Item = &'a Self::Resource>
	where
		Self: 'a;

	fn subjects(&self) -> Self::Subjects<'_>;

	fn subject_count(&self) -> usize {
		self.subjects().count()
	}
}

impl<G: SubjectFiniteGraph> SubjectFiniteDataset for G {
	type Subjects<'a>
		= G::GraphSubjects<'a>
	where
		Self: 'a;

	fn subjects(&self) -> Self::Subjects<'_> {
		self.graph_subjects()
	}

	fn subject_count(&self) -> usize {
		self.graph_subject_count()
	}
}

pub trait PredicateFiniteDataset: Dataset {
	type Predicates<'a>: Iterator<Item = &'a Self::Resource>
	where
		Self: 'a;

	fn predicates(&self) -> Self::Predicates<'_>;

	fn predicate_count(&self) -> usize {
		self.predicates().count()
	}
}

impl<G: PredicateFiniteGraph> PredicateFiniteDataset for G {
	type Predicates<'a>
		= G::GraphPredicates<'a>
	where
		Self: 'a;

	fn predicates(&self) -> Self::Predicates<'_> {
		self.graph_predicates()
	}

	fn predicate_count(&self) -> usize {
		self.graph_predicate_count()
	}
}

pub trait ObjectFiniteDataset: Dataset {
	type Objects<'a>: Iterator<Item = &'a Self::Resource>
	where
		Self: 'a;

	fn objects(&self) -> Self::Objects<'_>;

	fn object_count(&self) -> usize {
		self.objects().count()
	}
}

impl<G: ObjectFiniteGraph> ObjectFiniteDataset for G {
	type Objects<'a>
		= G::GraphObjects<'a>
	where
		Self: 'a;

	fn objects(&self) -> Self::Objects<'_> {
		self.graph_objects()
	}

	fn object_count(&self) -> usize {
		self.graph_object_count()
	}
}

pub trait NamedGraphFiniteDataset: Dataset {
	type NamedGraphs<'a>: Iterator<Item = &'a Self::Resource>
	where
		Self: 'a;

	fn named_graphs(&self) -> Self::NamedGraphs<'_>;

	fn named_graph_count(&self) -> usize {
		self.named_graphs().count()
	}
}

impl<G: Graph> NamedGraphFiniteDataset for G {
	type NamedGraphs<'a>
		= std::iter::Empty<&'a Self::Resource>
	where
		Self: 'a;

	fn named_graphs(&self) -> Self::NamedGraphs<'_> {
		std::iter::empty()
	}

	fn named_graph_count(&self) -> usize {
		0
	}
}

pub trait MultiPatternMatchingDataset: Dataset {
	/// Pattern-matching iterator.
	type QuadMultiPatternMatching<'a, 'p>: Iterator<Item = Quad<&'a Self::Resource>>
	where
		Self: 'a,
		Self::Resource: 'p;

	/// Returns an iterator over all the quads of the dataset matching the given
	/// pattern.
	fn quad_multi_pattern_matching<'p, P: IntoIterator<Item = &'p Self::Resource>>(
		&self,
		pattern: CanonicalQuadPattern<P>,
	) -> Self::QuadMultiPatternMatching<'_, 'p>;
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

	/// Checks if the dataset contains the given subject.
	fn contains_quad_subject(&self, subject: &Self::Resource) -> bool {
		use crate::pattern::quad::canonical::{
			GivenSubject, GivenSubjectAnyPredicate, GivenSubjectAnyPredicateAnyObject,
		};
		self.quad_pattern_matching(CanonicalQuadPattern::GivenSubject(
			subject,
			GivenSubject::AnyPredicate(GivenSubjectAnyPredicate::AnyObject(
				GivenSubjectAnyPredicateAnyObject::AnyGraph,
			)),
		))
		.next()
		.is_some()
	}

	/// Checks if the dataset contains the given predicate.
	fn contains_quad_predicate(&self, predicate: &Self::Resource) -> bool {
		use crate::pattern::quad::canonical::{
			AnySubject, AnySubjectGivenPredicate, AnySubjectGivenPredicateAnyObject,
		};
		self.quad_pattern_matching(CanonicalQuadPattern::AnySubject(
			AnySubject::GivenPredicate(
				predicate,
				AnySubjectGivenPredicate::AnyObject(AnySubjectGivenPredicateAnyObject::AnyGraph),
			),
		))
		.next()
		.is_some()
	}

	/// Checks if the dataset contains the given object.
	fn contains_quad_object(&self, object: &Self::Resource) -> bool {
		use crate::pattern::quad::canonical::{
			AnySubject, AnySubjectAnyPredicate, AnySubjectAnyPredicateGivenObject,
		};
		self.quad_pattern_matching(CanonicalQuadPattern::AnySubject(AnySubject::AnyPredicate(
			AnySubjectAnyPredicate::GivenObject(
				object,
				AnySubjectAnyPredicateGivenObject::AnyGraph,
			),
		)))
		.next()
		.is_some()
	}

	/// Checks if the dataset contains the given named graph.
	fn contains_named_graph(&self, named_graph: &Self::Resource) -> bool {
		use crate::pattern::quad::canonical::{
			AnySubject, AnySubjectAnyPredicate, AnySubjectAnyPredicateAnyObject,
		};
		self.quad_pattern_matching(CanonicalQuadPattern::AnySubject(AnySubject::AnyPredicate(
			AnySubjectAnyPredicate::AnyObject(AnySubjectAnyPredicateAnyObject::GivenGraph(Some(
				named_graph,
			))),
		)))
		.next()
		.is_some()
	}

	/// Returns an iterator over all the predicates `p` matching any quad
	/// `subject p o graph` present in the dataset, for any object `o`.
	fn quad_predicates_objects<'p>(
		&self,
		graph: Option<&'p Self::Resource>,
		subject: &'p Self::Resource,
	) -> QuadPredicatesObjects<'_, 'p, Self>
	where
		Self: PredicateFiniteDataset,
	{
		QuadPredicatesObjects {
			graph,
			subject,
			predicates: self.predicates(),
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
	type QuadPatternMatching<'a, 'p>
		= OptionIterator<TripleToQuadIterator<G::TriplePatternMatching<'a, 'p>, &'a G::Resource>>
	where
		Self: 'a,
		Self::Resource: 'p;

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
	D: 'a + ?Sized + PredicateFiniteDataset + PatternMatchingDataset,
> {
	graph: Option<&'p D::Resource>,
	subject: &'p D::Resource,
	predicates: D::Predicates<'a>,
	dataset: &'a D,
}

impl<'a: 'p, 'p, D: 'a + ?Sized + PredicateFiniteDataset + PatternMatchingDataset> Iterator
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
			let mut iter = self
				.dataset
				.quad_pattern_matching(CanonicalQuadPattern::GivenSubject(
					self.subject,
					GivenSubject::GivenPredicate(
						predicate,
						GivenSubjectGivenPredicate::AnyObject(
							GivenSubjectGivenPredicateAnyObject::GivenGraph(self.graph),
						),
					),
				));

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

/// Pattern-matching-capable mutable dataset.
pub trait PatternMatchingDatasetMut: PatternMatchingDataset {
	// Pattern-matching iterator.
	type ExtractMatchingQuads<'a, 'p>: Iterator<Item = Quad<Self::Resource>>
	where
		Self: 'a,
		Self::Resource: 'p;

	/// Returns an iterator over all the quads matching the given canonical
	/// quad pattern.
	///
	/// Each matching quad returned by [`Iterator::next`] are removed from the
	/// dataset. Matching quads that are not iterated on are kept in the
	/// dataset, even when the iterator is dropped.
	fn extract_matching_quads<'p>(
		&mut self,
		pattern: impl Into<CanonicalQuadPattern<&'p Self::Resource>>,
	) -> Self::ExtractMatchingQuads<'_, 'p>
	where
		Self::Resource: 'p;
}

/// Mutable dataset.
pub trait DatasetMut: Dataset {
	/// Inserts the given quad in the dataset.
	fn insert(&mut self, quad: Quad<Self::Resource>);

	/// Removes the given quad from the dataset.
	fn remove(&mut self, quad: Quad<&Self::Resource>);
}

/// Dataset view focusing on a given graph.
pub struct DatasetView<'a, D: Dataset> {
	pub dataset: &'a D,
	pub graph: Option<&'a D::Resource>,
}

/// Dataset view focusing on a given resource and restricted to the given graph.
pub struct DatasetGraphView<'a, D: Dataset> {
	pub dataset: &'a D,
	pub graph: Option<&'a D::Resource>,
	pub resource: &'a D::Resource,
}
