use std::{cmp::Ordering, fmt::Debug, hash::Hash};

use educe::Educe;
use raw_btree::RawBTree;
use slab::Slab;

use super::{super::Graph, IndexedBTreeGraph};
use crate::{
	dataset::{GraphMut, ResourceTraversableGraph, TraversableGraph},
	LocalTerm, RdfDisplay, Triple,
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

/// BTree-based RDF graph.
#[derive(Clone)]
pub struct BTreeGraph<R = LocalTerm> {
	pub(crate) resources: Slab<Resource<R>>,
	pub(crate) triples: Slab<Triple<usize>>,
	pub(crate) resources_indexes: RawBTree<usize>,
	pub(crate) triples_indexes: RawBTree<usize>,
}

impl<R> Default for BTreeGraph<R> {
	fn default() -> Self {
		Self {
			triples: Slab::new(),
			resources: Slab::new(),
			triples_indexes: RawBTree::new(),
			resources_indexes: RawBTree::new(),
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

	/// Indexes the triples to allow more operation on this graph, such as
	/// pattern matching using the [`PatternMatchingGraph`] trait.
	///
	/// [`PatternMatchingGraph`]: super::super::PatternMatchingGraph
	pub fn into_indexed(self) -> IndexedBTreeGraph<R> {
		IndexedBTreeGraph::from_non_indexed(self)
	}
}

impl<R: Ord> BTreeGraph<R> {
	fn index_of_resource(&self, resource: &R) -> Option<usize> {
		self.resources_indexes
			.get(resource_cmp(&self.resources), resource)
			.copied()
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
					self.resources[s_i].occurrences += 1;
					s_i
				}
				None => {
					let s_i = self.resources.insert(Resource::new(triple.0));
					self.resources_indexes
						.insert(resource_index_cmp(&self.resources), s_i);
					s_i
				}
			};

			let p_i = match p_i {
				Some(p_i) => {
					self.resources[p_i].occurrences += 1;
					p_i
				}
				None => {
					let p_i = self.resources.insert(Resource::new(triple.1));
					self.resources_indexes
						.insert(resource_index_cmp(&self.resources), p_i);
					p_i
				}
			};

			let o_i = match o_i {
				Some(o_i) => {
					self.resources[o_i].occurrences += 1;
					o_i
				}
				None => {
					let o_i = self.resources.insert(Resource::new(triple.2));
					self.resources_indexes
						.insert(resource_index_cmp(&self.resources), o_i);
					o_i
				}
			};

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

				let s = &mut self.resources[s_i];
				s.occurrences -= 1;
				if s.is_empty() {
					self.resources_indexes
						.remove(resource_cmp(&self.resources), triple.0);
					self.resources.remove(s_i);
				}

				let p = &mut self.resources[p_i];
				p.occurrences -= 1;
				if p.is_empty() {
					self.resources_indexes
						.remove(resource_cmp(&self.resources), triple.1);
					self.resources.remove(p_i);
				}

				let o = &mut self.resources[o_i];
				o.occurrences -= 1;
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
	type Triples<'a>
		= Triples<'a, R>
	where
		R: 'a;

	fn triples(&self) -> Self::Triples<'_> {
		self.iter()
	}
}

impl<R> ResourceTraversableGraph for BTreeGraph<R> {
	type GraphResources<'a>
		= Resources<'a, R>
	where
		R: 'a;

	fn graph_resources(&self) -> Self::GraphResources<'_> {
		self.resources()
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

impl<'a, R> IntoIterator for &'a BTreeGraph<R> {
	type Item = Triple<&'a R>;
	type IntoIter = Triples<'a, R>;

	fn into_iter(self) -> Self::IntoIter {
		self.iter()
	}
}

impl<R: Clone> IntoIterator for BTreeGraph<R> {
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

#[derive(Default, Clone)]
pub(crate) struct Resource<R> {
	pub value: R,
	occurrences: usize,
}

impl<R> Resource<R> {
	pub fn new(value: R) -> Self {
		Self {
			value,
			occurrences: 1,
		}
	}

	pub fn is_empty(&self) -> bool {
		self.occurrences == 0
	}
}

impl<R: Debug> Debug for BTreeGraph<R> {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_set().entries(self.iter()).finish()
	}
}

impl<R: RdfDisplay> RdfDisplay for BTreeGraph<R> {
	fn rdf_fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		for t in self {
			writeln!(f, "{} .", t.rdf_display())?;
		}

		Ok(())
	}
}

#[cfg(feature = "serde")]
impl<R: serde::Serialize> serde::Serialize for BTreeGraph<R> {
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
impl<'de, R: Clone + Ord + serde::Deserialize<'de>> serde::Deserialize<'de> for BTreeGraph<R> {
	fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
		struct Visitor<R>(std::marker::PhantomData<R>);

		impl<'de, R: Clone + Ord + serde::Deserialize<'de>> serde::de::Visitor<'de> for Visitor<R> {
			type Value = BTreeGraph<R>;

			fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
				write!(formatter, "an RDF graph")
			}

			fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
			where
				A: serde::de::SeqAccess<'de>,
			{
				let mut result = BTreeGraph::new();

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
