use crate::{pattern::CanonicalTriplePattern, Triple};

pub mod fallible;
pub use fallible::FallibleGraph;

mod r#impl;
pub use r#impl::*;

/// RDF graph.
pub trait Graph {
	type Resource;
}

pub trait TraversableGraph: Graph {
	type Triples<'a>: Iterator<Item = Triple<&'a Self::Resource>>
	where
		Self: 'a;

	fn triples(&self) -> Self::Triples<'_>;

	fn triples_count(&self) -> usize {
		self.triples().count()
	}
}

pub trait ResourceTraversableGraph: Graph {
	type GraphResources<'a>: Iterator<Item = &'a Self::Resource>
	where
		Self: 'a;

	fn graph_resources(&self) -> Self::GraphResources<'_>;

	fn graph_resource_count(&self) -> usize {
		self.graph_resources().count()
	}
}

/// Pattern-matching-capable dataset.
pub trait PatternMatchingGraph: Graph {
	type TriplePatternMatching<'a, 'p>: Iterator<Item = Triple<&'a Self::Resource>>
	where
		Self: 'a,
		Self::Resource: 'p;

	fn triple_pattern_matching<'p>(
		&self,
		pattern: CanonicalTriplePattern<&'p Self::Resource>,
	) -> Self::TriplePatternMatching<'_, 'p>;

	fn contains_triple(&self, triple: Triple<&Self::Resource>) -> bool {
		self.triple_pattern_matching(triple.into()).next().is_some()
	}

	/// Returns an iterator over all the predicates `p` matching the triple `subject p o` present in the graph, for some `o`.
	fn triple_predicates_objects<'p>(
		&self,
		subject: &'p Self::Resource,
	) -> TriplePredicatesObjects<'_, 'p, Self>
	where
		Self: ResourceTraversableGraph,
	{
		TriplePredicatesObjects {
			subject,
			predicates: self.graph_resources(),
			graph: self,
		}
	}

	/// Returns an iterator over all the objects `o` matching the triple `subject predicate o` present in the graph.
	fn triple_objects<'p>(
		&self,
		subject: &'p Self::Resource,
		predicate: &'p Self::Resource,
	) -> TripleObjects<'_, 'p, Self> {
		TripleObjects {
			first: None,
			inner: self.triple_pattern_matching(CanonicalTriplePattern::from_option_triple(
				Triple(Some(subject), Some(predicate), None),
			)),
		}
	}
}

pub struct TriplePredicatesObjects<
	'a,
	'p,
	G: 'a + ?Sized + ResourceTraversableGraph + PatternMatchingGraph,
> {
	subject: &'p G::Resource,
	predicates: G::GraphResources<'a>,
	graph: &'a G,
}

impl<'a: 'p, 'p, G: 'a + ?Sized + ResourceTraversableGraph + PatternMatchingGraph> Iterator
	for TriplePredicatesObjects<'a, 'p, G>
where
	G::Resource: 'p,
{
	type Item = (&'a G::Resource, TripleObjects<'p, 'p, G>);

	fn next(&mut self) -> Option<Self::Item> {
		for predicate in &mut self.predicates {
			use crate::pattern::triple::canonical::{GivenSubject, GivenSubjectGivenPredicate};
			let pattern = CanonicalTriplePattern::GivenSubject(
				self.subject,
				GivenSubject::GivenPredicate(predicate, GivenSubjectGivenPredicate::AnyObject),
			);

			let mut iter = self.graph.triple_pattern_matching(pattern);
			if let Some(Triple(_, _, o)) = iter.next() {
				return Some((
					predicate,
					TripleObjects {
						first: Some(o),
						inner: iter,
					},
				));
			}
		}

		None
	}
}

pub struct TripleObjects<'a, 'p, D: 'a + ?Sized + PatternMatchingGraph>
where
	D::Resource: 'p,
{
	first: Option<&'a D::Resource>,
	inner: D::TriplePatternMatching<'a, 'p>,
}

impl<'a, 'p, D: 'a + ?Sized + PatternMatchingGraph> Iterator for TripleObjects<'a, 'p, D>
where
	D::Resource: 'p,
{
	type Item = &'a D::Resource;

	fn next(&mut self) -> Option<Self::Item> {
		self.first
			.take()
			.or_else(|| self.inner.next().map(Triple::into_object))
	}
}

/// Mutable dataset.
pub trait GraphMut: Graph {
	fn insert(&mut self, triple: Triple<Self::Resource>);

	fn remove(&mut self, triple: Triple<&Self::Resource>);
}

/// Graph view focusing on a given resource.
pub struct GraphView<'a, G: Graph> {
	pub graph: &'a G,
	pub resource: &'a G::Resource,
}
