use core::fmt;
use std::{cmp::Ordering, collections::BTreeSet, fmt::Debug, hash::Hash};

use educe::Educe;
use raw_btree::RawBTree;
use slab::Slab;

use super::{
	super::{Dataset, PatternMatchingDataset},
	BTreeDataset,
};
use crate::{
	Quad,
	dataset::{
		BTreeGraph, DatasetMut, FiniteDataset, MultiPatternMatchingDataset,
		NamedGraphFiniteDataset, ObjectFiniteDataset, PatternMatchingDatasetMut,
		PredicateFiniteDataset, ResourceFiniteDataset, SubjectFiniteDataset,
	},
	pattern::LinearQuadPattern,
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
pub struct IndexedBTreeDataset<R> {
	/// All the resources appearing in this dataset.
	///
	/// Each of them is uniquely indexed by a `usize` in this slab.
	resources: Slab<Resource<R>>,

	/// All the quads in this dataset.
	///
	/// Each of them is uniquely indexed by a `usize` in this slab.
	quads: Slab<Quad<usize>>,

	/// Maps each resource to its index in `resources`.
	///
	/// Using a `RawBTree` so we don't have to actually store each resource
	/// there.
	resources_indexes: RawBTree<usize>,

	/// Maps each quad to its index in `quads`.
	///
	/// Using a `RawBTree` so we don't have to actually store each quad there.
	quads_indexes: RawBTree<usize>,

	/// All the resources that appear as quad subject.
	subjects: BTreeSet<usize>,

	/// All the resources that appear as quad predicate.
	predicates: BTreeSet<usize>,

	/// All the resources that appear as quad object.
	objects: BTreeSet<usize>,

	/// All the quads in the default graph.
	default_graph: BTreeSet<usize>,

	/// All the resources that appear as named graph.
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
	pub fn iter(&self) -> Iter<'_, R> {
		Iter {
			resources: &self.resources,
			quads: &self.quads,
			indexes: self.quads_indexes.iter(),
		}
	}

	/// Returns an iterator over the resources of the dataset.
	pub fn resources(&self) -> Resources<'_, R> {
		Resources {
			resources: &self.resources,
			indexes: self.resources_indexes.iter(),
		}
	}

	/// Returns an iterator over the subjects of the dataset.
	pub fn subjects(&self) -> Subjects<'_, R> {
		Subjects {
			resources: &self.resources,
			indexes: self.subjects.iter(),
		}
	}

	/// Returns an iterator over the predicates of the dataset.
	pub fn predicates(&self) -> Predicates<'_, R> {
		Predicates {
			resources: &self.resources,
			indexes: self.predicates.iter(),
		}
	}

	/// Returns an iterator over the objects of the dataset.
	pub fn objects(&self) -> Objects<'_, R> {
		Objects {
			resources: &self.resources,
			indexes: self.objects.iter(),
		}
	}

	/// Returns an iterator over the named graphs of the dataset.
	pub fn named_graphs(&self) -> NamedGraphs<'_, R> {
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

	/// Removes the given quad from the dataset.
	///
	/// If `remove_index` is set to `false`, this function will assume that
	/// the quad has already been removed from `self.quad_indexes`.
	fn remove_by_index(&mut self, i: usize, remove_index: bool) {
		if remove_index {
			self.quads_indexes
				.remove(quad_index_cmp(&self.resources, &self.quads), &i);
		}

		let Quad(s_i, p_i, o_i, g_i) = self.quads.remove(i);

		let s = &mut self.resources[s_i];
		s.as_subject.remove(&i);
		if s.as_subject.is_empty() {
			self.subjects.remove(&s_i);
			if s.is_empty() {
				self.resources_indexes
					.remove(resource_index_cmp(&self.resources), &s_i);
				self.resources.remove(s_i);
			}
		}

		let p = &mut self.resources[p_i];
		p.as_predicate.remove(&i);
		if p.as_predicate.is_empty() {
			self.predicates.remove(&p_i);
			if p.is_empty() {
				self.resources_indexes
					.remove(resource_index_cmp(&self.resources), &p_i);
				self.resources.remove(p_i);
			}
		}

		let o = &mut self.resources[o_i];
		o.as_object.remove(&i);
		if o.as_object.is_empty() {
			self.objects.remove(&o_i);
			if o.is_empty() {
				self.resources_indexes
					.remove(resource_index_cmp(&self.resources), &o_i);
				self.resources.remove(o_i);
			}
		}

		match g_i {
			Some(g_i) => {
				let g = &mut self.resources[g_i];
				g.as_graph.remove(&i);
				if g.as_graph.is_empty() {
					self.named_graphs.remove(&g_i);
					if g.is_empty() {
						self.resources_indexes
							.remove(resource_index_cmp(&self.resources), &g_i);
						self.resources.remove(g_i);
					}
				}
			}
			None => {
				self.default_graph.remove(&i);
			}
		}
	}

	/// Returns an iterator over all the quads matching the given canonical
	/// quad pattern.
	pub fn pattern_matching(
		&self,
		Quad(s, p, o, g): LinearQuadPattern<&R>,
	) -> PatternMatching<'_, R> {
		PatternMatching {
			resources: &self.resources,
			quads: &self.quads,
			subject: ComponentConstraints::new(self, s.map(Some), |r| &r.as_subject),
			predicate: ComponentConstraints::new(self, p.map(Some), |r| &r.as_predicate),
			object: ComponentConstraints::new(self, o.map(Some), |r| &r.as_object),
			graph: ComponentConstraints::new(self, g, |r| &r.as_graph),
			i: 0,
		}
	}

	/// Returns an iterator over all the quads matching the given canonical
	/// quad pattern.
	pub fn multi_pattern_matching<'a, P>(
		&self,
		Quad(s, p, o, g): LinearQuadPattern<P>,
	) -> MultiPatternMatching<'_, R>
	where
		P: IntoIterator<Item = &'a R>,
		R: 'a,
	{
		MultiPatternMatching {
			resources: &self.resources,
			quads: &self.quads,
			subject: ComponentConstraints::new_multi(self, s.map(Some), |r| &r.as_subject),
			predicate: ComponentConstraints::new_multi(self, p.map(Some), |r| &r.as_predicate),
			object: ComponentConstraints::new_multi(self, o.map(Some), |r| &r.as_object),
			graph: ComponentConstraints::new_multi(self, g, |r| &r.as_graph),
			i: 0,
		}
	}

	/// Returns an iterator over all the quads matching the given canonical
	/// quad pattern.
	///
	/// Each matching quad returned by [`Iterator::next`] are removed from the
	/// dataset. Matching quads that are not iterated on a kept in the dataset,
	/// even when the iterator is dropped.
	pub fn extract_pattern_matching(
		&mut self,
		Quad(s, p, o, g): LinearQuadPattern<&R>,
	) -> ExtractPatternMatching<'_, R> {
		let subject = ComponentConstraints::new_owned(self, s.map(Some), |r| &r.as_subject);
		let predicate = ComponentConstraints::new_owned(self, p.map(Some), |r| &r.as_predicate);
		let object = ComponentConstraints::new_owned(self, o.map(Some), |r| &r.as_object);
		let graph = ComponentConstraints::new_owned(self, g, |r| &r.as_graph);

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

impl<R> From<BTreeDataset<R>> for IndexedBTreeDataset<R> {
	fn from(value: BTreeDataset<R>) -> Self {
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

impl<R> FiniteDataset for IndexedBTreeDataset<R> {
	type Quads<'a>
		= Iter<'a, R>
	where
		R: 'a;

	fn quads(&self) -> Self::Quads<'_> {
		self.iter()
	}
}

impl<R> ResourceFiniteDataset for IndexedBTreeDataset<R> {
	type Resources<'a>
		= Resources<'a, R>
	where
		R: 'a;

	fn resources(&self) -> Self::Resources<'_> {
		self.resources()
	}

	fn resource_count(&self) -> usize {
		self.resources.len()
	}
}

impl<R> SubjectFiniteDataset for IndexedBTreeDataset<R> {
	type Subjects<'a>
		= Subjects<'a, R>
	where
		R: 'a;

	fn subjects(&self) -> Self::Subjects<'_> {
		self.subjects()
	}

	fn subject_count(&self) -> usize {
		self.subjects.len()
	}
}

impl<R> PredicateFiniteDataset for IndexedBTreeDataset<R> {
	type Predicates<'a>
		= Predicates<'a, R>
	where
		R: 'a;

	fn predicates(&self) -> Self::Predicates<'_> {
		self.predicates()
	}

	fn predicate_count(&self) -> usize {
		self.predicates.len()
	}
}

impl<R> ObjectFiniteDataset for IndexedBTreeDataset<R> {
	type Objects<'a>
		= Objects<'a, R>
	where
		R: 'a;

	fn objects(&self) -> Self::Objects<'_> {
		self.objects()
	}

	fn object_count(&self) -> usize {
		self.objects.len()
	}
}

impl<R> NamedGraphFiniteDataset for IndexedBTreeDataset<R> {
	type NamedGraphs<'a>
		= NamedGraphs<'a, R>
	where
		R: 'a;

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
	type QuadPatternMatching<'a, 'p>
		= PatternMatching<'a, R>
	where
		R: 'a,
		Self::Resource: 'p;

	fn quad_pattern_matching<'p>(
		&self,
		pattern: LinearQuadPattern<&'p Self::Resource>,
	) -> Self::QuadPatternMatching<'_, 'p> {
		self.pattern_matching(pattern)
	}

	fn contains_quad(&self, quad: Quad<&Self::Resource>) -> bool {
		self.contains(quad)
	}
}

impl<R: Ord> MultiPatternMatchingDataset for IndexedBTreeDataset<R> {
	type QuadMultiPatternMatching<'a, 'p>
		= MultiPatternMatching<'a, R>
	where
		R: 'a,
		Self::Resource: 'p;

	fn quad_multi_pattern_matching<'p, P>(
		&self,
		pattern: LinearQuadPattern<P>,
	) -> Self::QuadMultiPatternMatching<'_, 'p>
	where
		P: IntoIterator<Item = &'p R>,
		R: 'p,
	{
		self.multi_pattern_matching(pattern)
	}
}

impl<R: Clone + Ord> PatternMatchingDatasetMut for IndexedBTreeDataset<R> {
	type ExtractMatchingQuads<'a, 'p>
		= ExtractPatternMatching<'a, R>
	where
		Self: 'a,
		R: 'p;

	fn extract_matching_quads<'p>(
		&mut self,
		pattern: impl Into<LinearQuadPattern<&'p Self::Resource>>,
	) -> Self::ExtractMatchingQuads<'_, 'p>
	where
		R: 'p,
	{
		self.extract_pattern_matching(pattern.into())
	}
}

impl<R: fmt::Display> fmt::Display for IndexedBTreeDataset<R> {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		for quad in self {
			writeln!(f, "{quad}")?;
		}

		Ok(())
	}
}

/// Iterator over the quads of a [`BTreeGraph`].
#[derive(Educe)]
#[educe(Clone, Copy)]
pub struct Iter<'a, R> {
	resources: &'a Slab<Resource<R>>,
	quads: &'a Slab<Quad<usize>>,
	indexes: raw_btree::Iter<'a, usize>,
}

impl<'a, R> Iterator for Iter<'a, R> {
	type Item = Quad<&'a R>;

	fn next(&mut self) -> Option<Self::Item> {
		self.indexes
			.next()
			.map(|&i| quad_with_resources(self.resources, self.quads[i]))
	}
}

/// Iterator over the quads of a [`BTreeGraph`].
pub struct IntoIter<R> {
	resources: Slab<Resource<R>>,
	quads: Slab<Quad<usize>>,
	indexes: raw_btree::IntoIter<usize>,
}

impl<R: Clone> Iterator for IntoIter<R> {
	type Item = Quad<R>;

	fn next(&mut self) -> Option<Self::Item> {
		self.indexes
			.next()
			.map(|i| quad_with_resources(&self.resources, self.quads.remove(i)).cloned())
	}
}

impl<'a, R> IntoIterator for &'a IndexedBTreeDataset<R> {
	type Item = Quad<&'a R>;
	type IntoIter = Iter<'a, R>;

	fn into_iter(self) -> Self::IntoIter {
		self.iter()
	}
}

impl<R: Clone> IntoIterator for IndexedBTreeDataset<R> {
	type Item = Quad<R>;
	type IntoIter = IntoIter<R>;

	fn into_iter(self) -> Self::IntoIter {
		IntoIter {
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
	subject: ComponentConstraints<TripleIndexes<'a>>,
	predicate: ComponentConstraints<TripleIndexes<'a>>,
	object: ComponentConstraints<TripleIndexes<'a>>,
	graph: ComponentConstraints<TripleIndexes<'a>>,
	i: usize,
}

impl<'a, R> Iterator for PatternMatching<'a, R> {
	type Item = Quad<&'a R>;

	fn next(&mut self) -> Option<Self::Item> {
		while self.i < self.quads.capacity() {
			let i = match self.subject.next(self.i) {
				Ok(()) => self.i,
				Err(j) => j?,
			};

			match self.quads.get(i) {
				Some(&quad) => match self.predicate.next(i) {
					Ok(()) => match self.object.next(i) {
						Ok(()) => match self.graph.next(i) {
							Ok(()) => {
								if let Some(j) = i.checked_add(1) {
									self.i = j;
								}
								return Some(quad_with_resources(self.resources, quad));
							}
							Err(j) => self.i = j?,
						},
						Err(j) => self.i = j?,
					},
					Err(j) => self.i = j?,
				},
				None => {
					// If `subject` is `Any`, the selected quad might not even
					// exist.
					self.i = self.i.checked_add(1)?;
				}
			}
		}

		None
	}
}

/// Iterator over the quads of a [`BTreeGraph`] matching some given pattern.
pub struct MultiPatternMatching<'a, R> {
	resources: &'a Slab<Resource<R>>,
	quads: &'a Slab<Quad<usize>>,
	subject: ComponentConstraints<OwnedTripleIndexes>,
	predicate: ComponentConstraints<OwnedTripleIndexes>,
	object: ComponentConstraints<OwnedTripleIndexes>,
	graph: ComponentConstraints<OwnedTripleIndexes>,
	i: usize,
}

impl<'a, R> Iterator for MultiPatternMatching<'a, R> {
	type Item = Quad<&'a R>;

	fn next(&mut self) -> Option<Self::Item> {
		while self.i < self.quads.capacity() {
			let i = match self.subject.next(self.i) {
				Ok(()) => self.i,
				Err(j) => j?,
			};

			match self.quads.get(i) {
				Some(&quad) => match self.predicate.next(i) {
					Ok(()) => match self.object.next(i) {
						Ok(()) => match self.graph.next(i) {
							Ok(()) => {
								if let Some(j) = i.checked_add(1) {
									self.i = j;
								}
								return Some(quad_with_resources(self.resources, quad));
							}
							Err(j) => self.i = j?,
						},
						Err(j) => self.i = j?,
					},
					Err(j) => self.i = j?,
				},
				None => {
					// If `subject` is `Any`, the selected quad might not even
					// exist.
					self.i = self.i.checked_add(1)?;
				}
			}
		}

		None
	}
}

/// Iterator over the quads of a [`BTreeGraph`] matching some given pattern.
///
/// Dropping this iterator will *not* extract the remaining matching quads.
pub struct ExtractPatternMatching<'a, R> {
	dataset: &'a mut IndexedBTreeDataset<R>,
	subject: ComponentConstraints<OwnedTripleIndexes>,
	predicate: ComponentConstraints<OwnedTripleIndexes>,
	object: ComponentConstraints<OwnedTripleIndexes>,
	graph: ComponentConstraints<OwnedTripleIndexes>,
	i: usize,
}

impl<R: Clone + Ord> Iterator for ExtractPatternMatching<'_, R> {
	type Item = Quad<R>;

	fn next(&mut self) -> Option<Self::Item> {
		while self.i < self.dataset.quads.capacity() {
			let i = match self.subject.next(self.i) {
				Ok(()) => self.i,
				Err(j) => j?,
			};

			match self.dataset.quads.get(i) {
				Some(&quad) => match self.predicate.next(i) {
					Ok(()) => match self.object.next(i) {
						Ok(()) => match self.graph.next(i) {
							Ok(()) => {
								let value =
									quad_with_resources(&self.dataset.resources, quad).cloned();
								self.dataset.remove_by_index(i, true);
								if let Some(j) = i.checked_add(1) {
									self.i = j;
								}
								return Some(value);
							}
							Err(j) => self.i = j?,
						},
						Err(j) => self.i = j?,
					},
					Err(j) => self.i = j?,
				},
				None => {
					// If `subject` is `Any`, the selected quad might not even
					// exist.
					self.i = self.i.checked_add(1)?;
				}
			}
		}

		None
	}
}

type TripleIndexes<'a> = std::iter::Copied<std::collections::btree_set::Iter<'a, usize>>;
type OwnedTripleIndexes = std::vec::IntoIter<usize>;

enum ComponentConstraints<I: Iterator> {
	None,
	Any,
	Fixed(std::iter::Peekable<I>),
}

impl<'a> ComponentConstraints<TripleIndexes<'a>> {
	fn new<R: Ord>(
		dataset: &'a IndexedBTreeDataset<R>,
		r: Option<Option<&R>>,
		f: impl FnOnce(&Resource<R>) -> &BTreeSet<usize>,
	) -> Self {
		match r {
			None => Self::Any,
			Some(None) => Self::Fixed(dataset.default_graph.iter().copied().peekable()),
			Some(Some(r)) => match dataset.get_resource(r) {
				Some(resource) => Self::Fixed(f(resource).iter().copied().peekable()),
				None => Self::None,
			},
		}
	}
}

impl ComponentConstraints<OwnedTripleIndexes> {
	fn new_owned<R: Ord>(
		dataset: &IndexedBTreeDataset<R>,
		r: Option<Option<&R>>,
		f: impl FnOnce(&Resource<R>) -> &BTreeSet<usize>,
	) -> Self {
		match r {
			None => Self::Any,
			Some(None) => Self::Fixed(
				dataset
					.default_graph
					.iter()
					.copied()
					.collect::<Vec<_>>()
					.into_iter()
					.peekable(),
			),
			Some(Some(r)) => match dataset.get_resource(r) {
				Some(resource) => Self::Fixed(
					f(resource)
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

	fn new_multi<'p, R, P>(
		dataset: &IndexedBTreeDataset<R>,
		s: Option<Option<P>>,
		f: impl Fn(&Resource<R>) -> &BTreeSet<usize>,
	) -> Self
	where
		P: IntoIterator<Item = &'p R>,
		R: 'p + Ord,
	{
		match s {
			None => Self::Any,
			Some(None) => Self::Fixed(
				dataset
					.default_graph
					.iter()
					.copied()
					.collect::<Vec<_>>()
					.into_iter()
					.peekable(),
			),
			Some(Some(multi_s)) => {
				let mut indexes = Vec::new();
				for s in multi_s {
					if let Some(resource) = dataset.get_resource(s) {
						indexes.extend(f(resource).iter().copied());
					}
				}

				if indexes.is_empty() {
					Self::None
				} else {
					indexes.sort_unstable();
					Self::Fixed(indexes.into_iter().peekable())
				}
			}
		}
	}
}

impl<I: Iterator<Item = usize>> ComponentConstraints<I> {
	fn next(&mut self, i: usize) -> Result<(), Option<usize>> {
		match self {
			Self::None => Err(None),
			Self::Any => Ok(()),
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
	use rand::{RngCore, SeedableRng, rngs::SmallRng};

	use crate::Quad;

	use super::IndexedBTreeDataset;

	fn rng_graph(rng: &mut SmallRng) -> Option<u32> {
		let g = rng.next_u32();
		if g % 2 == 0 { Some(g) } else { None }
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
		let mut b = dataset.iter().map(|q| q.cloned());

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
