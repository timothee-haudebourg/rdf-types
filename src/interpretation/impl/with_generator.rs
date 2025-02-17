use std::borrow::Cow;

use iref::Iri;

use crate::{
	interpretation::{
		GenerativeInterpretation, LocalInterpretation, LocalInterpretationMut,
		ReverseInterpretation, ReverseLocalInterpretation,
	},
	BlankId, CowLiteral, Generator, Interpretation, InterpretationMut, LiteralRef, LocalGenerator,
};

/// Combines any RDF interpretation with a node id generator to make it
/// implement `InterpretationMut`.
///
/// # Use cases
///
/// ## `()` does not implement `InterpretationMut`
///
/// The transparent interpretation (the unit type `()`) does not implement
/// `InterpretationMut` because the `new_resource` method would require the
/// creation of a fresh, unused, blank node identifier, which the transparent
/// interpretation cannot create without a node id generator.
///
/// ## Resources returned by `InterpretationMut::new_resource` have no lexical representation
///
/// Interpreted resources are not required to have a lexical representation.
/// This is most probably the case for new resources returned by
/// `InterpretationMut::new_resource`. You can use `WithGenerator` to pair the
/// interpretation with a node id generator so that `new_resource` will assign
/// a lexical representation to new resources (a fresh blank node id for
/// instance).
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

impl<I: Interpretation, G> Interpretation for WithGenerator<G, I> {
	type Resource = I::Resource;

	fn iri(&self, iri: &Iri) -> Option<Self::Resource> {
		self.interpretation.iri(iri)
	}

	fn literal<'a>(&self, literal: impl Into<LiteralRef<'a>>) -> Option<Self::Resource> {
		self.interpretation.literal(literal)
	}
}

impl<I: InterpretationMut, G> InterpretationMut for WithGenerator<G, I> {
	fn insert_iri<'a>(&mut self, iri: impl Into<Cow<'a, Iri>>) -> Self::Resource {
		self.interpretation.insert_iri(iri)
	}

	fn insert_literal<'a>(&mut self, literal: impl Into<CowLiteral<'a>>) -> Self::Resource {
		self.interpretation.insert_literal(literal)
	}
}

impl<I: ReverseInterpretation, G> ReverseInterpretation for WithGenerator<G, I> {
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

impl<I: InterpretationMut, G: Generator> GenerativeInterpretation for WithGenerator<G, I> {
	fn new_resource(&mut self) -> Self::Resource {
		let term = self.generator.next_term();
		self.interpretation.insert_term(term)
	}
}

pub struct WithLocalGenerator<G, I = ()> {
	interpretation: I,
	generator: G,
}

impl<G, I> WithLocalGenerator<G, I> {
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

impl<I: Interpretation, G> Interpretation for WithLocalGenerator<G, I> {
	type Resource = I::Resource;

	fn iri<'a>(&self, iri: &'a Iri) -> Option<Self::Resource> {
		self.interpretation.iri(iri)
	}

	fn literal<'a>(&self, literal: impl Into<LiteralRef<'a>>) -> Option<Self::Resource> {
		self.interpretation.literal(literal)
	}
}

impl<I: LocalInterpretation, G> LocalInterpretation for WithLocalGenerator<G, I> {
	fn blank_id<'a>(&'a self, blank_id: &'a crate::BlankId) -> Option<Self::Resource> {
		self.interpretation.blank_id(blank_id)
	}
}

impl<I: InterpretationMut, G> InterpretationMut for WithLocalGenerator<G, I> {
	fn insert_iri<'a>(&mut self, iri: impl Into<Cow<'a, Iri>>) -> Self::Resource {
		self.interpretation.insert_iri(iri)
	}

	fn insert_literal<'a>(&mut self, literal: impl Into<CowLiteral<'a>>) -> Self::Resource {
		self.interpretation.insert_literal(literal)
	}
}

impl<I: LocalInterpretationMut, G> LocalInterpretationMut for WithLocalGenerator<G, I> {
	fn insert_blank_id<'a>(&mut self, blank_id: impl Into<Cow<'a, BlankId>>) -> Self::Resource {
		self.interpretation.insert_blank_id(blank_id)
	}
}

impl<I: ReverseInterpretation, G> ReverseInterpretation for WithLocalGenerator<G, I> {
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

impl<I: ReverseLocalInterpretation, G> ReverseLocalInterpretation for WithLocalGenerator<G, I> {
	type BlankIds<'a>
		= I::BlankIds<'a>
	where
		Self: 'a;

	fn blank_ids_of<'a>(&'a self, resource: &'a Self::Resource) -> Self::BlankIds<'a> {
		self.interpretation.blank_ids_of(resource)
	}
}

impl<I: LocalInterpretationMut, G: LocalGenerator> GenerativeInterpretation
	for WithLocalGenerator<G, I>
{
	fn new_resource(&mut self) -> Self::Resource {
		let term = self.generator.next_local_term();
		self.interpretation.insert_local_term(term)
	}
}
