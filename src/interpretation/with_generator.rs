use crate::{
	BlankIdInterpretation, BlankIdInterpretationMut, BlankIdVocabulary, Generator, Id,
	Interpretation, InterpretationMut, IriInterpretation, IriInterpretationMut, IriVocabulary,
	LiteralInterpretation, LiteralInterpretationMut, ReverseBlankIdInterpretation,
	ReverseBlankIdInterpretationMut, ReverseIriInterpretation, ReverseIriInterpretationMut,
	ReverseLiteralInterpretation, ReverseLiteralInterpretationMut,
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
pub struct WithGenerator<'a, I, G> {
	interpretation: &'a mut I,
	generator: G,
}

impl<'a, I, G> WithGenerator<'a, I, G> {
	pub fn new(interpretation: &'a mut I, generator: G) -> Self {
		Self {
			interpretation,
			generator,
		}
	}
}

impl<'a, I: Interpretation, G> Interpretation for WithGenerator<'a, I, G> {
	type Resource = I::Resource;
}

impl<'a, V: IriVocabulary + BlankIdVocabulary, I, G: Generator<V>> InterpretationMut<V>
	for WithGenerator<'a, I, G>
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

impl<'a, Iri, I: IriInterpretation<Iri>, G> IriInterpretation<Iri> for WithGenerator<'a, I, G> {
	fn iri_interpretation(&self, iri: &Iri) -> Option<Self::Resource> {
		self.interpretation.iri_interpretation(iri)
	}
}

impl<'a, Iri, I: IriInterpretationMut<Iri>, G> IriInterpretationMut<Iri>
	for WithGenerator<'a, I, G>
{
	fn interpret_iri(&mut self, iri: Iri) -> Self::Resource {
		self.interpretation.interpret_iri(iri)
	}
}

impl<'i, I: ReverseIriInterpretation, G> ReverseIriInterpretation for WithGenerator<'i, I, G> {
	type Iri = I::Iri;
	type Iris<'a> = I::Iris<'a> where Self: 'a;

	fn iris_of<'a>(&'a self, id: &'a Self::Resource) -> Self::Iris<'a> {
		self.interpretation.iris_of(id)
	}
}

impl<'i, I: ReverseIriInterpretationMut, G> ReverseIriInterpretationMut
	for WithGenerator<'i, I, G>
{
	fn assign_iri(&mut self, id: Self::Resource, iri: Self::Iri) -> bool {
		self.interpretation.assign_iri(id, iri)
	}
}

impl<'a, B, I: BlankIdInterpretation<B>, G> BlankIdInterpretation<B> for WithGenerator<'a, I, G> {
	fn blank_id_interpretation(&self, blank_id: &B) -> Option<Self::Resource> {
		self.interpretation.blank_id_interpretation(blank_id)
	}
}

impl<'a, B, I: BlankIdInterpretationMut<B>, G> BlankIdInterpretationMut<B>
	for WithGenerator<'a, I, G>
{
	fn interpret_blank_id(&mut self, blank_id: B) -> Self::Resource {
		self.interpretation.interpret_blank_id(blank_id)
	}
}

impl<'i, I: ReverseBlankIdInterpretation, G> ReverseBlankIdInterpretation
	for WithGenerator<'i, I, G>
{
	type BlankId = I::BlankId;
	type BlankIds<'a> = I::BlankIds<'a> where Self: 'a;

	fn blank_ids_of<'a>(&'a self, id: &'a Self::Resource) -> Self::BlankIds<'a> {
		self.interpretation.blank_ids_of(id)
	}
}

impl<'i, I: ReverseBlankIdInterpretationMut, G> ReverseBlankIdInterpretationMut
	for WithGenerator<'i, I, G>
{
	fn assign_blank_id(&mut self, id: Self::Resource, blank_id: Self::BlankId) -> bool {
		self.interpretation.assign_blank_id(id, blank_id)
	}
}

impl<'a, L, I: LiteralInterpretation<L>, G> LiteralInterpretation<L> for WithGenerator<'a, I, G> {
	fn literal_interpretation(&self, literal: &L) -> Option<Self::Resource> {
		self.interpretation.literal_interpretation(literal)
	}
}

impl<'a, L, I: LiteralInterpretationMut<L>, G> LiteralInterpretationMut<L>
	for WithGenerator<'a, I, G>
{
	fn interpret_literal(&mut self, literal: L) -> Self::Resource {
		self.interpretation.interpret_literal(literal)
	}
}

impl<'i, I: ReverseLiteralInterpretation, G> ReverseLiteralInterpretation
	for WithGenerator<'i, I, G>
{
	type Literal = I::Literal;
	type Literals<'a> = I::Literals<'a> where Self: 'a;

	fn literals_of<'a>(&'a self, id: &'a Self::Resource) -> Self::Literals<'a> {
		self.interpretation.literals_of(id)
	}
}

impl<'i, I: ReverseLiteralInterpretationMut, G> ReverseLiteralInterpretationMut
	for WithGenerator<'i, I, G>
{
	fn assign_literal(&mut self, resource: Self::Resource, literal: Self::Literal) -> bool {
		self.interpretation.assign_literal(resource, literal)
	}
}
