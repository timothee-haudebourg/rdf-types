use std::{cmp::Ordering, collections::BTreeSet, fmt::Debug, hash::Hash};

use educe::Educe;
use raw_btree::RawBTree;
use slab::Slab;

use super::super::{Graph, PatternMatchingGraph};
use crate::{
	dataset::{
		BTreeGraph, GraphMut, ObjectTraversableGraph, PredicateTraversableGraph,
		ResourceTraversableGraph, SubjectTraversableGraph, TraversableGraph,
	},
	pattern::{
		triple::canonical::{PatternObject, PatternPredicate, PatternSubject},
		CanonicalTriplePattern,
	},
	RdfDisplay, Term, Triple,
};

fn resource_cmp<R: Ord>(resources: &Slab<Resource<R>>) -> impl '_ + Fn(&usize, &R) -> Ordering {
	|&i, resource| resources[i].value.cmp(resource)
}

fn resource_index_cmp<R: Ord>(
	resources: &Slab<Resource<R>>,
) -> impl '_ + Fn(&usize, &usize) -> Ordering {
	|&i, &j| resources[i].value.cmp(&resources[j].value)
}

fn triple_with_resources<R>(
	resources: &Slab<Resource<R>>,
	Triple(s, p, o): Triple<usize>,
) -> Triple<&R> {
	Triple(
		&resources[s].value,
		&resources[p].value,
		&resources[o].value,
	)
}

fn triple_cmp<'a, R: Ord>(
	resources: &'a Slab<Resource<R>>,
	triples: &'a Slab<Triple<usize>>,
) -> impl 'a + Fn(&usize, &Triple<&R>) -> Ordering {
	|&i, triple| triple_with_resources(resources, triples[i]).cmp(triple)
}

fn triple_index_cmp<'a, R: Ord>(
	resources: &'a Slab<Resource<R>>,
	triples: &'a Slab<Triple<usize>>,
) -> impl 'a + Fn(&usize, &usize) -> Ordering {
	|&i, &j| {
		triple_with_resources(resources, triples[i])
			.cmp(&triple_with_resources(resources, triples[j]))
	}
}

/// Indexed BTree-based RDF graph, optimized for pattern matching operations.
#[derive(Clone)]
pub struct IndexedBTreeGraph<R = Term> {
	resources: Slab<Resource<R>>,
	triples: Slab<Triple<usize>>,
	resources_indexes: RawBTree<usize>,
	triples_indexes: RawBTree<usize>,
	subjects: BTreeSet<usize>,
	predicates: BTreeSet<usize>,
	objects: BTreeSet<usize>,
}

impl<R> Default for IndexedBTreeGraph<R> {
	fn default() -> Self {
		Self {
			triples: Slab::new(),
			resources: Slab::new(),
			triples_indexes: RawBTree::new(),
			resources_indexes: RawBTree::new(),
			subjects: BTreeSet::new(),
			predicates: BTreeSet::new(),
			objects: BTreeSet::new(),
		}
	}
}

impl<R> IndexedBTreeGraph<R> {
	/// Creates a new empty graph.
	pub fn new() -> Self {
		Self::default()
	}

	/// Creates a new indexed graph from a non-indexed one.
	pub fn from_non_indexed(graph: BTreeGraph<R>) -> Self {
		let mut resources: Slab<Resource<R>> = graph
			.resources
			.into_iter()
			.map(|(i, r)| {
				let indexed_r = Resource {
					value: r.value,
					as_subject: BTreeSet::new(),
					as_predicate: BTreeSet::new(),
					as_object: BTreeSet::new(),
				};

				(i, indexed_r)
			})
			.collect();

		let mut subjects = BTreeSet::new();
		let mut predicates = BTreeSet::new();
		let mut objects = BTreeSet::new();

		for &i in &graph.triples_indexes {
			let Triple(s, p, o) = graph.triples[i];

			resources[s].as_subject.insert(i);
			subjects.insert(s);

			resources[p].as_predicate.insert(i);
			predicates.insert(p);

			resources[o].as_object.insert(i);
			objects.insert(o);
		}

		Self {
			resources,
			triples: graph.triples,
			resources_indexes: graph.resources_indexes,
			triples_indexes: graph.triples_indexes,
			subjects,
			predicates,
			objects,
		}
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
			resources: &self.resources,
			triples: &self.triples,
			indexes: self.triples_indexes.iter(),
		}
	}

	/// Returns an iterator over the resources of the graph.
	pub fn resources(&self) -> Resources<R> {
		Resources {
			resources: &self.resources,
			indexes: self.resources_indexes.iter(),
		}
	}

	/// Returns an iterator over the subjects of the graph.
	pub fn subjects(&self) -> Subjects<R> {
		Subjects {
			resources: &self.resources,
			indexes: self.subjects.iter(),
		}
	}

	/// Returns an iterator over the predicates of the graph.
	pub fn predicates(&self) -> Predicates<R> {
		Predicates {
			resources: &self.resources,
			indexes: self.predicates.iter(),
		}
	}

	/// Returns an iterator over the objects of the graph.
	pub fn objects(&self) -> Objects<R> {
		Objects {
			resources: &self.resources,
			indexes: self.objects.iter(),
		}
	}
}

impl<R: Ord> IndexedBTreeGraph<R> {
	fn index_of_resource(&self, resource: &R) -> Option<usize> {
		self.resources_indexes
			.get(resource_cmp(&self.resources), resource)
			.copied()
	}

	fn get_resource(&self, resource: &R) -> Option<&Resource<R>> {
		self.resources.get(self.index_of_resource(resource)?)
	}

	/// Returns the index of the given triple in the graph, if any.
	fn index_of_triple(&self, triple: Triple<&R>) -> Option<usize> {
		self.triples_indexes
			.get(triple_cmp(&self.resources, &self.triples), &triple)
			.copied()
	}

	/// Checks if the provided resource appears in any triple in the graph.
	pub fn contains_resource(&self, resource: &R) -> bool {
		self.index_of_resource(resource).is_some()
	}

	/// Checks if the provided triple is in the graph.
	pub fn contains(&self, triple: Triple<&R>) -> bool {
		self.index_of_triple(triple).is_some()
	}

	/// Inserts the given triple in the graph.
	///
	/// Returns `true` if the triple was not already in the graph, and `false`
	/// if it was.
	pub fn insert(&mut self, triple: Triple<R>) -> bool {
		if self.contains(triple.as_ref()) {
			false
		} else {
			let s_i = self.index_of_resource(&triple.0);
			let p_i = self.index_of_resource(&triple.1);
			let o_i = self.index_of_resource(&triple.2);

			let e = self.triples.vacant_entry();
			let i = e.key();

			let s_i = match s_i {
				Some(s_i) => {
					self.resources[s_i].as_subject.insert(i);
					s_i
				}
				None => {
					let s_i = self.resources.insert(Resource::subject(triple.0, i));
					self.resources_indexes
						.insert(resource_index_cmp(&self.resources), s_i);
					s_i
				}
			};

			let p_i = match p_i {
				Some(p_i) => {
					self.resources[p_i].as_predicate.insert(i);
					p_i
				}
				None => {
					let p_i = self.resources.insert(Resource::predicate(triple.1, i));
					self.resources_indexes
						.insert(resource_index_cmp(&self.resources), p_i);
					p_i
				}
			};

			let o_i = match o_i {
				Some(o_i) => {
					self.resources[o_i].as_object.insert(i);
					o_i
				}
				None => {
					let o_i = self.resources.insert(Resource::object(triple.2, i));
					self.resources_indexes
						.insert(resource_index_cmp(&self.resources), o_i);
					o_i
				}
			};

			self.subjects.insert(s_i);
			self.predicates.insert(p_i);
			self.objects.insert(o_i);

			e.insert(Triple(s_i, p_i, o_i));

			self.triples_indexes
				.insert(triple_index_cmp(&self.resources, &self.triples), i);

			true
		}
	}

	/// Removes the given triple from the graph.
	///
	/// Returns whether or not the triple was in the graph.
	/// Does nothing if the triple was not in the graph.
	pub fn remove(&mut self, triple: Triple<&R>) -> bool {
		match self
			.triples_indexes
			.remove(triple_cmp(&self.resources, &self.triples), &triple)
		{
			Some(i) => {
				let Triple(s_i, p_i, o_i) = self.triples.remove(i);

				self.subjects.remove(&s_i);
				self.predicates.remove(&p_i);
				self.objects.remove(&o_i);

				let s = &mut self.resources[s_i];
				s.as_subject.remove(&i);
				if s.is_empty() {
					self.resources_indexes
						.remove(resource_cmp(&self.resources), triple.0);
					self.resources.remove(s_i);
				}

				let p = &mut self.resources[p_i];
				p.as_predicate.remove(&i);
				if p.is_empty() {
					self.resources_indexes
						.remove(resource_cmp(&self.resources), triple.1);
					self.resources.remove(p_i);
				}

				let o = &mut self.resources[o_i];
				o.as_object.remove(&i);
				if o.is_empty() {
					self.resources_indexes
						.remove(resource_cmp(&self.resources), triple.2);
					self.resources.remove(o_i);
				}

				true
			}
			None => false,
		}
	}

	/// Returns an iterator over all the triples matching the given canonical
	/// triple pattern.
	pub fn pattern_matching(&self, pattern: CanonicalTriplePattern<&R>) -> PatternMatching<R> {
		PatternMatching {
			resources: &self.resources,
			triples: &self.triples,
			subject: SubjectConstraints::new(self, pattern.into_subject()),
			predicate: PredicateConstraints::new(self, pattern.into_predicate()),
			object: ObjectConstraints::new(self, pattern.into_object()),
			i: 0,
		}
	}
}

impl<R> From<BTreeGraph<R>> for IndexedBTreeGraph<R> {
	fn from(value: BTreeGraph<R>) -> Self {
		Self::from_non_indexed(value)
	}
}

impl<R: Clone + Ord> FromIterator<Triple<R>> for IndexedBTreeGraph<R> {
	fn from_iter<T: IntoIterator<Item = Triple<R>>>(iter: T) -> Self {
		let mut result = Self::new();
		result.extend(iter);
		result
	}
}

impl<R: Clone + Ord> Extend<Triple<R>> for IndexedBTreeGraph<R> {
	fn extend<T: IntoIterator<Item = Triple<R>>>(&mut self, iter: T) {
		for triple in iter {
			self.insert(triple);
		}
	}
}

impl<R> Graph for IndexedBTreeGraph<R> {
	type Resource = R;
}

impl<R> TraversableGraph for IndexedBTreeGraph<R> {
	type Triples<'a> = Triples<'a, R> where R: 'a;

	fn triples(&self) -> Self::Triples<'_> {
		self.iter()
	}
}

impl<R> ResourceTraversableGraph for IndexedBTreeGraph<R> {
	type GraphResources<'a> = Resources<'a, R> where R: 'a;

	fn graph_resources(&self) -> Self::GraphResources<'_> {
		self.resources()
	}

	fn graph_resource_count(&self) -> usize {
		self.resources.len()
	}
}

impl<R> SubjectTraversableGraph for IndexedBTreeGraph<R> {
	type GraphSubjects<'a> = Subjects<'a, R> where R: 'a;

	fn graph_subjects(&self) -> Self::GraphSubjects<'_> {
		self.subjects()
	}

	fn graph_subject_count(&self) -> usize {
		self.subjects.len()
	}
}

impl<R> PredicateTraversableGraph for IndexedBTreeGraph<R> {
	type GraphPredicates<'a> = Predicates<'a, R> where R: 'a;

	fn graph_predicates(&self) -> Self::GraphPredicates<'_> {
		self.predicates()
	}

	fn graph_predicate_count(&self) -> usize {
		self.predicates.len()
	}
}

impl<R> ObjectTraversableGraph for IndexedBTreeGraph<R> {
	type GraphObjects<'a> = Objects<'a, R> where R: 'a;

	fn graph_objects(&self) -> Self::GraphObjects<'_> {
		self.objects()
	}

	fn graph_object_count(&self) -> usize {
		self.objects.len()
	}
}

impl<R: Clone + Ord> GraphMut for IndexedBTreeGraph<R> {
	fn insert(&mut self, triple: Triple<Self::Resource>) {
		self.insert(triple);
	}

	fn remove(&mut self, triple: Triple<&Self::Resource>) {
		self.remove(triple);
	}
}

impl<R: Ord> PatternMatchingGraph for IndexedBTreeGraph<R> {
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
#[derive(Educe)]
#[educe(Clone, Copy)]
pub struct Triples<'a, R> {
	resources: &'a Slab<Resource<R>>,
	triples: &'a Slab<Triple<usize>>,
	indexes: raw_btree::Iter<'a, usize>,
}

impl<'a, R> Iterator for Triples<'a, R> {
	type Item = Triple<&'a R>;

	fn next(&mut self) -> Option<Self::Item> {
		self.indexes
			.next()
			.map(|&i| triple_with_resources(self.resources, self.triples[i]))
	}
}

/// Iterator over the triples of a [`BTreeGraph`].
pub struct IntoTriples<R> {
	resources: Slab<Resource<R>>,
	triples: Slab<Triple<usize>>,
	indexes: raw_btree::IntoIter<usize>,
}

impl<R: Clone> Iterator for IntoTriples<R> {
	type Item = Triple<R>;

	fn next(&mut self) -> Option<Self::Item> {
		self.indexes
			.next()
			.map(|i| triple_with_resources(&self.resources, self.triples.remove(i)).cloned())
	}
}

impl<'a, R> IntoIterator for &'a IndexedBTreeGraph<R> {
	type Item = Triple<&'a R>;
	type IntoIter = Triples<'a, R>;

	fn into_iter(self) -> Self::IntoIter {
		self.iter()
	}
}

impl<R: Clone> IntoIterator for IndexedBTreeGraph<R> {
	type Item = Triple<R>;
	type IntoIter = IntoTriples<R>;

	fn into_iter(self) -> Self::IntoIter {
		IntoTriples {
			resources: self.resources,
			triples: self.triples,
			indexes: self.triples_indexes.into_iter(),
		}
	}
}

pub struct Resources<'a, R> {
	resources: &'a Slab<Resource<R>>,
	indexes: raw_btree::Iter<'a, usize>,
}

impl<'a, R> Iterator for Resources<'a, R> {
	type Item = &'a R;

	fn next(&mut self) -> Option<Self::Item> {
		self.indexes.next().map(|&i| &self.resources[i].value)
	}
}

pub struct Subjects<'a, R> {
	resources: &'a Slab<Resource<R>>,
	indexes: std::collections::btree_set::Iter<'a, usize>,
}

impl<'a, R> Iterator for Subjects<'a, R> {
	type Item = &'a R;

	fn next(&mut self) -> Option<Self::Item> {
		self.indexes.next().map(|&i| &self.resources[i].value)
	}
}

pub struct Predicates<'a, R> {
	resources: &'a Slab<Resource<R>>,
	indexes: std::collections::btree_set::Iter<'a, usize>,
}

impl<'a, R> Iterator for Predicates<'a, R> {
	type Item = &'a R;

	fn next(&mut self) -> Option<Self::Item> {
		self.indexes.next().map(|&i| &self.resources[i].value)
	}
}

pub struct Objects<'a, R> {
	resources: &'a Slab<Resource<R>>,
	indexes: std::collections::btree_set::Iter<'a, usize>,
}

impl<'a, R> Iterator for Objects<'a, R> {
	type Item = &'a R;

	fn next(&mut self) -> Option<Self::Item> {
		self.indexes.next().map(|&i| &self.resources[i].value)
	}
}

impl<R: PartialEq> PartialEq for IndexedBTreeGraph<R> {
	fn eq(&self, other: &Self) -> bool {
		self.len() == other.len() && self.iter().zip(other).all(|(a, b)| a == b)
	}
}

impl<R: Eq> Eq for IndexedBTreeGraph<R> {}

impl<R: PartialOrd> PartialOrd for IndexedBTreeGraph<R> {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		self.iter().partial_cmp(other)
	}
}

impl<R: Ord> Ord for IndexedBTreeGraph<R> {
	fn cmp(&self, other: &Self) -> Ordering {
		self.iter().cmp(other)
	}
}

impl<R: Hash> Hash for IndexedBTreeGraph<R> {
	fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
		state.write_usize(self.len());
		for elt in self {
			elt.hash(state);
		}
	}
}

/// Iterator over the triples of a [`BTreeGraph`] matching some given pattern.
pub struct PatternMatching<'a, R> {
	resources: &'a Slab<Resource<R>>,
	triples: &'a Slab<Triple<usize>>,
	subject: SubjectConstraints<'a>,
	predicate: PredicateConstraints<'a>,
	object: ObjectConstraints<'a>,
	i: usize,
}

impl<'a, R> Iterator for PatternMatching<'a, R> {
	type Item = Triple<&'a R>;

	fn next(&mut self) -> Option<Self::Item> {
		while self.i < self.triples.capacity() {
			let i = self.subject.next(self.i)?;
			let triple = *self.triples.get(i)?;
			match self.predicate.next(i, triple) {
				Ok(()) => match self.object.next(i, triple) {
					Ok(()) => {
						self.i = i + 1;
						return Some(triple_with_resources(self.resources, triple));
					}
					Err(j) => self.i = j?,
				},
				Err(j) => self.i = j?,
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
	fn new<R: Ord>(graph: &'a IndexedBTreeGraph<R>, s: PatternSubject<&R>) -> Self {
		match s {
			PatternSubject::Any => Self::Any,
			PatternSubject::Given(s) => match graph.get_resource(s) {
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
	fn new<R: Ord>(graph: &'a IndexedBTreeGraph<R>, p: PatternPredicate<&R>) -> Self {
		match p {
			PatternPredicate::Any => Self::Any,
			PatternPredicate::SameAsSubject => Self::SameAsSubject,
			PatternPredicate::Given(s) => match graph.get_resource(s) {
				Some(subject) => Self::Fixed(subject.as_predicate.iter().copied().peekable()),
				None => Self::None,
			},
		}
	}

	fn next(&mut self, i: usize, triple: Triple<usize>) -> Result<(), Option<usize>> {
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
	fn new<R: Ord>(graph: &'a IndexedBTreeGraph<R>, p: PatternObject<&R>) -> Self {
		match p {
			PatternObject::Any => Self::Any,
			PatternObject::SameAsSubject => Self::SameAsSubject,
			PatternObject::SameAsPredicate => Self::SameAsPredicate,
			PatternObject::Given(s) => match graph.get_resource(s) {
				Some(subject) => Self::Fixed(subject.as_object.iter().copied().peekable()),
				None => Self::None,
			},
		}
	}

	fn next(&mut self, i: usize, triple: Triple<usize>) -> Result<(), Option<usize>> {
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

#[derive(Default, Clone)]
pub(crate) struct Resource<R> {
	pub value: R,
	pub as_subject: BTreeSet<usize>,
	pub as_predicate: BTreeSet<usize>,
	pub as_object: BTreeSet<usize>,
}

impl<R> Resource<R> {
	pub fn subject(value: R, i: usize) -> Self {
		Self {
			value,
			as_subject: std::iter::once(i).collect(),
			as_predicate: BTreeSet::new(),
			as_object: BTreeSet::new(),
		}
	}

	pub fn predicate(value: R, i: usize) -> Self {
		Self {
			value,
			as_subject: BTreeSet::new(),
			as_predicate: std::iter::once(i).collect(),
			as_object: BTreeSet::new(),
		}
	}

	pub fn object(value: R, i: usize) -> Self {
		Self {
			value,
			as_subject: BTreeSet::new(),
			as_predicate: BTreeSet::new(),
			as_object: std::iter::once(i).collect(),
		}
	}

	pub fn is_empty(&self) -> bool {
		self.as_subject.is_empty() && self.as_predicate.is_empty() && self.as_object.is_empty()
	}
}

impl<R: Debug> Debug for IndexedBTreeGraph<R> {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_set().entries(self.iter()).finish()
	}
}

impl<R: RdfDisplay> RdfDisplay for IndexedBTreeGraph<R> {
	fn rdf_fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		for t in self {
			writeln!(f, "{} .", t.rdf_display())?;
		}

		Ok(())
	}
}

#[cfg(feature = "serde")]
impl<R: serde::Serialize> serde::Serialize for IndexedBTreeGraph<R> {
	fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
		use serde::ser::SerializeSeq;
		let mut seq = serializer.serialize_seq(Some(self.len()))?;

		for triple in self {
			seq.serialize_element(&triple)?;
		}

		seq.end()
	}
}

#[cfg(feature = "serde")]
impl<'de, R: Clone + Ord + serde::Deserialize<'de>> serde::Deserialize<'de>
	for IndexedBTreeGraph<R>
{
	fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
		struct Visitor<R>(std::marker::PhantomData<R>);

		impl<'de, R: Clone + Ord + serde::Deserialize<'de>> serde::de::Visitor<'de> for Visitor<R> {
			type Value = IndexedBTreeGraph<R>;

			fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
				write!(formatter, "an RDF graph")
			}

			fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
			where
				A: serde::de::SeqAccess<'de>,
			{
				let mut result = IndexedBTreeGraph::new();

				while let Some(triple) = seq.next_element()? {
					result.insert(triple);
				}

				Ok(result)
			}
		}

		deserializer.deserialize_seq(Visitor(std::marker::PhantomData))
	}
}

#[cfg(test)]
mod tests {
	use rand::{rngs::SmallRng, RngCore, SeedableRng};

	use crate::Triple;

	use super::IndexedBTreeGraph;

	fn insert_test(n: usize, seed: [u8; 32]) {
		let mut rng = SmallRng::from_seed(seed);
		let mut triples = Vec::new();
		triples.resize_with(n, || Triple(rng.next_u32(), rng.next_u32(), rng.next_u32()));

		let mut graph = IndexedBTreeGraph::new();
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

		let mut graph = IndexedBTreeGraph::new();
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

	fn test_eq(graph: IndexedBTreeGraph<u32>, triples: Vec<Triple<u32>>) {
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
