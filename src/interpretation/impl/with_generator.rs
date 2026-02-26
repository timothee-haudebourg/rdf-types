use std::borrow::Cow;

use iref::Iri;

use crate::{
	interpretation::{
		GenerativeInterpretation, Interpretation, InterpretationMut, ReverseGroundInterpretation,
		ReverseInterpretation,
	},
	BlankId, CowLiteral, Generator, GroundInterpretation, GroundInterpretationMut, LiteralRef,
};

/// Combines any RDF interpretation with a node id generator.
///
/// - Provides a [`GenerativeInterpretation`] implementation, using the given
///   generator to create fresh node identifiers.
/// - Assigns a node identifier to each new resource created by
///   [`GenerativeInterpretation::new_resource`], based on the generator.
///
/// Note that in the case of the unit (`()`) interpretation, using
/// [`GeneratorInterpretation`] will give you a similar result but with other
/// advantages (such as a [`ConstGenerativeInterpretation`] implementation).
///
/// [`GeneratorInterpretation`]: super::GeneratorInterpretation
/// [`ConstGenerativeInterpretation`]: crate::interpretation::ConstGenerativeInterpretation
pub struct WithIriGenerator<G, I = ()> {
	interpretation: I,
	generator: G,
}

impl<G, I> WithIriGenerator<G, I> {
	pub fn new(interpretation: I, generator: G) -> Self {
		Self {
			interpretation,
			generator,
		}
	}

	pub fn into_parts(self) -> (I, G) {
		(self.interpretation, self.generator)
	}

	pub fn inner_interpretation(&self) -> &I {
		&self.interpretation
	}

	pub fn inner_interpretation_mut(&mut self) -> &mut I {
		&mut self.interpretation
	}

	pub fn generator(&self) -> &G {
		&self.generator
	}

	pub fn generator_mut(&mut self) -> &mut G {
		&mut self.generator
	}

	pub fn into_inner_interpretation(self) -> I {
		self.interpretation
	}

	pub fn into_generator(self) -> G {
		self.generator
	}
}

impl<I: GroundInterpretation, G> GroundInterpretation for WithIriGenerator<G, I> {
	type Resource = I::Resource;

	fn iri(&self, iri: &Iri) -> Option<Self::Resource> {
		self.interpretation.iri(iri)
	}

	fn literal<'a>(&self, literal: impl Into<LiteralRef<'a>>) -> Option<Self::Resource> {
		self.interpretation.literal(literal)
	}
}

impl<I: GroundInterpretationMut, G> GroundInterpretationMut for WithIriGenerator<G, I> {
	fn insert_iri<'a>(&mut self, iri: impl Into<Cow<'a, Iri>>) -> Self::Resource {
		self.interpretation.insert_iri(iri)
	}

	fn insert_literal<'a>(&mut self, literal: impl Into<CowLiteral<'a>>) -> Self::Resource {
		self.interpretation.insert_literal(literal)
	}
}

impl<I: ReverseGroundInterpretation, G> ReverseGroundInterpretation for WithIriGenerator<G, I> {
	type Iris<'a>
		= I::Iris<'a>
	where
		Self: 'a;
	type Literals<'a>
		= I::Literals<'a>
	where
		Self: 'a;

	fn iris_of<'a>(&'a self, resource: &'a Self::Resource) -> Self::Iris<'a> {
		self.interpretation.iris_of(resource)
	}

	fn literals_of<'a>(&'a self, resource: &'a Self::Resource) -> Self::Literals<'a> {
		self.interpretation.literals_of(resource)
	}
}

impl<I: InterpretationMut, G: Generator> GenerativeInterpretation for WithIriGenerator<G, I> {
	fn new_resource(&mut self) -> Self::Resource {
		let id = self.generator.next_id();
		self.interpretation.insert_id(id)
	}
}

pub struct WithGenerator<G, I = ()> {
	interpretation: I,
	generator: G,
}

impl<G, I> WithGenerator<G, I> {
	pub fn new(interpretation: I, generator: G) -> Self {
		Self {
			interpretation,
			generator,
		}
	}

	pub fn into_parts(self) -> (I, G) {
		(self.interpretation, self.generator)
	}

	pub fn inner_interpretation(&self) -> &I {
		&self.interpretation
	}

	pub fn inner_interpretation_mut(&mut self) -> &mut I {
		&mut self.interpretation
	}

	pub fn generator(&self) -> &G {
		&self.generator
	}

	pub fn generator_mut(&mut self) -> &mut G {
		&mut self.generator
	}

	pub fn into_inner_interpretation(self) -> I {
		self.interpretation
	}

	pub fn into_generator(self) -> G {
		self.generator
	}
}

impl<I: GroundInterpretation, G> GroundInterpretation for WithGenerator<G, I> {
	type Resource = I::Resource;

	fn iri(&self, iri: &Iri) -> Option<Self::Resource> {
		self.interpretation.iri(iri)
	}

	fn literal<'a>(&self, literal: impl Into<LiteralRef<'a>>) -> Option<Self::Resource> {
		self.interpretation.literal(literal)
	}
}

impl<I: Interpretation, G> Interpretation for WithGenerator<G, I> {
	fn blank_id<'a>(&'a self, blank_id: &'a crate::BlankId) -> Option<Self::Resource> {
		self.interpretation.blank_id(blank_id)
	}
}

impl<I: GroundInterpretationMut, G> GroundInterpretationMut for WithGenerator<G, I> {
	fn insert_iri<'a>(&mut self, iri: impl Into<Cow<'a, Iri>>) -> Self::Resource {
		self.interpretation.insert_iri(iri)
	}

	fn insert_literal<'a>(&mut self, literal: impl Into<CowLiteral<'a>>) -> Self::Resource {
		self.interpretation.insert_literal(literal)
	}
}

impl<I: InterpretationMut, G> InterpretationMut for WithGenerator<G, I> {
	fn insert_blank_id<'a>(&mut self, blank_id: impl Into<Cow<'a, BlankId>>) -> Self::Resource {
		self.interpretation.insert_blank_id(blank_id)
	}
}

impl<I: ReverseGroundInterpretation, G> ReverseGroundInterpretation for WithGenerator<G, I> {
	type Iris<'a>
		= I::Iris<'a>
	where
		Self: 'a;
	type Literals<'a>
		= I::Literals<'a>
	where
		Self: 'a;

	fn iris_of<'a>(&'a self, resource: &'a Self::Resource) -> Self::Iris<'a> {
		self.interpretation.iris_of(resource)
	}

	fn literals_of<'a>(&'a self, resource: &'a Self::Resource) -> Self::Literals<'a> {
		self.interpretation.literals_of(resource)
	}
}

impl<I: ReverseInterpretation, G> ReverseInterpretation for WithGenerator<G, I> {
	type BlankIds<'a>
		= I::BlankIds<'a>
	where
		Self: 'a;

	fn blank_ids_of<'a>(&'a self, resource: &'a Self::Resource) -> Self::BlankIds<'a> {
		self.interpretation.blank_ids_of(resource)
	}
}

impl<I: InterpretationMut, G: Generator> GenerativeInterpretation for WithGenerator<G, I> {
	fn new_resource(&mut self) -> Self::Resource {
		let id = self.generator.next_id();
		self.interpretation.insert_id(id)
	}
}
