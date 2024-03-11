use std::{cmp::Ordering, collections::BTreeMap, fmt::Debug, hash::Hash};

use crate::{
	dataset::{btree_graph, BTreeGraph, DatasetMut, PatternMatchingDataset, TraversableDataset},
	pattern::{quad::canonical::PatternGraph, CanonicalQuadPattern, CanonicalTriplePattern},
	utils::OptionIterator,
	Dataset, Quad, RdfDisplay, Term, Triple,
};

/// BTree-based RDF dataset.
#[derive(Clone)]
pub struct BTreeDataset<R = Term> {
	default_graph: BTreeGraph<R>,
	named_graphs: BTreeMap<R, BTreeGraph<R>>,
}

impl<R> Default for BTreeDataset<R> {
	fn default() -> Self {
		Self {
			default_graph: BTreeGraph::new(),
			named_graphs: BTreeMap::new(),
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
		self.named_graphs
			.values()
			.fold(self.default_graph.len(), |l, g| l + g.len())
	}

	/// Checks if the dataset is empty.
	pub fn is_empty(&self) -> bool {
		self.default_graph.is_empty() && self.named_graphs.values().all(BTreeGraph::is_empty)
	}

	/// Returns an iterator over the quads of the dataset.
	pub fn iter(&self) -> Quads<R> {
		Quads {
			default_graph: self.default_graph.iter(),
			named_graphs: self.named_graphs.iter(),
			current_named_graph: None,
		}
	}
}

impl<R: Ord> BTreeDataset<R> {
	/// Checks if the dataset contains the given quad.
	pub fn contains(&self, quad: Quad<&R>) -> bool {
		let (triple, g) = quad.into_triple();
		match g {
			Some(name) => self
				.named_graphs
				.get(name)
				.is_some_and(|g| g.contains(triple)),
			None => self.default_graph.contains(triple),
		}
	}

	/// Returns the graph with the given label, if any.
	pub fn graph(&self, g: Option<&R>) -> Option<&BTreeGraph<R>> {
		match g {
			Some(name) => self.named_graphs.get(name),
			None => Some(&self.default_graph),
		}
	}

	/// Returns the a mutable reference to the graph with the given label, if
	/// any.
	pub fn graph_mut(&mut self, g: Option<&R>) -> Option<&mut BTreeGraph<R>> {
		match g {
			Some(name) => self.named_graphs.get_mut(name),
			None => Some(&mut self.default_graph),
		}
	}

	/// Returns the a mutable reference to the graph with the given label.
	///
	/// If the graph is not present in the dataset, adds it as an empty graph
	/// in return it.
	pub fn graph_mut_or_empty(&mut self, g: Option<&R>) -> &mut BTreeGraph<R>
	where
		R: Clone,
	{
		match g {
			Some(name) => {
				if !self.named_graphs.contains_key(name) {
					self.named_graphs.insert(name.clone(), BTreeGraph::new());
				}

				self.named_graphs.get_mut(name).unwrap()
			}
			None => &mut self.default_graph,
		}
	}

	/// Inserts the given graph into the dataset.
	///
	/// If a graph with the same label already exists, it is replaced in
	/// returned.
	pub fn insert_graph(&mut self, g: Option<R>, graph: BTreeGraph<R>) -> Option<BTreeGraph<R>> {
		match g {
			Some(name) => self.named_graphs.insert(name, graph),
			None => Some(std::mem::replace(&mut self.default_graph, graph)),
		}
	}

	/// Inserts the given quad into the dataset.
	pub fn insert(&mut self, quad: Quad<R>) -> bool
	where
		R: Clone,
	{
		let (triple, g) = quad.into_triple();
		match g {
			Some(name) => self.named_graphs.entry(name).or_default().insert(triple),
			None => self.default_graph.insert(triple),
		}
	}

	/// Removes the given graph from the dataset.
	///
	/// If `g` is `None`, the default graph is cleared and its old value
	/// returned.
	pub fn remove_graph(&mut self, g: Option<&R>) -> Option<BTreeGraph<R>> {
		match g {
			Some(name) => self.named_graphs.remove(name),
			None => Some(std::mem::take(&mut self.default_graph)),
		}
	}

	/// Removes the given quad from the dataset.
	pub fn remove(&mut self, quad: Quad<&R>) -> Option<Triple<R>> {
		let (triple, g) = quad.into_triple();
		match g {
			Some(name) => self.named_graphs.get_mut(name)?.remove(triple),
			None => self.default_graph.remove(triple),
		}
	}

	/// Returns an iterator over the quads matching the given pattern.
	pub fn pattern_matching<'p>(
		&self,
		quad: CanonicalQuadPattern<&'p R>,
	) -> PatternMatching<'_, 'p, R> {
		let (triple_pattern, graph_pattern) = quad.into_triple();

		let default_graph = match graph_pattern {
			PatternGraph::Any | PatternGraph::Given(None) => {
				Some(self.default_graph.pattern_matching(triple_pattern))
			}
			_ => None,
		};

		PatternMatching {
			triple_pattern,
			graph_pattern,
			default_graph: OptionIterator(default_graph),
			named_graphs: self.named_graphs.iter(),
			current_named_graph: None,
		}
	}
}

impl<R: PartialEq> PartialEq for BTreeDataset<R> {
	fn eq(&self, other: &Self) -> bool {
		self.named_graphs.len() == other.named_graphs.len()
			&& self.default_graph == other.default_graph
			&& self
				.named_graphs
				.iter()
				.zip(&other.named_graphs)
				.all(|(a, b)| a == b)
	}
}

impl<R: Eq> Eq for BTreeDataset<R> {}

impl<R: PartialOrd> PartialOrd for BTreeDataset<R> {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		match self.default_graph.partial_cmp(&other.default_graph) {
			Some(Ordering::Equal) => self.named_graphs.partial_cmp(&other.named_graphs),
			cmp => cmp,
		}
	}
}

impl<R: Ord> Ord for BTreeDataset<R> {
	fn cmp(&self, other: &Self) -> Ordering {
		self.default_graph
			.cmp(&other.default_graph)
			.then_with(|| self.named_graphs.cmp(&other.named_graphs))
	}
}

impl<R: Hash> Hash for BTreeDataset<R> {
	fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
		self.default_graph.hash(state);
		self.named_graphs.hash(state)
	}
}

impl<R> Dataset for BTreeDataset<R> {
	type Resource = R;
}

impl<R> TraversableDataset for BTreeDataset<R> {
	type Quads<'a> = Quads<'a, R> where R: 'a;

	fn quads(&self) -> Self::Quads<'_> {
		self.iter()
	}
}

/// Iterator over the quads of a [`BTreeDataset`].
pub struct Quads<'a, R> {
	default_graph: btree_graph::Triples<'a, R>,
	named_graphs: std::collections::btree_map::Iter<'a, R, BTreeGraph<R>>,
	current_named_graph: Option<(&'a R, btree_graph::Triples<'a, R>)>,
}

impl<'a, R> Iterator for Quads<'a, R> {
	type Item = Quad<&'a R>;

	fn next(&mut self) -> Option<Self::Item> {
		self.default_graph
			.next()
			.map(|t| t.into_quad(None))
			.or_else(|| loop {
				match &mut self.current_named_graph {
					Some((g, triples)) => match triples.next() {
						Some(triple) => break Some(triple.into_quad(Some(*g))),
						None => self.current_named_graph = None,
					},
					None => match self.named_graphs.next() {
						Some((g, graph)) => self.current_named_graph = Some((g, graph.iter())),
						None => break None,
					},
				}
			})
	}
}

/// Iterator over the quads of a [`BTreeDataset`].
pub struct IntoQuads<R> {
	default_graph: btree_graph::IntoTriples<R>,
	named_graphs: std::collections::btree_map::IntoIter<R, BTreeGraph<R>>,
	current_named_graph: Option<(R, btree_graph::IntoTriples<R>)>,
}

impl<R: Clone> Iterator for IntoQuads<R> {
	type Item = Quad<R>;

	fn next(&mut self) -> Option<Self::Item> {
		self.default_graph
			.next()
			.map(|t| t.into_quad(None))
			.or_else(|| loop {
				match &mut self.current_named_graph {
					Some((g, triples)) => match triples.next() {
						Some(triple) => break Some(triple.into_quad(Some(g.clone()))),
						None => self.current_named_graph = None,
					},
					None => match self.named_graphs.next() {
						Some((g, graph)) => self.current_named_graph = Some((g, graph.into_iter())),
						None => break None,
					},
				}
			})
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
	type IntoIter = IntoQuads<R>;

	fn into_iter(self) -> Self::IntoIter {
		IntoQuads {
			default_graph: self.default_graph.into_iter(),
			named_graphs: self.named_graphs.into_iter(),
			current_named_graph: None,
		}
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

impl<R: Ord> PatternMatchingDataset for BTreeDataset<R> {
	type QuadPatternMatching<'a, 'p> = PatternMatching<'a, 'p, R> where Self: 'a, R: 'p;

	fn quad_pattern_matching<'p>(
		&self,
		pattern: CanonicalQuadPattern<&'p Self::Resource>,
	) -> Self::QuadPatternMatching<'_, 'p> {
		self.pattern_matching(pattern)
	}
}

/// Iterator over the quads of a [`BTreeGraph`] matching some given pattern.
pub struct PatternMatching<'a, 'p, R> {
	triple_pattern: CanonicalTriplePattern<&'p R>,
	graph_pattern: PatternGraph<&'p R>,
	default_graph: OptionIterator<btree_graph::PatternMatching<'a, R>>,
	named_graphs: std::collections::btree_map::Iter<'a, R, BTreeGraph<R>>,
	current_named_graph: Option<(&'a R, btree_graph::PatternMatching<'a, R>)>,
}

impl<'a, 'p, R: Ord> Iterator for PatternMatching<'a, 'p, R> {
	type Item = Quad<&'a R>;

	fn next(&mut self) -> Option<Self::Item> {
		self.default_graph
			.next()
			.map(|t| t.into_quad(None))
			.or_else(|| loop {
				match &mut self.current_named_graph {
					Some((g, triples)) => match triples.next() {
						Some(triple) => break Some(triple.into_quad(Some(*g))),
						None => self.current_named_graph = None,
					},
					None => match self.named_graphs.next() {
						Some((g, graph)) => {
							let triple = match self.graph_pattern {
								PatternGraph::Given(None) => break None,
								PatternGraph::Given(Some(h)) => {
									if h == g {
										Some(self.triple_pattern)
									} else {
										None
									}
								}
								PatternGraph::Any => Some(self.triple_pattern),
								PatternGraph::SameAsSubject => {
									Some(self.triple_pattern.with_subject(g))
								}
								PatternGraph::SameAsPredicate => {
									Some(self.triple_pattern.with_predicate(g))
								}
								PatternGraph::SameAsObject => {
									Some(self.triple_pattern.with_object(g))
								}
							};

							if let Some(triple) = triple {
								self.current_named_graph = Some((g, graph.pattern_matching(triple)))
							}
						}
						None => break None,
					},
				}
			})
	}
}

impl<R: Clone + Ord> Extend<Quad<R>> for BTreeDataset<R> {
	fn extend<T: IntoIterator<Item = Quad<R>>>(&mut self, iter: T) {
		for quad in iter {
			self.insert(quad);
		}
	}
}

impl<R: Clone + Ord> Extend<Triple<R>> for BTreeDataset<R> {
	fn extend<T: IntoIterator<Item = Triple<R>>>(&mut self, iter: T) {
		for triple in iter {
			self.insert(triple.into_quad(None));
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

impl<R: Clone + Ord> FromIterator<Triple<R>> for BTreeDataset<R> {
	fn from_iter<T: IntoIterator<Item = Triple<R>>>(iter: T) -> Self {
		let mut result = Self::new();
		result.extend(iter);
		result
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

#[cfg(test)]
mod tests {
	use crate::{grdf_quad, grdf_quad_pattern, grdf_quads, pattern::ResourceOrVar};

	use super::*;
	#[test]
	fn pattern_matching() {
		let dataset: BTreeDataset = grdf_quads! [
			_:"0" _:"1" _:"2" .
			_:"0" _:"2" _:"1" .
			_:"1" _:"0" _:"2" .
			_:"1" _:"2" _:"0" .
			_:"2" _:"0" _:"1" .
			_:"2" _:"1" _:"0" .
			_:"0" _:"0" _:"0" .
			_:"0" _:"2" _:"2" _:"graph0" .
			_:"0" _:"2" _:"2" _:"graph2" .
			_:"0" _:"0" _:"0" _:"0" .
			_:"0" _:"1" _:"0" _:"1" .
		]
		.into_iter()
		.collect();

		assert_eq!(
			dataset
				.pattern_matching(grdf_quad!(_:"0" _:"1" _:"2").as_ref().into())
				.count(),
			1
		);

		let x = 0;
		assert_eq!(
			dataset
				.pattern_matching(
					grdf_quad_pattern!(?x ?x _:"0")
						.as_ref()
						.map(ResourceOrVar::as_ref)
						.into()
				)
				.count(),
			1
		);
		assert_eq!(
			dataset
				.pattern_matching(
					grdf_quad_pattern!(?x ?x _:"1")
						.as_ref()
						.map(ResourceOrVar::as_ref)
						.into()
				)
				.count(),
			0
		);

		let y = 1;
		assert_eq!(
			dataset
				.pattern_matching(
					grdf_quad_pattern!(_:"0" ?x ?y)
						.as_ref()
						.map(ResourceOrVar::as_ref)
						.into()
				)
				.count(),
			3
		);
		assert_eq!(
			dataset
				.pattern_matching(
					grdf_quad_pattern!(?x _:"2" ?y)
						.as_ref()
						.map(ResourceOrVar::as_ref)
						.into()
				)
				.count(),
			2
		);
		assert_eq!(
			dataset
				.pattern_matching(
					grdf_quad_pattern!(?x ?y _:"0")
						.as_ref()
						.map(ResourceOrVar::as_ref)
						.into()
				)
				.count(),
			3
		);

		let z = 2;
		assert_eq!(
			dataset
				.pattern_matching(
					grdf_quad_pattern!(?x ?y ?z)
						.as_ref()
						.map(ResourceOrVar::as_ref)
						.into()
				)
				.count(),
			7
		);

		let w = 3;
		assert_eq!(
			dataset
				.pattern_matching(
					grdf_quad_pattern!(?x ?y ?z ?w)
						.as_ref()
						.map(ResourceOrVar::as_ref)
						.into()
				)
				.count(),
			11
		);
		assert_eq!(
			dataset
				.pattern_matching(
					grdf_quad_pattern!(?x ?y ?z _:"graph0")
						.as_ref()
						.map(ResourceOrVar::as_ref)
						.into()
				)
				.count(),
			1
		);

		assert_eq!(
			dataset
				.pattern_matching(
					grdf_quad_pattern!(?x ?y ?z ?x)
						.as_ref()
						.map(ResourceOrVar::as_ref)
						.into()
				)
				.count(),
			1
		);
		assert_eq!(
			dataset
				.pattern_matching(
					grdf_quad_pattern!(?x ?y ?z ?y)
						.as_ref()
						.map(ResourceOrVar::as_ref)
						.into()
				)
				.count(),
			2
		);
	}
}
