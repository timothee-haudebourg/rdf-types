use std::{cmp::Ordering, fmt::Debug, hash::Hash};

use educe::Educe;
use raw_btree::RawBTree;
use slab::Slab;

use super::super::Dataset;
use crate::{
	dataset::{DatasetMut, IndexedBTreeDataset, ResourceTraversableDataset, TraversableDataset},
	LocalTerm, Quad, RdfDisplay,
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

/// BTree-based RDF dataset.
#[derive(Clone)]
pub struct BTreeDataset<R = LocalTerm> {
	pub(crate) resources: Slab<Resource<R>>,
	pub(crate) quads: Slab<Quad<usize>>,
	pub(crate) resources_indexes: RawBTree<usize>,
	pub(crate) quads_indexes: RawBTree<usize>,
}

impl<R> Default for BTreeDataset<R> {
	fn default() -> Self {
		Self {
			quads: Slab::new(),
			resources: Slab::new(),
			quads_indexes: RawBTree::new(),
			resources_indexes: RawBTree::new(),
		}
	}
}

impl<R> BTreeDataset<R> {
	/// Creates a new empty dataset.
	pub fn new() -> Self {
		Self::default()
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

	/// Indexes the quads to allow more operation on this dataset, such as
	/// pattern matching using the [`PatternMatchingDataset`] trait.
	///
	/// [`PatternMatchingDataset`]: super::super::PatternMatchingDataset
	pub fn into_indexed(self) -> IndexedBTreeDataset<R> {
		IndexedBTreeDataset::from_non_indexed(self)
	}
}

impl<R: Ord> BTreeDataset<R> {
	fn index_of_resource(&self, resource: &R) -> Option<usize> {
		self.resources_indexes
			.get(resource_cmp(&self.resources), resource)
			.copied()
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
					self.resources[s_i].occurrences += 1;
					s_i
				}
				None => {
					let s_i = self.resources.insert(Resource::new(quad.0));
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
					let p_i = self.resources.insert(Resource::new(quad.1));
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
					let o_i = self.resources.insert(Resource::new(quad.2));
					self.resources_indexes
						.insert(resource_index_cmp(&self.resources), o_i);
					o_i
				}
			};

			let g_i = match g_i {
				Some((_, Some(g_i))) => {
					self.resources[g_i].occurrences += 1;
					Some(g_i)
				}
				Some((g, None)) => {
					let g_i = self.resources.insert(Resource::new(g));
					self.resources_indexes
						.insert(resource_index_cmp(&self.resources), g_i);
					Some(g_i)
				}
				None => None,
			};

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
				let Quad(s_i, p_i, o_i, g_i) = self.quads.remove(i);

				let s = &mut self.resources[s_i];
				s.occurrences -= 1;
				if s.is_empty() {
					self.resources_indexes
						.remove(resource_cmp(&self.resources), quad.0);
					self.resources.remove(s_i);
				}

				let p = &mut self.resources[p_i];
				p.occurrences -= 1;
				if p.is_empty() {
					self.resources_indexes
						.remove(resource_cmp(&self.resources), quad.1);
					self.resources.remove(p_i);
				}

				let o = &mut self.resources[o_i];
				o.occurrences -= 1;
				if o.is_empty() {
					self.resources_indexes
						.remove(resource_cmp(&self.resources), quad.2);
					self.resources.remove(o_i);
				}

				if let Some(g_i) = g_i {
					let g = &mut self.resources[g_i];
					g.occurrences -= 1;
					if g.is_empty() {
						self.resources_indexes
							.remove(resource_index_cmp(&self.resources), &g_i);
						self.resources.remove(g_i);
					}
				}

				true
			}
			None => false,
		}
	}
}

impl<R: Clone + Ord> FromIterator<Quad<R>> for BTreeDataset<R> {
	fn from_iter<T: IntoIterator<Item = Quad<R>>>(iter: T) -> Self {
		let mut result = Self::new();
		result.extend(iter);
		result
	}
}

impl<R: Clone + Ord> Extend<Quad<R>> for BTreeDataset<R> {
	fn extend<T: IntoIterator<Item = Quad<R>>>(&mut self, iter: T) {
		for quad in iter {
			self.insert(quad);
		}
	}
}

impl<R> Dataset for BTreeDataset<R> {
	type Resource = R;
}

impl<R> TraversableDataset for BTreeDataset<R> {
	type Quads<'a>
		= Quads<'a, R>
	where
		R: 'a;

	fn quads(&self) -> Self::Quads<'_> {
		self.iter()
	}
}

impl<R> ResourceTraversableDataset for BTreeDataset<R> {
	type Resources<'a>
		= Resources<'a, R>
	where
		R: 'a;

	fn resources(&self) -> Self::Resources<'_> {
		self.resources()
	}
}

impl<R: Clone + Ord> DatasetMut for BTreeDataset<R> {
	fn insert(&mut self, quad: Quad<Self::Resource>) {
		self.insert(quad);
	}

	fn remove(&mut self, quad: Quad<&Self::Resource>) {
		self.remove(quad);
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

impl<'a, R> IntoIterator for &'a BTreeDataset<R> {
	type Item = Quad<&'a R>;
	type IntoIter = Quads<'a, R>;

	fn into_iter(self) -> Self::IntoIter {
		self.iter()
	}
}

impl<R: Clone> IntoIterator for BTreeDataset<R> {
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

impl<A: PartialEq<B>, B> PartialEq<BTreeDataset<B>> for BTreeDataset<A> {
	fn eq(&self, other: &BTreeDataset<B>) -> bool {
		self.len() == other.len() && self.iter().zip(other).all(|(a, b)| a == b)
	}
}

impl<R: Eq> Eq for BTreeDataset<R> {}

impl<R: PartialOrd> PartialOrd for BTreeDataset<R> {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		self.iter().partial_cmp(other)
	}
}

impl<R: Ord> Ord for BTreeDataset<R> {
	fn cmp(&self, other: &Self) -> Ordering {
		self.iter().cmp(other)
	}
}

impl<R: Hash> Hash for BTreeDataset<R> {
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

impl<R: Debug> Debug for BTreeDataset<R> {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_set().entries(self.iter()).finish()
	}
}

impl<R: RdfDisplay> RdfDisplay for BTreeDataset<R> {
	fn rdf_fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		for t in self {
			writeln!(f, "{} .", t.rdf_display())?;
		}

		Ok(())
	}
}

#[cfg(feature = "serde")]
impl<R: serde::Serialize> serde::Serialize for BTreeDataset<R> {
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
impl<'de, R: Clone + Ord + serde::Deserialize<'de>> serde::Deserialize<'de> for BTreeDataset<R> {
	fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
		struct Visitor<R>(std::marker::PhantomData<R>);

		impl<'de, R: Clone + Ord + serde::Deserialize<'de>> serde::de::Visitor<'de> for Visitor<R> {
			type Value = BTreeDataset<R>;

			fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
				write!(formatter, "an RDF dataset")
			}

			fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
			where
				A: serde::de::SeqAccess<'de>,
			{
				let mut result = BTreeDataset::new();

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

	use super::BTreeDataset;

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

		let mut dataset = BTreeDataset::new();
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

		let mut dataset = BTreeDataset::new();
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

	fn test_eq(dataset: BTreeDataset<u32>, quads: Vec<Quad<u32>>) {
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
