use std::collections::{HashMap, HashSet};

use crate::interpretation::{
	BlankIdInterpretation, BlankIdInterpretationMut, IriInterpretation, IriInterpretationMut,
	LiteralInterpretation, LiteralInterpretationMut, ReverseBlankIdInterpretation,
	ReverseBlankIdInterpretationMut, ReverseIriInterpretation, ReverseIriInterpretationMut,
	ReverseLiteralInterpretation, ReverseLiteralInterpretationMut, TraversableInterpretation,
};
use crate::vocabulary::{BlankIdIndex, IriIndex, LiteralIndex};
use crate::{Interpretation, InterpretationMut};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ResourceIndex(usize);

impl From<usize> for ResourceIndex {
	fn from(i: usize) -> Self {
		Self(i)
	}
}

impl From<ResourceIndex> for usize {
	fn from(value: ResourceIndex) -> Self {
		value.0
	}
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Resource {
	index: ResourceIndex,
	iris: HashSet<IriIndex>,
	blank_ids: HashSet<BlankIdIndex>,
	literals: HashSet<LiteralIndex>,
}

impl Resource {
	fn new(index: ResourceIndex) -> Self {
		Self {
			index,
			iris: HashSet::new(),
			blank_ids: HashSet::new(),
			literals: HashSet::new(),
		}
	}
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
struct Resources(Vec<Resource>);

impl Resources {
	fn len(&self) -> usize {
		self.0.len()
	}

	fn is_empty(&self) -> bool {
		self.0.is_empty()
	}

	fn get(&self, i: ResourceIndex) -> Option<&Resource> {
		self.0.get(i.0)
	}

	fn get_mut(&mut self, i: ResourceIndex) -> Option<&mut Resource> {
		self.0.get_mut(i.0)
	}

	fn insert(&mut self) -> (ResourceIndex, &mut Resource) {
		let i = ResourceIndex(self.0.len());
		self.0.push(Resource::new(i));
		let r = self.0.last_mut().unwrap();
		(i, r)
	}
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Indexed {
	resources: Resources,
	by_iri: HashMap<IriIndex, ResourceIndex>,
	by_blank_id: HashMap<BlankIdIndex, ResourceIndex>,
	by_literal: HashMap<LiteralIndex, ResourceIndex>,
}

impl Indexed {
	pub fn new() -> Self {
		Self::default()
	}

	pub fn len(&self) -> usize {
		self.resources.len()
	}

	pub fn is_empty(&self) -> bool {
		self.resources.is_empty()
	}
}

impl Interpretation for Indexed {
	type Resource = ResourceIndex;
}

impl TraversableInterpretation for Indexed {
	type Resources<'a> = ResourceIndexIter<'a>;

	fn resources(&self) -> Self::Resources<'_> {
		ResourceIndexIter(self.resources.0.iter())
	}
}

impl<V> InterpretationMut<V> for Indexed {
	fn new_resource(&mut self, _vocabulary: &mut V) -> Self::Resource {
		self.resources.insert().0
	}
}

pub struct ResourceIndexIter<'a>(std::slice::Iter<'a, Resource>);

impl<'a> Iterator for ResourceIndexIter<'a> {
	type Item = &'a ResourceIndex;

	fn next(&mut self) -> Option<Self::Item> {
		self.0.next().map(|r| &r.index)
	}
}

impl IriInterpretation<IriIndex> for Indexed {
	fn iri_interpretation(&self, iri: &IriIndex) -> Option<Self::Resource> {
		self.by_iri.get(iri).copied()
	}
}

impl IriInterpretationMut<IriIndex> for Indexed {
	fn interpret_iri(&mut self, iri: IriIndex) -> Self::Resource {
		*self.by_iri.entry(iri).or_insert_with(|| {
			let (i, r) = self.resources.insert();
			r.iris.insert(iri);
			i
		})
	}
}

impl BlankIdInterpretation<BlankIdIndex> for Indexed {
	fn blank_id_interpretation(&self, blank_id: &BlankIdIndex) -> Option<Self::Resource> {
		self.by_blank_id.get(blank_id).copied()
	}
}

impl BlankIdInterpretationMut<BlankIdIndex> for Indexed {
	fn interpret_blank_id(&mut self, blank_id: BlankIdIndex) -> Self::Resource {
		*self.by_blank_id.entry(blank_id).or_insert_with(|| {
			let (i, r) = self.resources.insert();
			r.blank_ids.insert(blank_id);
			i
		})
	}
}

impl LiteralInterpretation<LiteralIndex> for Indexed {
	fn literal_interpretation(&self, literal: &LiteralIndex) -> Option<Self::Resource> {
		self.by_literal.get(literal).copied()
	}
}

impl LiteralInterpretationMut<LiteralIndex> for Indexed {
	fn interpret_literal(&mut self, literal: LiteralIndex) -> Self::Resource {
		*self.by_literal.entry(literal).or_insert_with(|| {
			let (i, r) = self.resources.insert();
			r.literals.insert(literal);
			i
		})
	}
}

impl ReverseIriInterpretation for Indexed {
	type Iri = IriIndex;

	type Iris<'a> =
		std::iter::Flatten<std::option::IntoIter<std::collections::hash_set::Iter<'a, IriIndex>>>;

	fn iris_of(&self, id: &Self::Resource) -> Self::Iris<'_> {
		self.resources
			.get(*id)
			.map(|r| r.iris.iter())
			.into_iter()
			.flatten()
	}
}

impl ReverseBlankIdInterpretation for Indexed {
	type BlankId = BlankIdIndex;

	type BlankIds<'a> = std::iter::Flatten<
		std::option::IntoIter<std::collections::hash_set::Iter<'a, BlankIdIndex>>,
	>;

	fn blank_ids_of(&self, id: &Self::Resource) -> Self::BlankIds<'_> {
		self.resources
			.get(*id)
			.map(|r| r.blank_ids.iter())
			.into_iter()
			.flatten()
	}
}

impl ReverseLiteralInterpretation for Indexed {
	type Literal = LiteralIndex;

	type Literals<'a> = std::iter::Flatten<
		std::option::IntoIter<std::collections::hash_set::Iter<'a, LiteralIndex>>,
	>;

	fn literals_of(&self, id: &Self::Resource) -> Self::Literals<'_> {
		self.resources
			.get(*id)
			.map(|r| r.literals.iter())
			.into_iter()
			.flatten()
	}
}

impl ReverseIriInterpretationMut for Indexed {
	fn assign_iri(&mut self, resource: &Self::Resource, iri: Self::Iri) -> bool {
		let r = self.resources.get_mut(*resource).unwrap();
		r.iris.insert(iri)
	}
}

impl ReverseBlankIdInterpretationMut for Indexed {
	fn assign_blank_id(&mut self, resource: &Self::Resource, blank_id: Self::BlankId) -> bool {
		let r = self.resources.get_mut(*resource).unwrap();
		r.blank_ids.insert(blank_id)
	}
}

impl ReverseLiteralInterpretationMut for Indexed {
	fn assign_literal(&mut self, resource: &Self::Resource, literal: Self::Literal) -> bool {
		self.resources
			.get_mut(*resource)
			.unwrap()
			.literals
			.insert(literal)
	}
}
