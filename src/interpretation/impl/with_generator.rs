use crate::{
	interpretation::{
		BlankIdInterpretation, BlankIdInterpretationMut, IriInterpretation, IriInterpretationMut,
		LiteralInterpretation, LiteralInterpretationMut, ReverseBlankIdInterpretation,
		ReverseBlankIdInterpretationMut, ReverseIriInterpretation, ReverseIriInterpretationMut,
		ReverseLiteralInterpretation, ReverseLiteralInterpretationMut,
	},
	vocabulary::{BlankIdVocabulary, IriVocabulary},
	Generator, Id, Interpretation, InterpretationMut,
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
}

impl<V: IriVocabulary + BlankIdVocabulary, I, G: Generator<V>> InterpretationMut<V>
	for WithGenerator<G, I>
where
	I: IriInterpretationMut<V::Iri> + BlankIdInterpretationMut<V::BlankId>,
{
	fn new_resource(&mut self, vocabulary: &mut V) -> Self::Resource {
		match self.generator.next(vocabulary) {
			Id::Iri(i) => self.interpretation.interpret_iri(i),
			Id::Blank(b) => self.interpretation.interpret_blank_id(b),
		}
	}
}

impl<Iri, I: IriInterpretation<Iri>, G> IriInterpretation<Iri> for WithGenerator<G, I> {
	fn iri_interpretation(&self, iri: &Iri) -> Option<Self::Resource> {
		self.interpretation.iri_interpretation(iri)
	}
}

impl<Iri, I: IriInterpretationMut<Iri>, G> IriInterpretationMut<Iri> for WithGenerator<G, I> {
	fn interpret_iri(&mut self, iri: Iri) -> Self::Resource {
		self.interpretation.interpret_iri(iri)
	}
}

impl<I: ReverseIriInterpretation, G> ReverseIriInterpretation for WithGenerator<G, I> {
	type Iri = I::Iri;
	type Iris<'a> = I::Iris<'a> where Self: 'a;

	fn iris_of<'a>(&'a self, id: &'a Self::Resource) -> Self::Iris<'a> {
		self.interpretation.iris_of(id)
	}
}

impl<I: ReverseIriInterpretationMut, G> ReverseIriInterpretationMut for WithGenerator<G, I> {
	fn assign_iri(&mut self, id: &Self::Resource, iri: Self::Iri) -> bool {
		self.interpretation.assign_iri(id, iri)
	}
}

impl<B, I: BlankIdInterpretation<B>, G> BlankIdInterpretation<B> for WithGenerator<G, I> {
	fn blank_id_interpretation(&self, blank_id: &B) -> Option<Self::Resource> {
		self.interpretation.blank_id_interpretation(blank_id)
	}
}

impl<B, I: BlankIdInterpretationMut<B>, G> BlankIdInterpretationMut<B> for WithGenerator<G, I> {
	fn interpret_blank_id(&mut self, blank_id: B) -> Self::Resource {
		self.interpretation.interpret_blank_id(blank_id)
	}
}

impl<I: ReverseBlankIdInterpretation, G> ReverseBlankIdInterpretation for WithGenerator<G, I> {
	type BlankId = I::BlankId;
	type BlankIds<'a> = I::BlankIds<'a> where Self: 'a;

	fn blank_ids_of<'a>(&'a self, id: &'a Self::Resource) -> Self::BlankIds<'a> {
		self.interpretation.blank_ids_of(id)
	}
}

impl<I: ReverseBlankIdInterpretationMut, G> ReverseBlankIdInterpretationMut
	for WithGenerator<G, I>
{
	fn assign_blank_id(&mut self, id: &Self::Resource, blank_id: Self::BlankId) -> bool {
		self.interpretation.assign_blank_id(id, blank_id)
	}
}

impl<L, I: LiteralInterpretation<L>, G> LiteralInterpretation<L> for WithGenerator<G, I> {
	fn literal_interpretation(&self, literal: &L) -> Option<Self::Resource> {
		self.interpretation.literal_interpretation(literal)
	}
}

impl<L, I: LiteralInterpretationMut<L>, G> LiteralInterpretationMut<L> for WithGenerator<G, I> {
	fn interpret_literal(&mut self, literal: L) -> Self::Resource {
		self.interpretation.interpret_literal(literal)
	}
}

impl<I: ReverseLiteralInterpretation, G> ReverseLiteralInterpretation for WithGenerator<G, I> {
	type Literal = I::Literal;
	type Literals<'a> = I::Literals<'a> where Self: 'a;

	fn literals_of<'a>(&'a self, id: &'a Self::Resource) -> Self::Literals<'a> {
		self.interpretation.literals_of(id)
	}
}

impl<I: ReverseLiteralInterpretationMut, G> ReverseLiteralInterpretationMut
	for WithGenerator<G, I>
{
	fn assign_literal(&mut self, resource: &Self::Resource, literal: Self::Literal) -> bool {
		self.interpretation.assign_literal(resource, literal)
	}
}
