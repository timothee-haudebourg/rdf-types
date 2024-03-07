use std::{
	cmp::Ordering,
	collections::{BTreeMap, BTreeSet},
	hash::Hash,
};

use raw_btree::RawBTree;
use slab::Slab;

use super::super::{Graph, PatternMatchingGraph};
use crate::{
	dataset::{GraphMut, TraversableGraph},
	pattern::{
		triple::canonical::{PatternObject, PatternPredicate, PatternSubject},
		CanonicalTriplePattern,
	},
	Term, Triple,
};

fn triple_cmp<R: Ord>(triples: &Slab<Triple<R>>) -> impl '_ + Fn(&usize, &Triple<&R>) -> Ordering {
	|&i, triple| triples[i].as_ref().cmp(triple)
}

fn index_cmp<R: Ord>(triples: &Slab<Triple<R>>) -> impl '_ + Fn(&usize, &usize) -> Ordering {
	|&i, &j| triples[i].cmp(&triples[j])
}

/// BTree-based RDF graph.
pub struct BTreeGraph<R = Term> {
	triples: Slab<Triple<R>>,
	indexes: RawBTree<usize>,
	resources: BTreeMap<R, Resource>,
}

impl<R> Default for BTreeGraph<R> {
	fn default() -> Self {
		Self {
			triples: Slab::new(),
			indexes: RawBTree::new(),
			resources: BTreeMap::new(),
		}
	}
}

impl<R> BTreeGraph<R> {
	/// Creates a new empty graph.
	pub fn new() -> Self {
		Self::default()
	}

	/// Returns the number of triples in the graph.
	pub fn len(&self) -> usize {
		self.triples.len()
	}

	/// Checks if the graph is empty.
	pub fn is_empty(&self) -> bool {
		self.triples.is_empty()
	}

	/// Returns an iterator over the triples of the graph.
	pub fn iter(&self) -> Triples<R> {
		Triples {
			triples: &self.triples,
			indexes: self.indexes.iter(),
		}
	}
}

impl<R: Ord> BTreeGraph<R> {
	/// Returns the index of the given triple in the graph, if any.
	fn index_of(&self, triple: Triple<&R>) -> Option<usize> {
		self.indexes
			.get(triple_cmp(&self.triples), &triple)
			.copied()
	}

	/// Checks if the provided triple is in the graph.
	pub fn contains(&self, triple: Triple<&R>) -> bool {
		self.index_of(triple).is_some()
	}

	/// Inserts the given triple in the graph.
	///
	/// Returns `true` if the triple was not already in the graph, and `false`
	/// if it was.
	pub fn insert(&mut self, triple: Triple<R>) -> bool
	where
		R: Clone,
	{
		if self.contains(triple.as_ref()) {
			false
		} else {
			let e = self.triples.vacant_entry();
			let i = e.key();

			match self.resources.get_mut(&triple.0) {
				Some(s) => {
					s.as_subject.insert(i);
				}
				None => {
					self.resources
						.insert(triple.0.clone(), Resource::subject(i));
				}
			}

			match self.resources.get_mut(&triple.1) {
				Some(p) => {
					p.as_predicate.insert(i);
				}
				None => {
					self.resources
						.insert(triple.1.clone(), Resource::predicate(i));
				}
			}

			match self.resources.get_mut(&triple.2) {
				Some(o) => {
					o.as_object.insert(i);
				}
				None => {
					self.resources.insert(triple.2.clone(), Resource::object(i));
				}
			}

			e.insert(triple);

			self.indexes.insert(index_cmp(&self.triples), i);

			true
		}
	}

	/// Removes the given triple from the graph.
	///
	/// Returns the instance of the triple that has been removed from the graph,
	/// if any. Does nothing if the triple was not in the graph.
	pub fn remove(&mut self, triple: Triple<&R>) -> Option<Triple<R>> {
		match self.index_of(triple) {
			Some(i) => {
				self.indexes.remove(index_cmp(&self.triples), &i);

				let s = self.resources.get_mut(triple.0).unwrap();
				s.as_subject.remove(&i);
				if s.is_empty() {
					self.resources.remove(triple.0);
				}

				let p = self.resources.get_mut(triple.1).unwrap();
				p.as_predicate.remove(&i);
				if p.is_empty() {
					self.resources.remove(triple.1);
				}

				let o = self.resources.get_mut(triple.2).unwrap();
				o.as_object.remove(&i);
				if o.is_empty() {
					self.resources.remove(triple.2);
				}

				Some(self.triples.remove(i))
			}
			None => None,
		}
	}

	/// Returns an iterator over all the triples matching the given canonical
	/// triple pattern.
	pub fn pattern_matching(&self, pattern: CanonicalTriplePattern<&R>) -> PatternMatching<R> {
		PatternMatching {
			triples: &self.triples,
			subject: SubjectConstraints::new(&self.resources, pattern.into_subject()),
			predicate: PredicateConstraints::new(&self.resources, pattern.into_predicate()),
			object: ObjectConstraints::new(&self.resources, pattern.into_object()),
			i: 0,
		}
	}
}

impl<R: Clone + Ord> FromIterator<Triple<R>> for BTreeGraph<R> {
	fn from_iter<T: IntoIterator<Item = Triple<R>>>(iter: T) -> Self {
		let mut result = Self::new();
		result.extend(iter);
		result
	}
}

impl<R: Clone + Ord> Extend<Triple<R>> for BTreeGraph<R> {
	fn extend<T: IntoIterator<Item = Triple<R>>>(&mut self, iter: T) {
		for triple in iter {
			self.insert(triple);
		}
	}
}

impl<R> Graph for BTreeGraph<R> {
	type Resource = R;
}

impl<R> TraversableGraph for BTreeGraph<R> {
	type Triples<'a> = Triples<'a, R> where R: 'a;

	fn triples(&self) -> Self::Triples<'_> {
		self.iter()
	}
}

impl<R: Clone + Ord> GraphMut for BTreeGraph<R> {
	fn insert(&mut self, triple: Triple<Self::Resource>) {
		self.insert(triple);
	}

	fn remove(&mut self, triple: Triple<&Self::Resource>) {
		self.remove(triple);
	}
}

impl<R: Ord> PatternMatchingGraph for BTreeGraph<R> {
	type TriplePatternMatching<'a, 'p> = PatternMatching<'a, R> where R: 'a, Self::Resource: 'p;

	fn triple_pattern_matching<'p>(
		&self,
		pattern: CanonicalTriplePattern<&'p Self::Resource>,
	) -> Self::TriplePatternMatching<'_, 'p> {
		self.pattern_matching(pattern)
	}

	fn contains_triple(&self, triple: Triple<&Self::Resource>) -> bool {
		self.contains(triple)
	}
}

/// Iterator over the triples of a [`BTreeGraph`].
pub struct Triples<'a, R> {
	triples: &'a Slab<Triple<R>>,
	indexes: raw_btree::Iter<'a, usize>,
}

impl<'a, R> Iterator for Triples<'a, R> {
	type Item = Triple<&'a R>;

	fn next(&mut self) -> Option<Self::Item> {
		self.indexes.next().map(|&i| self.triples[i].as_ref())
	}
}

/// Iterator over the triples of a [`BTreeGraph`].
pub struct IntoTriples<R> {
	triples: Slab<Triple<R>>,
	indexes: raw_btree::IntoIter<usize>,
}

impl<R> Iterator for IntoTriples<R> {
	type Item = Triple<R>;

	fn next(&mut self) -> Option<Self::Item> {
		self.indexes.next().map(|i| self.triples.remove(i))
	}
}

impl<'a, R> IntoIterator for &'a BTreeGraph<R> {
	type Item = Triple<&'a R>;
	type IntoIter = Triples<'a, R>;

	fn into_iter(self) -> Self::IntoIter {
		self.iter()
	}
}

impl<R> IntoIterator for BTreeGraph<R> {
	type Item = Triple<R>;
	type IntoIter = IntoTriples<R>;

	fn into_iter(self) -> Self::IntoIter {
		IntoTriples {
			triples: self.triples,
			indexes: self.indexes.into_iter(),
		}
	}
}

impl<R: PartialEq> PartialEq for BTreeGraph<R> {
	fn eq(&self, other: &Self) -> bool {
		self.len() == other.len() && self.iter().zip(other).all(|(a, b)| a == b)
	}
}

impl<R: Eq> Eq for BTreeGraph<R> {}

impl<R: PartialOrd> PartialOrd for BTreeGraph<R> {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		self.iter().partial_cmp(other)
	}
}

impl<R: Ord> Ord for BTreeGraph<R> {
	fn cmp(&self, other: &Self) -> Ordering {
		self.iter().cmp(other)
	}
}

impl<R: Hash> Hash for BTreeGraph<R> {
	fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
		state.write_usize(self.len());
		for elt in self {
			elt.hash(state);
		}
	}
}

/// Iterator over the triples of a [`BTreeGraph`] matching some given pattern.
pub struct PatternMatching<'a, R> {
	triples: &'a Slab<Triple<R>>,
	subject: SubjectConstraints<'a>,
	predicate: PredicateConstraints<'a>,
	object: ObjectConstraints<'a>,
	i: usize,
}

impl<'a, R: PartialEq> Iterator for PatternMatching<'a, R> {
	type Item = Triple<&'a R>;

	fn next(&mut self) -> Option<Self::Item> {
		while self.i < self.triples.capacity() {
			match self.triples.get(self.i) {
				Some(triple) => match self.subject.next(self.i) {
					Some(i) => match self.predicate.next(i, triple) {
						Ok(()) => match self.object.next(i, triple) {
							Ok(()) => {
								self.i = i + 1;
								return Some(triple.as_ref());
							}
							Err(Some(j)) => self.i = j,
							Err(None) => return None,
						},
						Err(Some(j)) => self.i = j,
						Err(None) => return None,
					},
					None => return None,
				},
				None => self.i += 1,
			}
		}

		None
	}
}

enum SubjectConstraints<'a> {
	None,
	Any,
	Fixed(std::iter::Peekable<std::iter::Copied<std::collections::btree_set::Iter<'a, usize>>>),
}

impl<'a> SubjectConstraints<'a> {
	fn new<R: Ord>(resources: &'a BTreeMap<R, Resource>, s: PatternSubject<&R>) -> Self {
		match s {
			PatternSubject::Any => Self::Any,
			PatternSubject::Given(s) => match resources.get(s) {
				Some(subject) => Self::Fixed(subject.as_subject.iter().copied().peekable()),
				None => Self::None,
			},
		}
	}

	fn next(&mut self, i: usize) -> Option<usize> {
		match self {
			Self::None => None,
			Self::Any => Some(i),
			Self::Fixed(indexes) => {
				while let Some(j) = indexes.peek().copied() {
					if j >= i {
						return Some(j);
					}

					indexes.next();
				}

				None
			}
		}
	}
}

enum PredicateConstraints<'a> {
	None,
	Any,
	SameAsSubject,
	Fixed(std::iter::Peekable<std::iter::Copied<std::collections::btree_set::Iter<'a, usize>>>),
}

impl<'a> PredicateConstraints<'a> {
	fn new<R: Ord>(resources: &'a BTreeMap<R, Resource>, p: PatternPredicate<&R>) -> Self {
		match p {
			PatternPredicate::Any => Self::Any,
			PatternPredicate::SameAsSubject => Self::SameAsSubject,
			PatternPredicate::Given(s) => match resources.get(s) {
				Some(subject) => Self::Fixed(subject.as_predicate.iter().copied().peekable()),
				None => Self::None,
			},
		}
	}

	fn next<R: PartialEq>(&mut self, i: usize, triple: &Triple<R>) -> Result<(), Option<usize>> {
		match self {
			Self::None => Err(None),
			Self::Any => Ok(()),
			Self::SameAsSubject => {
				if triple.0 == triple.1 {
					Ok(())
				} else {
					Err(i.checked_add(1))
				}
			}
			Self::Fixed(indexes) => {
				while let Some(j) = indexes.peek().copied() {
					match j.cmp(&i) {
						Ordering::Equal => return Ok(()),
						Ordering::Greater => return Err(Some(j)),
						Ordering::Less => {
							indexes.next();
						}
					}
				}

				Err(None)
			}
		}
	}
}

enum ObjectConstraints<'a> {
	None,
	Any,
	SameAsSubject,
	SameAsPredicate,
	Fixed(std::iter::Peekable<std::iter::Copied<std::collections::btree_set::Iter<'a, usize>>>),
}

impl<'a> ObjectConstraints<'a> {
	fn new<R: Ord>(resources: &'a BTreeMap<R, Resource>, p: PatternObject<&R>) -> Self {
		match p {
			PatternObject::Any => Self::Any,
			PatternObject::SameAsSubject => Self::SameAsSubject,
			PatternObject::SameAsPredicate => Self::SameAsPredicate,
			PatternObject::Given(s) => match resources.get(s) {
				Some(subject) => Self::Fixed(subject.as_object.iter().copied().peekable()),
				None => Self::None,
			},
		}
	}

	fn next<R: PartialEq>(&mut self, i: usize, triple: &Triple<R>) -> Result<(), Option<usize>> {
		match self {
			Self::None => Err(None),
			Self::Any => Ok(()),
			Self::SameAsSubject => {
				if triple.0 == triple.2 {
					Ok(())
				} else {
					Err(i.checked_add(1))
				}
			}
			Self::SameAsPredicate => {
				if triple.1 == triple.2 {
					Ok(())
				} else {
					Err(i.checked_add(1))
				}
			}
			Self::Fixed(indexes) => {
				while let Some(j) = indexes.peek().copied() {
					match j.cmp(&i) {
						Ordering::Equal => return Ok(()),
						Ordering::Greater => return Err(Some(j)),
						Ordering::Less => {
							indexes.next();
						}
					}
				}

				Err(None)
			}
		}
	}
}

#[derive(Default)]
struct Resource {
	as_subject: BTreeSet<usize>,
	as_predicate: BTreeSet<usize>,
	as_object: BTreeSet<usize>,
}

impl Resource {
	pub fn subject(i: usize) -> Self {
		Self {
			as_subject: std::iter::once(i).collect(),
			as_predicate: BTreeSet::new(),
			as_object: BTreeSet::new(),
		}
	}

	pub fn predicate(i: usize) -> Self {
		Self {
			as_subject: BTreeSet::new(),
			as_predicate: std::iter::once(i).collect(),
			as_object: BTreeSet::new(),
		}
	}

	pub fn object(i: usize) -> Self {
		Self {
			as_subject: BTreeSet::new(),
			as_predicate: BTreeSet::new(),
			as_object: std::iter::once(i).collect(),
		}
	}

	pub fn is_empty(&self) -> bool {
		self.as_subject.is_empty() && self.as_predicate.is_empty() && self.as_object.is_empty()
	}
}

#[cfg(test)]
mod tests {
	use rand::{rngs::SmallRng, RngCore, SeedableRng};

	use crate::Triple;

	use super::BTreeGraph;

	fn insert_test(n: usize, seed: [u8; 32]) {
		let mut rng = SmallRng::from_seed(seed);
		let mut triples = Vec::new();
		triples.resize_with(n, || Triple(rng.next_u32(), rng.next_u32(), rng.next_u32()));

		let mut graph = BTreeGraph::new();
		for &t in &triples {
			graph.insert(t);
		}

		triples.sort_unstable();
		triples.dedup();

		assert_eq!(graph.len(), triples.len());

		test_eq(graph, triples)
	}

	fn remove_test(n: usize, seed: [u8; 32]) {
		use rand::prelude::SliceRandom;
		let mut rng = SmallRng::from_seed(seed);
		let mut triples = Vec::new();
		triples.resize_with(n, || Triple(rng.next_u32(), rng.next_u32(), rng.next_u32()));

		let mut graph = BTreeGraph::new();
		for &t in &triples {
			graph.insert(t);
		}

		triples.shuffle(&mut rng);

		for _ in 0..(n / 2) {
			let t = triples.pop().unwrap();
			graph.remove(t.as_ref());
		}

		triples.sort_unstable();
		triples.dedup();

		test_eq(graph, triples)
	}

	fn test_eq(graph: BTreeGraph<u32>, triples: Vec<Triple<u32>>) {
		assert_eq!(graph.len(), triples.len());

		let mut a = triples.iter().copied();
		let mut b = graph.iter().map(Triple::into_copied);

		loop {
			match (a.next(), b.next()) {
				(Some(a), Some(b)) => assert_eq!(a, b),
				(None, None) => break,
				_ => panic!("different length"),
			}
		}
	}

	#[test]
	fn insert() {
		for i in 0u8..32 {
			insert_test(i as usize * 11, [i; 32]);
		}
	}

	#[test]
	fn remove() {
		for i in 0u8..32 {
			remove_test(i as usize * 11, [i; 32]);
		}
	}
}
