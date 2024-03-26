use std::{cmp::Ordering, collections::BTreeSet, fmt::Debug, hash::Hash};

use educe::Educe;
use raw_btree::RawBTree;
use slab::Slab;

use super::{
	super::{Dataset, PatternMatchingDataset},
	BTreeDataset,
};
use crate::{
	dataset::{
		BTreeGraph, DatasetMut, NamedGraphTraversableDataset, ObjectTraversableDataset,
		PredicateTraversableDataset, ResourceTraversableDataset, SubjectTraversableDataset,
		TraversableDataset,
	},
	pattern::{
		quad::canonical::{PatternGraph, PatternObject, PatternPredicate, PatternSubject},
		CanonicalQuadPattern,
	},
	Quad, RdfDisplay, Term,
};

fn resource_cmp<R: Ord>(resources: &Slab<Resource<R>>) -> impl '_ + Fn(&usize, &R) -> Ordering {
	|&i, resource| resources[i].value.cmp(resource)
}

fn resource_index_cmp<R: Ord>(
	resources: &Slab<Resource<R>>,
) -> impl '_ + Fn(&usize, &usize) -> Ordering {
	|&i, &j| resources[i].value.cmp(&resources[j].value)
}

fn quad_with_resources<R>(
	resources: &Slab<Resource<R>>,
	Quad(s, p, o, g): Quad<usize>,
) -> Quad<&R> {
	Quad(
		&resources[s].value,
		&resources[p].value,
		&resources[o].value,
		g.map(|g| &resources[g].value),
	)
}

fn quad_cmp<'a, R: Ord>(
	resources: &'a Slab<Resource<R>>,
	quads: &'a Slab<Quad<usize>>,
) -> impl 'a + Fn(&usize, &Quad<&R>) -> Ordering {
	|&i, quad| quad_with_resources(resources, quads[i]).cmp(quad)
}

fn quad_index_cmp<'a, R: Ord>(
	resources: &'a Slab<Resource<R>>,
	quads: &'a Slab<Quad<usize>>,
) -> impl 'a + Fn(&usize, &usize) -> Ordering {
	|&i, &j| quad_with_resources(resources, quads[i]).cmp(&quad_with_resources(resources, quads[j]))
}

/// Indexed BTree-based RDF dataset, optimized for pattern matching operations.
#[derive(Clone)]
pub struct IndexedBTreeDataset<R = Term> {
	resources: Slab<Resource<R>>,
	quads: Slab<Quad<usize>>,
	resources_indexes: RawBTree<usize>,
	quads_indexes: RawBTree<usize>,
	subjects: BTreeSet<usize>,
	predicates: BTreeSet<usize>,
	objects: BTreeSet<usize>,
	default_graph: BTreeSet<usize>,
	named_graphs: BTreeSet<usize>,
}

impl<R> Default for IndexedBTreeDataset<R> {
	fn default() -> Self {
		Self {
			quads: Slab::new(),
			resources: Slab::new(),
			quads_indexes: RawBTree::new(),
			resources_indexes: RawBTree::new(),
			default_graph: BTreeSet::new(),
			subjects: BTreeSet::new(),
			predicates: BTreeSet::new(),
			objects: BTreeSet::new(),
			named_graphs: BTreeSet::new(),
		}
	}
}

impl<R> IndexedBTreeDataset<R> {
	/// Creates a new empty dataset.
	pub fn new() -> Self {
		Self::default()
	}

	/// Creates a new indexed dataset from a non-indexed one.
	pub fn from_non_indexed(dataset: BTreeDataset<R>) -> Self {
		let mut resources: Slab<Resource<R>> = dataset
			.resources
			.into_iter()
			.map(|(i, r)| {
				let indexed_r = Resource {
					value: r.value,
					as_subject: BTreeSet::new(),
					as_predicate: BTreeSet::new(),
					as_object: BTreeSet::new(),
					as_graph: BTreeSet::new(),
				};

				(i, indexed_r)
			})
			.collect();

		let mut subjects = BTreeSet::new();
		let mut predicates = BTreeSet::new();
		let mut objects = BTreeSet::new();
		let mut named_graphs = BTreeSet::new();
		let mut default_graph = BTreeSet::new();

		for &i in &dataset.quads_indexes {
			let Quad(s, p, o, g) = dataset.quads[i];

			resources[s].as_subject.insert(i);
			subjects.insert(s);

			resources[p].as_predicate.insert(i);
			predicates.insert(p);

			resources[o].as_object.insert(i);
			objects.insert(o);

			match g {
				Some(g) => {
					resources[g].as_graph.insert(i);
					named_graphs.insert(g);
				}
				None => {
					default_graph.insert(i);
				}
			}
		}

		Self {
			resources,
			quads: dataset.quads,
			resources_indexes: dataset.resources_indexes,
			quads_indexes: dataset.quads_indexes,
			default_graph,
			subjects,
			predicates,
			objects,
			named_graphs,
		}
	}

	/// Returns the number of quads in the dataset.
	pub fn len(&self) -> usize {
		self.quads.len()
	}

	/// Checks if the dataset is empty.
	pub fn is_empty(&self) -> bool {
		self.quads.is_empty()
	}

	/// Returns an iterator over the quads of the dataset.
	pub fn iter(&self) -> Quads<R> {
		Quads {
			resources: &self.resources,
			quads: &self.quads,
			indexes: self.quads_indexes.iter(),
		}
	}

	/// Returns an iterator over the resources of the dataset.
	pub fn resources(&self) -> Resources<R> {
		Resources {
			resources: &self.resources,
			indexes: self.resources_indexes.iter(),
		}
	}

	/// Returns an iterator over the subjects of the dataset.
	pub fn subjects(&self) -> Subjects<R> {
		Subjects {
			resources: &self.resources,
			indexes: self.subjects.iter(),
		}
	}

	/// Returns an iterator over the predicates of the dataset.
	pub fn predicates(&self) -> Predicates<R> {
		Predicates {
			resources: &self.resources,
			indexes: self.predicates.iter(),
		}
	}

	/// Returns an iterator over the objects of the dataset.
	pub fn objects(&self) -> Objects<R> {
		Objects {
			resources: &self.resources,
			indexes: self.objects.iter(),
		}
	}

	/// Returns an iterator over the named graphs of the dataset.
	pub fn named_graphs(&self) -> NamedGraphs<R> {
		NamedGraphs {
			resources: &self.resources,
			indexes: self.named_graphs.iter(),
		}
	}
}

impl<R: Ord> IndexedBTreeDataset<R> {
	fn index_of_resource(&self, resource: &R) -> Option<usize> {
		self.resources_indexes
			.get(resource_cmp(&self.resources), resource)
			.copied()
	}

	fn get_resource(&self, resource: &R) -> Option<&Resource<R>> {
		self.resources.get(self.index_of_resource(resource)?)
	}

	/// Returns the index of the given quad in the dataset, if any.
	fn index_of_quad(&self, quad: Quad<&R>) -> Option<usize> {
		self.quads_indexes
			.get(quad_cmp(&self.resources, &self.quads), &quad)
			.copied()
	}

	/// Checks if the provided resource appears in any quad in the dataset.
	pub fn contains_resource(&self, resource: &R) -> bool {
		self.index_of_resource(resource).is_some()
	}

	/// Checks if the provided quad is in the dataset.
	pub fn contains(&self, quad: Quad<&R>) -> bool {
		self.index_of_quad(quad).is_some()
	}

	/// Inserts the given quad in the dataset.
	///
	/// Returns `true` if the quad was not already in the dataset, and `false`
	/// if it was.
	pub fn insert(&mut self, quad: Quad<R>) -> bool {
		if self.contains(quad.as_ref()) {
			false
		} else {
			let s_i = self.index_of_resource(&quad.0);
			let p_i = self.index_of_resource(&quad.1);
			let o_i = self.index_of_resource(&quad.2);
			let g_i = quad.3.map(|g| {
				let g_i = self.index_of_resource(&g);
				(g, g_i)
			});

			let e = self.quads.vacant_entry();
			let i = e.key();

			let s_i = match s_i {
				Some(s_i) => {
					self.resources[s_i].as_subject.insert(i);
					s_i
				}
				None => {
					let s_i = self.resources.insert(Resource::subject(quad.0, i));
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
					let p_i = self.resources.insert(Resource::predicate(quad.1, i));
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
					let o_i = self.resources.insert(Resource::object(quad.2, i));
					self.resources_indexes
						.insert(resource_index_cmp(&self.resources), o_i);
					o_i
				}
			};

			let g_i = match g_i {
				Some((_, Some(g_i))) => {
					self.resources[g_i].as_graph.insert(i);
					Some(g_i)
				}
				Some((g, None)) => {
					let g_i = self.resources.insert(Resource::graph(g, i));
					self.resources_indexes
						.insert(resource_index_cmp(&self.resources), g_i);
					Some(g_i)
				}
				None => {
					self.default_graph.insert(i);
					None
				}
			};

			self.subjects.insert(s_i);
			self.predicates.insert(p_i);
			self.objects.insert(o_i);
			if let Some(g_i) = g_i {
				self.named_graphs.insert(g_i);
			}

			e.insert(Quad(s_i, p_i, o_i, g_i));

			self.quads_indexes
				.insert(quad_index_cmp(&self.resources, &self.quads), i);

			true
		}
	}

	/// Removes the given quad from the dataset.
	///
	/// Returns whether or not the quad was in the dataset.
	/// Does nothing if the quad was not in the dataset.
	pub fn remove(&mut self, quad: Quad<&R>) -> bool {
		match self
			.quads_indexes
			.remove(quad_cmp(&self.resources, &self.quads), &quad)
		{
			Some(i) => {
				self.remove_by_index(i, false);
				true
			}
			None => false,
		}
	}

	/// Removes the given graph from the dataset if it exists, and returns it.
	pub fn remove_graph(&mut self, graph: Option<&R>) -> Option<BTreeGraph<R>>
	where
		R: Clone,
	{
		let indexes: Vec<usize> = match graph {
			Some(g) => {
				let g_i = self.index_of_resource(g)?;
				if self.named_graphs.contains(&g_i) {
					self.resources[g_i].as_graph.iter().copied().collect()
				} else {
					return None;
				}
			}
			None => self.default_graph.iter().copied().collect(),
		};

		let mut graph = BTreeGraph::new();
		for i in indexes {
			let quad = quad_with_resources(&self.resources, self.quads[i]).cloned();
			self.remove_by_index(i, true);
			graph.insert(quad.into_triple().0); // TODO: could be optimized
		}

		Some(graph)
	}

	fn remove_by_index(&mut self, i: usize, remove_index: bool) {
		if remove_index {
			self.quads_indexes
				.remove(quad_index_cmp(&self.resources, &self.quads), &i);
		}

		let Quad(s_i, p_i, o_i, g_i) = self.quads.remove(i);

		self.subjects.remove(&s_i);
		self.predicates.remove(&p_i);
		self.objects.remove(&o_i);

		let s = &mut self.resources[s_i];
		s.as_subject.remove(&i);
		if s.is_empty() {
			self.resources_indexes
				.remove(resource_index_cmp(&self.resources), &s_i);
			self.resources.remove(s_i);
		}

		let p = &mut self.resources[p_i];
		p.as_predicate.remove(&i);
		if p.is_empty() {
			self.resources_indexes
				.remove(resource_index_cmp(&self.resources), &p_i);
			self.resources.remove(p_i);
		}

		let o = &mut self.resources[o_i];
		o.as_object.remove(&i);
		if o.is_empty() {
			self.resources_indexes
				.remove(resource_index_cmp(&self.resources), &o_i);
			self.resources.remove(o_i);
		}

		match g_i {
			Some(g_i) => {
				let g = &mut self.resources[g_i];
				g.as_graph.remove(&i);
				if g.is_empty() {
					self.resources_indexes
						.remove(resource_index_cmp(&self.resources), &g_i);
					self.resources.remove(g_i);
					self.named_graphs.remove(&g_i);
				}
			}
			None => {
				self.default_graph.remove(&i);
			}
		}
	}

	/// Returns an iterator over all the quads matching the given canonical
	/// quad pattern.
	pub fn pattern_matching(&self, pattern: CanonicalQuadPattern<&R>) -> PatternMatching<R> {
		PatternMatching {
			resources: &self.resources,
			quads: &self.quads,
			subject: SubjectConstraints::new(self, pattern.into_subject()),
			predicate: PredicateConstraints::new(self, pattern.into_predicate()),
			object: ObjectConstraints::new(self, pattern.into_object()),
			graph: GraphConstraints::new(self, pattern.into_graph()),
			i: 0,
		}
	}

	/// Returns an iterator over all the quads matching the given canonical
	/// quad pattern. The matching quads are removed from the dataset.
	pub fn extract_pattern_matching(
		&mut self,
		pattern: CanonicalQuadPattern<&R>,
	) -> ExtractPatternMatching<R> {
		let subject = SubjectConstraints::new_owned(self, pattern.into_subject());
		let predicate = PredicateConstraints::new_owned(self, pattern.into_predicate());
		let object = ObjectConstraints::new_owned(self, pattern.into_object());
		let graph = GraphConstraints::new_owned(self, pattern.into_graph());

		ExtractPatternMatching {
			dataset: self,
			subject,
			predicate,
			object,
			graph,
			i: 0,
		}
	}
}

impl From<BTreeDataset> for IndexedBTreeDataset {
	fn from(value: BTreeDataset) -> Self {
		Self::from_non_indexed(value)
	}
}

impl<R: Clone + Ord> FromIterator<Quad<R>> for IndexedBTreeDataset<R> {
	fn from_iter<T: IntoIterator<Item = Quad<R>>>(iter: T) -> Self {
		let mut result = Self::new();
		result.extend(iter);
		result
	}
}

impl<R: Clone + Ord> Extend<Quad<R>> for IndexedBTreeDataset<R> {
	fn extend<T: IntoIterator<Item = Quad<R>>>(&mut self, iter: T) {
		for quad in iter {
			self.insert(quad);
		}
	}
}

impl<R> Dataset for IndexedBTreeDataset<R> {
	type Resource = R;
}

impl<R> TraversableDataset for IndexedBTreeDataset<R> {
	type Quads<'a> = Quads<'a, R> where R: 'a;

	fn quads(&self) -> Self::Quads<'_> {
		self.iter()
	}
}

impl<R> ResourceTraversableDataset for IndexedBTreeDataset<R> {
	type Resources<'a> = Resources<'a, R> where R: 'a;

	fn resources(&self) -> Self::Resources<'_> {
		self.resources()
	}

	fn resource_count(&self) -> usize {
		self.resources.len()
	}
}

impl<R> SubjectTraversableDataset for IndexedBTreeDataset<R> {
	type Subjects<'a> = Subjects<'a, R> where R: 'a;

	fn subjects(&self) -> Self::Subjects<'_> {
		self.subjects()
	}

	fn subject_count(&self) -> usize {
		self.subjects.len()
	}
}

impl<R> PredicateTraversableDataset for IndexedBTreeDataset<R> {
	type Predicates<'a> = Predicates<'a, R> where R: 'a;

	fn predicates(&self) -> Self::Predicates<'_> {
		self.predicates()
	}

	fn predicate_count(&self) -> usize {
		self.predicates.len()
	}
}

impl<R> ObjectTraversableDataset for IndexedBTreeDataset<R> {
	type Objects<'a> = Objects<'a, R> where R: 'a;

	fn objects(&self) -> Self::Objects<'_> {
		self.objects()
	}

	fn object_count(&self) -> usize {
		self.objects.len()
	}
}

impl<R> NamedGraphTraversableDataset for IndexedBTreeDataset<R> {
	type NamedGraphs<'a> = NamedGraphs<'a, R> where R: 'a;

	fn named_graphs(&self) -> Self::NamedGraphs<'_> {
		self.named_graphs()
	}

	fn named_graph_count(&self) -> usize {
		self.named_graphs.len()
	}
}

impl<R: Clone + Ord> DatasetMut for IndexedBTreeDataset<R> {
	fn insert(&mut self, quad: Quad<Self::Resource>) {
		self.insert(quad);
	}

	fn remove(&mut self, quad: Quad<&Self::Resource>) {
		self.remove(quad);
	}
}

impl<R: Ord> PatternMatchingDataset for IndexedBTreeDataset<R> {
	type QuadPatternMatching<'a, 'p> = PatternMatching<'a, R> where R: 'a, Self::Resource: 'p;

	fn quad_pattern_matching<'p>(
		&self,
		pattern: CanonicalQuadPattern<&'p Self::Resource>,
	) -> Self::QuadPatternMatching<'_, 'p> {
		self.pattern_matching(pattern)
	}

	fn contains_quad(&self, quad: Quad<&Self::Resource>) -> bool {
		self.contains(quad)
	}
}

/// Iterator over the quads of a [`BTreeGraph`].
#[derive(Educe)]
#[educe(Clone, Copy)]
pub struct Quads<'a, R> {
	resources: &'a Slab<Resource<R>>,
	quads: &'a Slab<Quad<usize>>,
	indexes: raw_btree::Iter<'a, usize>,
}

impl<'a, R> Iterator for Quads<'a, R> {
	type Item = Quad<&'a R>;

	fn next(&mut self) -> Option<Self::Item> {
		self.indexes
			.next()
			.map(|&i| quad_with_resources(self.resources, self.quads[i]))
	}
}

/// Iterator over the quads of a [`BTreeGraph`].
pub struct IntoTriples<R> {
	resources: Slab<Resource<R>>,
	quads: Slab<Quad<usize>>,
	indexes: raw_btree::IntoIter<usize>,
}

impl<R: Clone> Iterator for IntoTriples<R> {
	type Item = Quad<R>;

	fn next(&mut self) -> Option<Self::Item> {
		self.indexes
			.next()
			.map(|i| quad_with_resources(&self.resources, self.quads.remove(i)).cloned())
	}
}

impl<'a, R> IntoIterator for &'a IndexedBTreeDataset<R> {
	type Item = Quad<&'a R>;
	type IntoIter = Quads<'a, R>;

	fn into_iter(self) -> Self::IntoIter {
		self.iter()
	}
}

impl<R: Clone> IntoIterator for IndexedBTreeDataset<R> {
	type Item = Quad<R>;
	type IntoIter = IntoTriples<R>;

	fn into_iter(self) -> Self::IntoIter {
		IntoTriples {
			resources: self.resources,
			quads: self.quads,
			indexes: self.quads_indexes.into_iter(),
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

pub struct NamedGraphs<'a, R> {
	resources: &'a Slab<Resource<R>>,
	indexes: std::collections::btree_set::Iter<'a, usize>,
}

impl<'a, R> Iterator for NamedGraphs<'a, R> {
	type Item = &'a R;

	fn next(&mut self) -> Option<Self::Item> {
		self.indexes.next().map(|&i| &self.resources[i].value)
	}
}

impl<R: PartialEq> PartialEq for IndexedBTreeDataset<R> {
	fn eq(&self, other: &Self) -> bool {
		self.len() == other.len() && self.iter().zip(other).all(|(a, b)| a == b)
	}
}

impl<R: Eq> Eq for IndexedBTreeDataset<R> {}

impl<R: PartialOrd> PartialOrd for IndexedBTreeDataset<R> {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		self.iter().partial_cmp(other)
	}
}

impl<R: Ord> Ord for IndexedBTreeDataset<R> {
	fn cmp(&self, other: &Self) -> Ordering {
		self.iter().cmp(other)
	}
}

impl<R: Hash> Hash for IndexedBTreeDataset<R> {
	fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
		state.write_usize(self.len());
		for elt in self {
			elt.hash(state);
		}
	}
}

/// Iterator over the quads of a [`BTreeGraph`] matching some given pattern.
pub struct PatternMatching<'a, R> {
	resources: &'a Slab<Resource<R>>,
	quads: &'a Slab<Quad<usize>>,
	subject: SubjectConstraints<TripleIndexes<'a>>,
	predicate: PredicateConstraints<TripleIndexes<'a>>,
	object: ObjectConstraints<TripleIndexes<'a>>,
	graph: GraphConstraints<TripleIndexes<'a>>,
	i: usize,
}

impl<'a, R> Iterator for PatternMatching<'a, R> {
	type Item = Quad<&'a R>;

	fn next(&mut self) -> Option<Self::Item> {
		while self.i < self.quads.capacity() {
			let i = self.subject.next(self.i)?;
			let quad = *self.quads.get(i)?;
			match self.predicate.next(i, quad) {
				Ok(()) => match self.object.next(i, quad) {
					Ok(()) => match self.graph.next(i, quad) {
						Ok(()) => {
							self.i = i + 1;
							return Some(quad_with_resources(self.resources, quad));
						}
						Err(j) => self.i = j?,
					},
					Err(j) => self.i = j?,
				},
				Err(j) => self.i = j?,
			}
		}

		None
	}
}

/// Iterator over the quads of a [`BTreeGraph`] matching some given pattern.
pub struct ExtractPatternMatching<'a, R> {
	dataset: &'a mut IndexedBTreeDataset<R>,
	subject: SubjectConstraints<OwnedTripleIndexes>,
	predicate: PredicateConstraints<OwnedTripleIndexes>,
	object: ObjectConstraints<OwnedTripleIndexes>,
	graph: GraphConstraints<OwnedTripleIndexes>,
	i: usize,
}

impl<'a, R: Clone + Ord> Iterator for ExtractPatternMatching<'a, R> {
	type Item = Quad<R>;

	fn next(&mut self) -> Option<Self::Item> {
		while self.i < self.dataset.quads.capacity() {
			let i = self.subject.next(self.i)?;
			let quad = *self.dataset.quads.get(i)?;
			match self.predicate.next(i, quad) {
				Ok(()) => match self.object.next(i, quad) {
					Ok(()) => match self.graph.next(i, quad) {
						Ok(()) => {
							let value = quad_with_resources(&self.dataset.resources, quad).cloned();
							self.dataset.remove_by_index(i, true);
							self.i = i + 1;
							return Some(value);
						}
						Err(j) => self.i = j?,
					},
					Err(j) => self.i = j?,
				},
				Err(j) => self.i = j?,
			}
		}

		None
	}
}

type TripleIndexes<'a> = std::iter::Copied<std::collections::btree_set::Iter<'a, usize>>;
type OwnedTripleIndexes = std::vec::IntoIter<usize>;

enum SubjectConstraints<I: Iterator> {
	None,
	Any,
	Fixed(std::iter::Peekable<I>),
}

impl<'a> SubjectConstraints<TripleIndexes<'a>> {
	fn new<R: Ord>(dataset: &'a IndexedBTreeDataset<R>, s: PatternSubject<&R>) -> Self {
		match s {
			PatternSubject::Any => Self::Any,
			PatternSubject::Given(s) => match dataset.get_resource(s) {
				Some(subject) => Self::Fixed(subject.as_subject.iter().copied().peekable()),
				None => Self::None,
			},
		}
	}
}

impl SubjectConstraints<OwnedTripleIndexes> {
	fn new_owned<R: Ord>(dataset: &IndexedBTreeDataset<R>, s: PatternSubject<&R>) -> Self {
		match s {
			PatternSubject::Any => Self::Any,
			PatternSubject::Given(s) => match dataset.get_resource(s) {
				Some(subject) => Self::Fixed(
					subject
						.as_subject
						.iter()
						.copied()
						.collect::<Vec<_>>()
						.into_iter()
						.peekable(),
				),
				None => Self::None,
			},
		}
	}
}

impl<I: Iterator<Item = usize>> SubjectConstraints<I> {
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

enum PredicateConstraints<I: Iterator> {
	None,
	Any,
	SameAsSubject,
	Fixed(std::iter::Peekable<I>),
}

impl<'a> PredicateConstraints<TripleIndexes<'a>> {
	fn new<R: Ord>(dataset: &'a IndexedBTreeDataset<R>, p: PatternPredicate<&R>) -> Self {
		match p {
			PatternPredicate::Any => Self::Any,
			PatternPredicate::SameAsSubject => Self::SameAsSubject,
			PatternPredicate::Given(s) => match dataset.get_resource(s) {
				Some(subject) => Self::Fixed(subject.as_predicate.iter().copied().peekable()),
				None => Self::None,
			},
		}
	}
}

impl PredicateConstraints<OwnedTripleIndexes> {
	fn new_owned<R: Ord>(dataset: &IndexedBTreeDataset<R>, p: PatternPredicate<&R>) -> Self {
		match p {
			PatternPredicate::Any => Self::Any,
			PatternPredicate::SameAsSubject => Self::SameAsSubject,
			PatternPredicate::Given(s) => match dataset.get_resource(s) {
				Some(subject) => Self::Fixed(
					subject
						.as_predicate
						.iter()
						.copied()
						.collect::<Vec<_>>()
						.into_iter()
						.peekable(),
				),
				None => Self::None,
			},
		}
	}
}

impl<I: Iterator<Item = usize>> PredicateConstraints<I> {
	fn next(&mut self, i: usize, quad: Quad<usize>) -> Result<(), Option<usize>> {
		match self {
			Self::None => Err(None),
			Self::Any => Ok(()),
			Self::SameAsSubject => {
				if quad.0 == quad.1 {
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

enum ObjectConstraints<I: Iterator> {
	None,
	Any,
	SameAsSubject,
	SameAsPredicate,
	Fixed(std::iter::Peekable<I>),
}

impl<'a> ObjectConstraints<TripleIndexes<'a>> {
	fn new<R: Ord>(dataset: &'a IndexedBTreeDataset<R>, p: PatternObject<&R>) -> Self {
		match p {
			PatternObject::Any => Self::Any,
			PatternObject::SameAsSubject => Self::SameAsSubject,
			PatternObject::SameAsPredicate => Self::SameAsPredicate,
			PatternObject::Given(s) => match dataset.get_resource(s) {
				Some(subject) => Self::Fixed(subject.as_object.iter().copied().peekable()),
				None => Self::None,
			},
		}
	}
}

impl ObjectConstraints<OwnedTripleIndexes> {
	fn new_owned<R: Ord>(dataset: &IndexedBTreeDataset<R>, p: PatternObject<&R>) -> Self {
		match p {
			PatternObject::Any => Self::Any,
			PatternObject::SameAsSubject => Self::SameAsSubject,
			PatternObject::SameAsPredicate => Self::SameAsPredicate,
			PatternObject::Given(s) => match dataset.get_resource(s) {
				Some(subject) => Self::Fixed(
					subject
						.as_object
						.iter()
						.copied()
						.collect::<Vec<_>>()
						.into_iter()
						.peekable(),
				),
				None => Self::None,
			},
		}
	}
}

impl<I: Iterator<Item = usize>> ObjectConstraints<I> {
	fn next(&mut self, i: usize, quad: Quad<usize>) -> Result<(), Option<usize>> {
		match self {
			Self::None => Err(None),
			Self::Any => Ok(()),
			Self::SameAsSubject => {
				if quad.0 == quad.2 {
					Ok(())
				} else {
					Err(i.checked_add(1))
				}
			}
			Self::SameAsPredicate => {
				if quad.1 == quad.2 {
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

enum GraphConstraints<I: Iterator> {
	None,
	Any,
	SameAsSubject,
	SameAsPredicate,
	SameAsObject,
	Fixed(std::iter::Peekable<I>),
}

impl<'a> GraphConstraints<TripleIndexes<'a>> {
	fn new<R: Ord>(dataset: &'a IndexedBTreeDataset<R>, g: PatternGraph<&R>) -> Self {
		match g {
			PatternGraph::Any => Self::Any,
			PatternGraph::SameAsSubject => Self::SameAsSubject,
			PatternGraph::SameAsPredicate => Self::SameAsPredicate,
			PatternGraph::SameAsObject => Self::SameAsObject,
			PatternGraph::Given(Some(s)) => match dataset.get_resource(s) {
				Some(subject) => Self::Fixed(subject.as_graph.iter().copied().peekable()),
				None => Self::None,
			},
			PatternGraph::Given(None) => {
				Self::Fixed(dataset.default_graph.iter().copied().peekable())
			}
		}
	}
}

impl GraphConstraints<OwnedTripleIndexes> {
	fn new_owned<R: Ord>(dataset: &IndexedBTreeDataset<R>, g: PatternGraph<&R>) -> Self {
		match g {
			PatternGraph::Any => Self::Any,
			PatternGraph::SameAsSubject => Self::SameAsSubject,
			PatternGraph::SameAsPredicate => Self::SameAsPredicate,
			PatternGraph::SameAsObject => Self::SameAsObject,
			PatternGraph::Given(Some(s)) => match dataset.get_resource(s) {
				Some(subject) => Self::Fixed(
					subject
						.as_graph
						.iter()
						.copied()
						.collect::<Vec<_>>()
						.into_iter()
						.peekable(),
				),
				None => Self::None,
			},
			PatternGraph::Given(None) => Self::Fixed(
				dataset
					.default_graph
					.iter()
					.copied()
					.collect::<Vec<_>>()
					.into_iter()
					.peekable(),
			),
		}
	}
}

impl<I: Iterator<Item = usize>> GraphConstraints<I> {
	fn next(&mut self, i: usize, quad: Quad<usize>) -> Result<(), Option<usize>> {
		match self {
			Self::None => Err(None),
			Self::Any => Ok(()),
			Self::SameAsSubject => {
				if Some(quad.0) == quad.3 {
					Ok(())
				} else {
					Err(i.checked_add(1))
				}
			}
			Self::SameAsPredicate => {
				if Some(quad.1) == quad.3 {
					Ok(())
				} else {
					Err(i.checked_add(1))
				}
			}
			Self::SameAsObject => {
				if Some(quad.2) == quad.3 {
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
struct Resource<R> {
	value: R,
	as_subject: BTreeSet<usize>,
	as_predicate: BTreeSet<usize>,
	as_object: BTreeSet<usize>,
	as_graph: BTreeSet<usize>,
}

impl<R> Resource<R> {
	pub fn subject(value: R, i: usize) -> Self {
		Self {
			value,
			as_subject: std::iter::once(i).collect(),
			as_predicate: BTreeSet::new(),
			as_object: BTreeSet::new(),
			as_graph: BTreeSet::new(),
		}
	}

	pub fn predicate(value: R, i: usize) -> Self {
		Self {
			value,
			as_subject: BTreeSet::new(),
			as_predicate: std::iter::once(i).collect(),
			as_object: BTreeSet::new(),
			as_graph: BTreeSet::new(),
		}
	}

	pub fn object(value: R, i: usize) -> Self {
		Self {
			value,
			as_subject: BTreeSet::new(),
			as_predicate: BTreeSet::new(),
			as_object: std::iter::once(i).collect(),
			as_graph: BTreeSet::new(),
		}
	}

	pub fn graph(value: R, i: usize) -> Self {
		Self {
			value,
			as_subject: BTreeSet::new(),
			as_predicate: BTreeSet::new(),
			as_object: BTreeSet::new(),
			as_graph: std::iter::once(i).collect(),
		}
	}

	pub fn is_empty(&self) -> bool {
		self.as_subject.is_empty()
			&& self.as_predicate.is_empty()
			&& self.as_object.is_empty()
			&& self.as_graph.is_empty()
	}
}

impl<R: Debug> Debug for IndexedBTreeDataset<R> {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_set().entries(self.iter()).finish()
	}
}

impl<R: RdfDisplay> RdfDisplay for IndexedBTreeDataset<R> {
	fn rdf_fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		for t in self {
			writeln!(f, "{} .", t.rdf_display())?;
		}

		Ok(())
	}
}

#[cfg(feature = "serde")]
impl<R: serde::Serialize> serde::Serialize for IndexedBTreeDataset<R> {
	fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
		use serde::ser::SerializeSeq;
		let mut seq = serializer.serialize_seq(Some(self.len()))?;

		for quad in self {
			seq.serialize_element(&quad)?;
		}

		seq.end()
	}
}

#[cfg(feature = "serde")]
impl<'de, R: Clone + Ord + serde::Deserialize<'de>> serde::Deserialize<'de>
	for IndexedBTreeDataset<R>
{
	fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
		struct Visitor<R>(std::marker::PhantomData<R>);

		impl<'de, R: Clone + Ord + serde::Deserialize<'de>> serde::de::Visitor<'de> for Visitor<R> {
			type Value = IndexedBTreeDataset<R>;

			fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
				write!(formatter, "an RDF dataset")
			}

			fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
			where
				A: serde::de::SeqAccess<'de>,
			{
				let mut result = IndexedBTreeDataset::new();

				while let Some(quad) = seq.next_element()? {
					result.insert(quad);
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

	use crate::Quad;

	use super::IndexedBTreeDataset;

	fn rng_graph(rng: &mut SmallRng) -> Option<u32> {
		let g = rng.next_u32();
		if g % 2 == 0 {
			Some(g)
		} else {
			None
		}
	}

	fn insert_test(n: usize, seed: [u8; 32]) {
		let mut rng = SmallRng::from_seed(seed);
		let mut quads = Vec::new();
		quads.resize_with(n, || {
			Quad(
				rng.next_u32(),
				rng.next_u32(),
				rng.next_u32(),
				rng_graph(&mut rng),
			)
		});

		let mut dataset = IndexedBTreeDataset::new();
		for &t in &quads {
			dataset.insert(t);
		}

		quads.sort_unstable();
		quads.dedup();

		assert_eq!(dataset.len(), quads.len());

		test_eq(dataset, quads)
	}

	fn remove_test(n: usize, seed: [u8; 32]) {
		use rand::prelude::SliceRandom;
		let mut rng = SmallRng::from_seed(seed);
		let mut quads = Vec::new();
		quads.resize_with(n, || {
			Quad(
				rng.next_u32(),
				rng.next_u32(),
				rng.next_u32(),
				rng_graph(&mut rng),
			)
		});

		let mut dataset = IndexedBTreeDataset::new();
		for &t in &quads {
			dataset.insert(t);
		}

		quads.shuffle(&mut rng);

		for _ in 0..(n / 2) {
			let t = quads.pop().unwrap();
			dataset.remove(t.as_ref());
		}

		quads.sort_unstable();
		quads.dedup();

		test_eq(dataset, quads)
	}

	fn test_eq(dataset: IndexedBTreeDataset<u32>, quads: Vec<Quad<u32>>) {
		assert_eq!(dataset.len(), quads.len());

		let mut a = quads.iter().copied();
		let mut b = dataset.iter().map(Quad::into_copied);

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
