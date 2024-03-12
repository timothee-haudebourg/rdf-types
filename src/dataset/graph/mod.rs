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

	/// Returns an iterator over all the objects `o` matching the triple `subject predicate o`.
	fn triple_objects<'p>(
		&self,
		subject: &'p Self::Resource,
		predicate: &'p Self::Resource,
	) -> TripleObjects<'_, 'p, Self> {
		TripleObjects(
			self.triple_pattern_matching(CanonicalTriplePattern::from_option_triple(Triple(
				Some(subject),
				Some(predicate),
				None,
			))),
		)
	}
}

pub struct TripleObjects<'a, 'p, D: 'a + ?Sized + PatternMatchingGraph>(
	D::TriplePatternMatching<'a, 'p>,
)
where
	D::Resource: 'p;

impl<'a, 'p, D: 'a + ?Sized + PatternMatchingGraph> Iterator for TripleObjects<'a, 'p, D>
where
	D::Resource: 'p,
{
	type Item = &'a D::Resource;

	fn next(&mut self) -> Option<Self::Item> {
		self.0.next().map(Triple::into_object)
	}
}

/// Mutable dataset.
pub trait GraphMut: Graph {
	fn insert(&mut self, triple: Triple<Self::Resource>);

	fn remove(&mut self, triple: Triple<&Self::Resource>);
}
