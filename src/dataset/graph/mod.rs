//! RDF graph traits and implementations.
use crate::{Triple, pattern::LinearTriplePattern};

mod r#async;
mod fallible;
mod r#impl;

pub use r#async::*;
pub use fallible::*;
pub use r#impl::*;

/// RDF graph.
///
/// A graph is a set of [`Triple`]s. This trait only fixes the resource
/// type; see [`FiniteGraph`] and the other traits of this module for
/// actually usable graphs.
pub trait Graph {
	/// Resource type.
	type Resource;
}

/// Graph that can be traversed using a provided triple iterator.
pub trait FiniteGraph: Graph {
	/// Triples iterator.
	type Triples<'a>: Iterator<Item = Triple<&'a Self::Resource>>
	where
		Self: 'a;

	/// Returns an iterator over the triples of the graph.
	fn triples(&self) -> Self::Triples<'_>;

	/// Returns the number of triples in the graph.
	fn triples_count(&self) -> usize {
		self.triples().count()
	}
}

/// Graph that can enumerate the distinct resources it mentions (in any
/// position of any triple).
pub trait ResourceFiniteGraph: Graph {
	/// Resources iterator.
	type GraphResources<'a>: Iterator<Item = &'a Self::Resource>
	where
		Self: 'a;

	/// Returns an iterator over the distinct resources of the graph.
	fn graph_resources(&self) -> Self::GraphResources<'_>;

	/// Returns the number of distinct resources in the graph.
	fn graph_resource_count(&self) -> usize {
		self.graph_resources().count()
	}
}

/// Graph that can enumerate the distinct resources it uses as a subject.
pub trait SubjectFiniteGraph: Graph {
	/// Subjects iterator.
	type GraphSubjects<'a>: Iterator<Item = &'a Self::Resource>
	where
		Self: 'a;

	/// Returns an iterator over the distinct subjects of the graph.
	fn graph_subjects(&self) -> Self::GraphSubjects<'_>;

	/// Returns the number of distinct subjects in the graph.
	fn graph_subject_count(&self) -> usize {
		self.graph_subjects().count()
	}
}

/// Graph that can enumerate the distinct resources it uses as a predicate.
pub trait PredicateFiniteGraph: Graph {
	/// Predicates iterator.
	type GraphPredicates<'a>: Iterator<Item = &'a Self::Resource>
	where
		Self: 'a;

	/// Returns an iterator over the distinct predicates of the graph.
	fn graph_predicates(&self) -> Self::GraphPredicates<'_>;

	/// Returns the number of distinct predicates in the graph.
	fn graph_predicate_count(&self) -> usize {
		self.graph_predicates().count()
	}
}

/// Graph that can enumerate the distinct resources it uses as an object.
pub trait ObjectFiniteGraph: Graph {
	/// Objects iterator.
	type GraphObjects<'a>: Iterator<Item = &'a Self::Resource>
	where
		Self: 'a;

	/// Returns an iterator over the distinct objects of the graph.
	fn graph_objects(&self) -> Self::GraphObjects<'_>;

	/// Returns the number of distinct objects in the graph.
	fn graph_object_count(&self) -> usize {
		self.graph_objects().count()
	}
}

/// Multi-pattern-matching-capable graph.
///
/// Unlike [`PatternMatchingGraph`], each component of the pattern may match
/// any resource from a given set, rather than at most one fixed resource.
pub trait MultiPatternMatchingGraph: Graph {
	/// Pattern-matching iterator.
	type TripleMultiPatternMatching<'a, 'p>: Iterator<Item = Triple<&'a Self::Resource>>
	where
		Self: 'a,
		Self::Resource: 'p;

	/// Returns an iterator over all the triples of the graph matching the
	/// given pattern.
	fn triple_multi_pattern_matching<'p, P: IntoIterator<Item = &'p Self::Resource>>(
		&self,
		pattern: LinearTriplePattern<P>,
	) -> Self::TripleMultiPatternMatching<'_, 'p>;
}

/// Pattern-matching-capable graph.
///
/// A graph that can be queried with a [`LinearTriplePattern`], i.e. a triple
/// where each component is either a fixed resource or left unconstrained.
pub trait PatternMatchingGraph: Graph {
	/// Pattern-matching iterator.
	type TriplePatternMatching<'a, 'p>: Iterator<Item = Triple<&'a Self::Resource>>
	where
		Self: 'a,
		Self::Resource: 'p;

	/// Returns an iterator over all the triples of the graph matching the
	/// given pattern.
	fn triple_pattern_matching<'p>(
		&self,
		pattern: LinearTriplePattern<&'p Self::Resource>,
	) -> Self::TriplePatternMatching<'_, 'p>;

	/// Checks if the graph contains the given triple.
	fn contains_triple(&self, triple: Triple<&Self::Resource>) -> bool {
		self.triple_pattern_matching(triple.into()).next().is_some()
	}

	/// Checks if the graph contains the given subject.
	fn contains_triple_subject(&self, subject: &Self::Resource) -> bool {
		self.triple_pattern_matching(Triple(Some(subject), None, None))
			.next()
			.is_some()
	}

	/// Checks if the graph contains the given predicate.
	fn contains_triple_predicate(&self, predicate: &Self::Resource) -> bool {
		self.triple_pattern_matching(Triple(None, Some(predicate), None))
			.next()
			.is_some()
	}

	/// Checks if the graph contains the given object.
	fn contains_triple_object(&self, object: &Self::Resource) -> bool {
		self.triple_pattern_matching(Triple(None, None, Some(object)))
			.next()
			.is_some()
	}

	/// Returns an iterator over all the predicates `p` matching the triple `subject p o` present in the graph, for some `o`.
	fn triple_predicates_objects<'p>(
		&self,
		subject: &'p Self::Resource,
	) -> TriplePredicatesObjects<'_, 'p, Self>
	where
		Self: PredicateFiniteGraph,
		Self::Resource: 'p,
	{
		TriplePredicatesObjects {
			subject,
			predicates: self.graph_predicates(),
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
			inner: self.triple_pattern_matching(Triple(Some(subject), Some(predicate), None)),
		}
	}
}

/// Iterator over the predicates of a graph matching a given subject, along
/// with, for each predicate, the objects matching it.
///
/// Created by [`PatternMatchingGraph::triple_predicates_objects`].
pub struct TriplePredicatesObjects<
	'a,
	'p,
	G: 'a + ?Sized + PredicateFiniteGraph + PatternMatchingGraph,
> {
	subject: &'p G::Resource,
	predicates: G::GraphPredicates<'a>,
	graph: &'a G,
}

impl<'a: 'p, 'p, G: 'a + ?Sized + PredicateFiniteGraph + PatternMatchingGraph> Iterator
	for TriplePredicatesObjects<'a, 'p, G>
where
	G::Resource: 'p,
{
	type Item = (&'a G::Resource, TripleObjects<'p, 'p, G>);

	fn next(&mut self) -> Option<Self::Item> {
		for predicate in &mut self.predicates {
			let pattern = Triple(Some(self.subject), Some(predicate), None);

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

/// Iterator over the objects of a graph matching a given subject and
/// predicate.
///
/// Created by [`PatternMatchingGraph::triple_objects`].
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

/// Pattern-matching-capable mutable graph.
pub trait PatternMatchingGraphMut: PatternMatchingGraph {
	/// Pattern-matching iterator.
	type ExtractMatchingTriples<'a, 'p>: Iterator<Item = Triple<Self::Resource>>
	where
		Self: 'a,
		Self::Resource: 'p;

	/// Returns an iterator over all the triples matching the given linear
	/// triple pattern.
	///
	/// Each matching triple returned by [`Iterator::next`] are removed from
	/// the graph. Matching triples that are not iterated on are kept in the
	/// graph, even when the iterator is dropped.
	fn extract_matching_triples<'p>(
		&mut self,
		pattern: impl Into<LinearTriplePattern<&'p Self::Resource>>,
	) -> Self::ExtractMatchingTriples<'_, 'p>
	where
		Self::Resource: 'p;
}

/// Mutable graph.
pub trait GraphMut: Graph {
	/// Inserts the given triple in the graph.
	fn insert(&mut self, triple: Triple<Self::Resource>);

	/// Removes the given triple from the graph.
	fn remove(&mut self, triple: Triple<&Self::Resource>);
}
