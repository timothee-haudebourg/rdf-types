use iref::{Iri, IriBuf};
use langtag::LanguageTagBuf;

use crate::{
	literal, BlankId, BlankIdBuf, BlankIdVocabulary, BlankIdVocabularyMut, Generator, Id,
	IriVocabulary, IriVocabularyMut, LanguageTagVocabularyMut, Literal, LiteralVocabulary,
	LiteralVocabularyMut, Namespace, Quad, Term,
};

mod indexed;
mod none;

pub use indexed::*;

pub type UninterpretedIdRef<'a, I> =
	Id<&'a <I as ReverseIriInterpretation>::Iri, &'a <I as ReverseBlankIdInterpretation>::BlankId>;

pub type UninterpretedTermRef<'a, I> =
	Term<UninterpretedIdRef<'a, I>, &'a <I as ReverseLiteralInterpretation>::Literal>;

pub type UninterpretedQuadRef<'a, I> = Quad<
	UninterpretedIdRef<'a, I>,
	&'a <I as ReverseIriInterpretation>::Iri,
	UninterpretedTermRef<'a, I>,
	UninterpretedIdRef<'a, I>,
>;

pub type UninterpretedGrdfQuadRef<'a, I> = Quad<
	UninterpretedTermRef<'a, I>,
	UninterpretedTermRef<'a, I>,
	UninterpretedTermRef<'a, I>,
	UninterpretedTermRef<'a, I>,
>;

/// RDF resource interpretation.
pub trait Interpretation {
	/// Resource identifier type.
	type Resource;
}

pub trait TraversableInterpretation: Interpretation {
	/// Interpreted resource iterator.
	type Resources<'a>: 'a + Iterator<Item = Self::Resource>
	where
		Self: 'a;

	/// Returns an iterator over the interpreted resources.
	fn resources(&self) -> Self::Resources<'_>;
}

/// IRI Interpretation.
pub trait IriInterpretation<I: ?Sized>: Interpretation {
	/// Returns the interpretation of the given IRI, if any.
	fn iri_interpretation(&self, iri: &I) -> Option<Self::Resource>;

	fn lexical_iri_interpretation(
		&self,
		vocabulary: &impl IriVocabulary<Iri = I>,
		iri: &Iri,
	) -> Option<Self::Resource>
	where
		I: Sized,
	{
		vocabulary
			.get(iri)
			.and_then(|iri| self.iri_interpretation(&iri))
	}
}

pub trait ReverseIriInterpretation: Interpretation {
	type Iri;
	type Iris<'a>: Clone + Iterator<Item = &'a Self::Iri>
	where
		Self: 'a;

	fn iris_of<'a>(&'a self, id: &'a Self::Resource) -> Self::Iris<'a>;
}

pub trait ReverseIriInterpretationMut: ReverseIriInterpretation {
	fn assign_iri(&mut self, id: Self::Resource, iri: Self::Iri) -> bool;
}

/// Blank node identifier interpretation.
pub trait BlankIdInterpretation<B: ?Sized>: Interpretation {
	/// Returns the interpretation of the given blank node identifier, if any.
	fn blank_id_interpretation(&self, blank_id: &B) -> Option<Self::Resource>;

	fn lexical_blank_id_interpretation(
		&self,
		vocabulary: &impl BlankIdVocabulary<BlankId = B>,
		blank_id: &BlankId,
	) -> Option<Self::Resource>
	where
		B: Sized,
	{
		vocabulary
			.get_blank_id(blank_id)
			.and_then(|blank_id| self.blank_id_interpretation(&blank_id))
	}
}

pub trait ReverseBlankIdInterpretation: Interpretation {
	type BlankId;
	type BlankIds<'a>: Clone + Iterator<Item = &'a Self::BlankId>
	where
		Self: 'a;

	fn blank_ids_of<'a>(&'a self, id: &'a Self::Resource) -> Self::BlankIds<'a>;
}

pub trait ReverseBlankIdInterpretationMut: ReverseBlankIdInterpretation {
	fn assign_blank_id(&mut self, id: Self::Resource, blank_id: Self::BlankId) -> bool;
}

/// Node identifier interpretation.
pub trait IdInterpretation<I, B>: Interpretation {
	/// Returns the interpretation of the given node identifier, if any.
	fn id_interpretation(&self, id: &Id<I, B>) -> Option<Self::Resource>;
}

impl<I, B, T: IriInterpretation<I> + BlankIdInterpretation<B>> IdInterpretation<I, B> for T {
	fn id_interpretation(&self, id: &Id<I, B>) -> Option<Self::Resource> {
		match id {
			Id::Iri(i) => self.iri_interpretation(i),
			Id::Blank(b) => self.blank_id_interpretation(b),
		}
	}
}

pub trait LexicalIdInterpretation<V>: Interpretation {
	fn lexical_id_interpretation(
		&self,
		vocabulary: &V,
		id: Id<&Iri, &BlankId>,
	) -> Option<Self::Resource>;
}

impl<
		V: IriVocabulary + BlankIdVocabulary,
		T: IriInterpretation<V::Iri> + BlankIdInterpretation<V::BlankId>,
	> LexicalIdInterpretation<V> for T
{
	fn lexical_id_interpretation(
		&self,
		vocabulary: &V,
		id: Id<&Iri, &BlankId>,
	) -> Option<Self::Resource> {
		match id {
			Id::Iri(i) => self.lexical_iri_interpretation(vocabulary, i),
			Id::Blank(b) => self.lexical_blank_id_interpretation(vocabulary, b),
		}
	}
}

/// Literal value interpretation.
pub trait LiteralInterpretation<L>: Interpretation {
	/// Returns the interpretation of the given literal value, if any.
	fn literal_interpretation(&self, literal: &L) -> Option<Self::Resource>;

	fn lexical_literal_interpretation<V: LiteralVocabulary<Literal = L>>(
		&self,
		vocabulary: &V,
		literal: &Literal<V::Type, V::Value>,
	) -> Option<Self::Resource> {
		vocabulary
			.get_literal(literal)
			.and_then(|l| self.literal_interpretation(&l))
	}
}

/// RDF Term interpretation.
pub trait TermInterpretation<I, B, L = Literal>:
	IdInterpretation<I, B> + LiteralInterpretation<L>
{
	/// Returns the interpretation of the given term, if any.
	fn term_interpretation(&self, term: &Term<Id<I, B>, L>) -> Option<Self::Resource> {
		match term {
			Term::Id(id) => self.id_interpretation(id),
			Term::Literal(l) => self.literal_interpretation(l),
		}
	}
}

pub trait LexicalTermInterpretation<V: LiteralVocabulary>: Interpretation {
	#[allow(clippy::type_complexity)]
	fn lexical_term_interpretation(
		&self,
		vocabulary: &V,
		term: Term<Id<&Iri, &BlankId>, &Literal<V::Type, V::Value>>,
	) -> Option<Self::Resource>;
}

impl<V: LiteralVocabulary, I: LexicalIdInterpretation<V> + LiteralInterpretation<V::Literal>>
	LexicalTermInterpretation<V> for I
{
	fn lexical_term_interpretation(
		&self,
		vocabulary: &V,
		term: Term<Id<&Iri, &BlankId>, &Literal<V::Type, V::Value>>,
	) -> Option<Self::Resource> {
		match term {
			Term::Id(id) => self.lexical_id_interpretation(vocabulary, id),
			Term::Literal(l) => self.lexical_literal_interpretation(vocabulary, l),
		}
	}
}

/// Mutable RDF resource interpretation.
pub trait InterpretationMut: Interpretation {
	/// Creates a new resource.
	fn new_resource(&mut self) -> Self::Resource;
}

/// Mutable IRI interpretation.
pub trait IriInterpretationMut<I = IriBuf>: Interpretation {
	/// Interprets the given IRI.
	fn interpret_iri(&mut self, iri: I) -> Self::Resource;

	fn interpret_lexical_iri(
		&mut self,
		vocabulary: &mut impl IriVocabularyMut<Iri = I>,
		iri: &Iri,
	) -> Self::Resource {
		self.interpret_iri(vocabulary.insert(iri))
	}

	fn interpret_owned_lexical_iri(
		&mut self,
		vocabulary: &mut impl IriVocabularyMut<Iri = I>,
		iri: IriBuf,
	) -> Self::Resource {
		self.interpret_iri(vocabulary.insert_owned(iri))
	}
}

/// Blank node identifier interpretation.
pub trait BlankIdInterpretationMut<B = BlankIdBuf>: Interpretation {
	/// Interprets the given blank node identifier.
	fn interpret_blank_id(&mut self, blank_id: B) -> Self::Resource;

	fn interpret_lexical_blank_id(
		&mut self,
		vocabulary: &mut impl BlankIdVocabularyMut<BlankId = B>,
		blank_id: &BlankId,
	) -> Self::Resource {
		self.interpret_blank_id(vocabulary.insert_blank_id(blank_id))
	}

	fn interpret_owned_lexical_blank_id(
		&mut self,
		vocabulary: &mut impl BlankIdVocabularyMut<BlankId = B>,
		blank_id: BlankIdBuf,
	) -> Self::Resource {
		self.interpret_blank_id(vocabulary.insert_owned_blank_id(blank_id))
	}
}

/// Node identifier interpretation.
pub trait IdInterpretationMut<I, B>: IriInterpretationMut<I> + BlankIdInterpretationMut<B> {
	/// Interprets the given identifier.
	fn interpret_id(&mut self, id: Id<I, B>) -> Self::Resource;
}

impl<I, B, T: IriInterpretationMut<I> + BlankIdInterpretationMut<B>> IdInterpretationMut<I, B>
	for T
{
	fn interpret_id(&mut self, id: Id<I, B>) -> T::Resource {
		match id {
			Id::Iri(i) => self.interpret_iri(i),
			Id::Blank(b) => self.interpret_blank_id(b),
		}
	}
}

pub trait LexicalIdInterpretationMut<V>: Interpretation {
	fn interpret_lexical_id(
		&mut self,
		vocabulary: &mut V,
		id: Id<&Iri, &BlankId>,
	) -> Self::Resource;

	fn interpret_owned_lexical_id(
		&mut self,
		vocabulary: &mut V,
		id: Id<IriBuf, BlankIdBuf>,
	) -> Self::Resource;
}

impl<
		V: IriVocabularyMut + BlankIdVocabularyMut,
		I: IriInterpretationMut<V::Iri> + BlankIdInterpretationMut<V::BlankId>,
	> LexicalIdInterpretationMut<V> for I
{
	fn interpret_lexical_id(
		&mut self,
		vocabulary: &mut V,
		id: Id<&Iri, &BlankId>,
	) -> Self::Resource {
		match id {
			Id::Iri(i) => self.interpret_lexical_iri(vocabulary, i),
			Id::Blank(b) => self.interpret_lexical_blank_id(vocabulary, b),
		}
	}

	fn interpret_owned_lexical_id(
		&mut self,
		vocabulary: &mut V,
		id: Id<IriBuf, BlankIdBuf>,
	) -> Self::Resource {
		match id {
			Id::Iri(i) => self.interpret_owned_lexical_iri(vocabulary, i),
			Id::Blank(b) => self.interpret_owned_lexical_blank_id(vocabulary, b),
		}
	}
}

/// Mutable literal value interpretation.
pub trait LiteralInterpretationMut<L = Literal>: Interpretation {
	/// Interprets the given literal value.
	fn interpret_literal(&mut self, literal: L) -> Self::Resource;

	fn interpret_lexical_literal<V: LiteralVocabularyMut<Literal = L>>(
		&mut self,
		vocabulary: &mut V,
		literal: &Literal<V::Type, V::Value>,
	) -> Self::Resource {
		self.interpret_literal(vocabulary.insert_literal(literal))
	}

	fn interpret_owned_lexical_literal<V: LiteralVocabularyMut<Literal = L>>(
		&mut self,
		vocabulary: &mut V,
		literal: Literal<V::Type, V::Value>,
	) -> Self::Resource {
		self.interpret_literal(vocabulary.insert_owned_literal(literal))
	}

	fn interpret_full_lexical_literal<V>(
		&mut self,
		vocabulary: &mut V,
		literal: Literal<literal::Type<IriBuf, LanguageTagBuf>, V::Value>,
	) -> Self::Resource
	where
		V: LiteralVocabularyMut<Literal = L, Type = literal::Type<V::Iri, V::LanguageTag>>
			+ IriVocabularyMut
			+ LanguageTagVocabularyMut,
	{
		let (value, type_) = literal.into_parts();
		let type_ = match type_ {
			literal::Type::Any(ty) => literal::Type::Any(vocabulary.insert_owned(ty)),
			literal::Type::LangString(tag) => {
				literal::Type::LangString(vocabulary.insert_owned_language_tag(tag))
			}
		};

		self.interpret_literal(vocabulary.insert_owned_literal(Literal::new(value, type_)))
	}
}

pub trait TermInterpretationMut<I, B, L = Literal>:
	IdInterpretationMut<I, B> + LiteralInterpretationMut<L>
{
	fn interpret_term(&mut self, term: Term<Id<I, B>, L>) -> Self::Resource {
		match term {
			Term::Id(id) => self.interpret_id(id),
			Term::Literal(l) => self.interpret_literal(l),
		}
	}
}

pub trait LexicalTermInterpretationMut<V: LiteralVocabulary>: Interpretation {
	#[allow(clippy::type_complexity)]
	fn interpret_lexical_term(
		&mut self,
		vocabulary: &mut V,
		term: Term<Id<&Iri, &BlankId>, &Literal<V::Type, V::Value>>,
	) -> Self::Resource;

	#[allow(clippy::type_complexity)]
	fn interpret_owned_lexical_term(
		&mut self,
		vocabulary: &mut V,
		term: Term<Id<IriBuf, BlankIdBuf>, Literal<V::Type, V::Value>>,
	) -> Self::Resource;

	#[allow(clippy::type_complexity)]
	fn interpret_full_lexical_term(
		&mut self,
		vocabulary: &mut V,
		term: Term<
			Id<IriBuf, BlankIdBuf>,
			Literal<literal::Type<IriBuf, LanguageTagBuf>, V::Value>,
		>,
	) -> Self::Resource
	where
		V: LiteralVocabularyMut<Type = literal::Type<V::Iri, V::LanguageTag>>
			+ IriVocabularyMut
			+ LanguageTagVocabularyMut;
}

impl<
		V: LiteralVocabularyMut,
		I: LexicalIdInterpretationMut<V> + LiteralInterpretationMut<V::Literal>,
	> LexicalTermInterpretationMut<V> for I
{
	fn interpret_lexical_term(
		&mut self,
		vocabulary: &mut V,
		term: Term<
			Id<&Iri, &BlankId>,
			&Literal<<V as LiteralVocabulary>::Type, <V as LiteralVocabulary>::Value>,
		>,
	) -> Self::Resource {
		match term {
			Term::Id(id) => self.interpret_lexical_id(vocabulary, id),
			Term::Literal(l) => self.interpret_lexical_literal(vocabulary, l),
		}
	}

	fn interpret_owned_lexical_term(
		&mut self,
		vocabulary: &mut V,
		term: Term<Id<IriBuf, BlankIdBuf>, Literal<V::Type, V::Value>>,
	) -> Self::Resource {
		match term {
			Term::Id(id) => self.interpret_owned_lexical_id(vocabulary, id),
			Term::Literal(l) => self.interpret_owned_lexical_literal(vocabulary, l),
		}
	}

	fn interpret_full_lexical_term(
		&mut self,
		vocabulary: &mut V,
		term: Term<
			Id<IriBuf, BlankIdBuf>,
			Literal<literal::Type<IriBuf, LanguageTagBuf>, V::Value>,
		>,
	) -> Self::Resource
	where
		V: LiteralVocabularyMut<Type = literal::Type<V::Iri, V::LanguageTag>>
			+ IriVocabularyMut
			+ LanguageTagVocabularyMut,
	{
		match term {
			Term::Id(id) => self.interpret_owned_lexical_id(vocabulary, id),
			Term::Literal(l) => self.interpret_full_lexical_literal(vocabulary, l),
		}
	}
}

/// Reverse node identifier interpretation.
///
/// Used to retrieve the node identifiers of a given resource.
pub trait ReverseIdInterpretation: ReverseIriInterpretation + ReverseBlankIdInterpretation {
	fn ids_of<'a>(&'a self, id: &'a Self::Resource) -> IdsOf<'a, Self> {
		IdsOf {
			iris: self.iris_of(id),
			blanks: self.blank_ids_of(id),
		}
	}
}

impl<I: ?Sized + ReverseIriInterpretation + ReverseBlankIdInterpretation> ReverseIdInterpretation
	for I
{
}

pub struct IdsOf<'a, I: 'a + ?Sized + ReverseIdInterpretation> {
	iris: I::Iris<'a>,
	blanks: I::BlankIds<'a>,
}

impl<'a, I: 'a + ?Sized + ReverseIdInterpretation> Clone for IdsOf<'a, I> {
	fn clone(&self) -> Self {
		Self {
			iris: self.iris.clone(),
			blanks: self.blanks.clone(),
		}
	}
}

impl<'a, I: 'a + ?Sized + ReverseIdInterpretation> Copy for IdsOf<'a, I>
where
	I::Iris<'a>: Copy,
	I::BlankIds<'a>: Copy,
{
}

impl<'a, I: 'a + ?Sized + ReverseIdInterpretation> Iterator for IdsOf<'a, I> {
	type Item = Id<&'a I::Iri, &'a I::BlankId>;

	fn next(&mut self) -> Option<Self::Item> {
		self.iris
			.next()
			.map(Id::Iri)
			.or_else(|| self.blanks.next().map(Id::Blank))
	}
}

pub trait ReverseIdInterpretationMut:
	ReverseIriInterpretationMut + ReverseBlankIdInterpretationMut
{
	fn assign_id(&mut self, r: Self::Resource, id: Id<Self::Iri, Self::BlankId>) -> bool {
		match id {
			Id::Iri(i) => self.assign_iri(r, i),
			Id::Blank(b) => self.assign_blank_id(r, b),
		}
	}
}

impl<I: ?Sized + ReverseIriInterpretationMut + ReverseBlankIdInterpretationMut>
	ReverseIdInterpretationMut for I
{
}

pub trait ReverseLiteralInterpretation: Interpretation {
	type Literal;

	type Literals<'a>: Clone + Iterator<Item = &'a Self::Literal>
	where
		Self: 'a;

	fn literals_of<'a>(&'a self, id: &'a Self::Resource) -> Self::Literals<'a>;
}

pub type TermOf<'a, I> = Term<
	Id<&'a <I as ReverseIriInterpretation>::Iri, &'a <I as ReverseBlankIdInterpretation>::BlankId>,
	&'a <I as ReverseLiteralInterpretation>::Literal,
>;

pub trait ReverseTermInterpretation:
	ReverseIdInterpretation + ReverseLiteralInterpretation
{
	fn terms_of<'a>(&'a self, id: &'a Self::Resource) -> TermsOf<'a, Self> {
		TermsOf {
			ids: self.ids_of(id),
			literals: self.literals_of(id),
		}
	}

	fn term_of<'a>(&'a self, id: &'a Self::Resource) -> Option<TermOf<'a, Self>> {
		self.terms_of(id).next()
	}

	fn has_term(&self, id: &Self::Resource) -> bool {
		self.term_of(id).is_some()
	}

	fn quads_of<'a>(
		&'a self,
		quad: Quad<&'a Self::Resource, &'a Self::Resource, &'a Self::Resource, &'a Self::Resource>,
	) -> QuadsOf<'a, Self> {
		QuadsOf {
			s: self.ids_of(quad.0),
			p: self.iris_of(quad.1),
			o: self.terms_of(quad.2),
			g: quad.3.map(|g| self.ids_of(g)),
			pogs: None,
		}
	}

	fn grdf_quads_of<'a>(
		&'a self,
		quad: Quad<&'a Self::Resource, &'a Self::Resource, &'a Self::Resource, &'a Self::Resource>,
	) -> GrdfQuadsOf<'a, Self> {
		GrdfQuadsOf {
			s: self.terms_of(quad.0),
			p: self.terms_of(quad.1),
			o: self.terms_of(quad.2),
			g: quad.3.map(|g| self.terms_of(g)),
			pogs: None,
		}
	}
}

pub struct QuadsOf<'a, I: ?Sized + ReverseTermInterpretation> {
	s: IdsOf<'a, I>,
	p: I::Iris<'a>,
	o: TermsOf<'a, I>,
	g: Option<IdsOf<'a, I>>,
	pogs: Option<PogsOf<'a, I>>,
}

impl<'a, I: ?Sized + ReverseTermInterpretation> Iterator for QuadsOf<'a, I> {
	type Item = Quad<
		Id<&'a I::Iri, &'a I::BlankId>,
		&'a I::Iri,
		Term<Id<&'a I::Iri, &'a I::BlankId>, &'a I::Literal>,
		Id<&'a I::Iri, &'a I::BlankId>,
	>;

	fn next(&mut self) -> Option<Self::Item> {
		loop {
			match self.pogs.as_mut() {
				Some(pogs) => match pogs.next() {
					Some(quad) => break Some(quad),
					None => self.pogs = None,
				},
				None => match self.s.next() {
					Some(s) => {
						self.pogs = Some(PogsOf {
							s,
							p: self.p.clone(),
							o: self.o.clone(),
							g: self.g.clone(),
							ogs: None,
						})
					}
					None => break None,
				},
			}
		}
	}
}

struct PogsOf<'a, I: ?Sized + ReverseTermInterpretation> {
	s: Id<&'a I::Iri, &'a I::BlankId>,
	p: I::Iris<'a>,
	o: TermsOf<'a, I>,
	g: Option<IdsOf<'a, I>>,
	ogs: Option<OgsOf<'a, I>>,
}

impl<'a, I: ?Sized + ReverseTermInterpretation> Iterator for PogsOf<'a, I> {
	type Item = Quad<
		Id<&'a I::Iri, &'a I::BlankId>,
		&'a I::Iri,
		Term<Id<&'a I::Iri, &'a I::BlankId>, &'a I::Literal>,
		Id<&'a I::Iri, &'a I::BlankId>,
	>;

	fn next(&mut self) -> Option<Self::Item> {
		loop {
			match self.ogs.as_mut() {
				Some(ogs) => match ogs.next() {
					Some(quad) => break Some(quad),
					None => self.ogs = None,
				},
				None => match self.p.next() {
					Some(p) => {
						self.ogs = Some(OgsOf {
							s: self.s,
							p,
							o: self.o.clone(),
							g: self.g.clone(),
							gs: None,
						})
					}
					None => break None,
				},
			}
		}
	}
}

struct OgsOf<'a, I: ?Sized + ReverseTermInterpretation> {
	s: Id<&'a I::Iri, &'a I::BlankId>,
	p: &'a I::Iri,
	o: TermsOf<'a, I>,
	g: Option<IdsOf<'a, I>>,
	gs: Option<GsOf<'a, I>>,
}

impl<'a, I: ?Sized + ReverseTermInterpretation> Iterator for OgsOf<'a, I> {
	type Item = Quad<
		Id<&'a I::Iri, &'a I::BlankId>,
		&'a I::Iri,
		Term<Id<&'a I::Iri, &'a I::BlankId>, &'a I::Literal>,
		Id<&'a I::Iri, &'a I::BlankId>,
	>;

	fn next(&mut self) -> Option<Self::Item> {
		loop {
			match self.gs.as_mut() {
				Some(gs) => match gs.next() {
					Some(quad) => break Some(quad),
					None => self.gs = None,
				},
				None => match self.o.next() {
					Some(o) => match self.g.clone() {
						Some(g) => {
							self.gs = Some(GsOf {
								s: self.s,
								p: self.p,
								o,
								g,
							})
						}
						None => break Some(Quad(self.s, self.p, o, None)),
					},
					None => break None,
				},
			}
		}
	}
}

struct GsOf<'a, I: ?Sized + ReverseTermInterpretation> {
	s: Id<&'a I::Iri, &'a I::BlankId>,
	p: &'a I::Iri,
	o: Term<Id<&'a I::Iri, &'a I::BlankId>, &'a I::Literal>,
	g: IdsOf<'a, I>,
}

impl<'a, I: ?Sized + ReverseTermInterpretation> Iterator for GsOf<'a, I> {
	type Item = Quad<
		Id<&'a I::Iri, &'a I::BlankId>,
		&'a I::Iri,
		Term<Id<&'a I::Iri, &'a I::BlankId>, &'a I::Literal>,
		Id<&'a I::Iri, &'a I::BlankId>,
	>;

	fn next(&mut self) -> Option<Self::Item> {
		self.g.next().map(|g| Quad(self.s, self.p, self.o, Some(g)))
	}
}

pub struct GrdfQuadsOf<'a, I: ?Sized + ReverseTermInterpretation> {
	s: TermsOf<'a, I>,
	p: TermsOf<'a, I>,
	o: TermsOf<'a, I>,
	g: Option<TermsOf<'a, I>>,
	pogs: Option<GrdfPogsOf<'a, I>>,
}

impl<'a, I: ?Sized + ReverseTermInterpretation> Iterator for GrdfQuadsOf<'a, I> {
	type Item = UninterpretedGrdfQuadRef<'a, I>;

	fn next(&mut self) -> Option<Self::Item> {
		loop {
			match self.pogs.as_mut() {
				Some(pogs) => match pogs.next() {
					Some(quad) => break Some(quad),
					None => self.pogs = None,
				},
				None => match self.s.next() {
					Some(s) => {
						self.pogs = Some(GrdfPogsOf {
							s,
							p: self.p.clone(),
							o: self.o.clone(),
							g: self.g.clone(),
							ogs: None,
						})
					}
					None => break None,
				},
			}
		}
	}
}

struct GrdfPogsOf<'a, I: ?Sized + ReverseTermInterpretation> {
	s: UninterpretedTermRef<'a, I>,
	p: TermsOf<'a, I>,
	o: TermsOf<'a, I>,
	g: Option<TermsOf<'a, I>>,
	ogs: Option<GrdfOgsOf<'a, I>>,
}

impl<'a, I: ?Sized + ReverseTermInterpretation> Iterator for GrdfPogsOf<'a, I> {
	type Item = UninterpretedGrdfQuadRef<'a, I>;

	fn next(&mut self) -> Option<Self::Item> {
		loop {
			match self.ogs.as_mut() {
				Some(ogs) => match ogs.next() {
					Some(quad) => break Some(quad),
					None => self.ogs = None,
				},
				None => match self.p.next() {
					Some(p) => {
						self.ogs = Some(GrdfOgsOf {
							s: self.s,
							p,
							o: self.o.clone(),
							g: self.g.clone(),
							gs: None,
						})
					}
					None => break None,
				},
			}
		}
	}
}

struct GrdfOgsOf<'a, I: ?Sized + ReverseTermInterpretation> {
	s: UninterpretedTermRef<'a, I>,
	p: UninterpretedTermRef<'a, I>,
	o: TermsOf<'a, I>,
	g: Option<TermsOf<'a, I>>,
	gs: Option<GrdfGsOf<'a, I>>,
}

impl<'a, I: ?Sized + ReverseTermInterpretation> Iterator for GrdfOgsOf<'a, I> {
	type Item = UninterpretedGrdfQuadRef<'a, I>;

	fn next(&mut self) -> Option<Self::Item> {
		loop {
			match self.gs.as_mut() {
				Some(gs) => match gs.next() {
					Some(quad) => break Some(quad),
					None => self.gs = None,
				},
				None => match self.o.next() {
					Some(o) => match self.g.clone() {
						Some(g) => {
							self.gs = Some(GrdfGsOf {
								s: self.s,
								p: self.p,
								o,
								g,
							})
						}
						None => break Some(Quad(self.s, self.p, o, None)),
					},
					None => break None,
				},
			}
		}
	}
}

struct GrdfGsOf<'a, I: ?Sized + ReverseTermInterpretation> {
	s: UninterpretedTermRef<'a, I>,
	p: UninterpretedTermRef<'a, I>,
	o: UninterpretedTermRef<'a, I>,
	g: TermsOf<'a, I>,
}

impl<'a, I: ?Sized + ReverseTermInterpretation> Iterator for GrdfGsOf<'a, I> {
	type Item = UninterpretedGrdfQuadRef<'a, I>;

	fn next(&mut self) -> Option<Self::Item> {
		self.g.next().map(|g| Quad(self.s, self.p, self.o, Some(g)))
	}
}

impl<I: ?Sized + ReverseIdInterpretation + ReverseLiteralInterpretation> ReverseTermInterpretation
	for I
{
}

pub trait ReverseLiteralInterpretationMut: ReverseLiteralInterpretation {
	/// Assigns the given literal to the given resource.
	fn assign_literal(&mut self, resource: Self::Resource, literal: Self::Literal) -> bool;
}

/// Mutable reverse term identifier interpretation.
///
/// Used to associate terms to resources.
pub trait ReverseTermInterpretationMut:
	ReverseIdInterpretationMut + ReverseLiteralInterpretationMut
{
	/// Assigns the given term to the given resource.
	fn assign_term(
		&mut self,
		resource: Self::Resource,
		term: Term<Id<Self::Iri, Self::BlankId>, Self::Literal>,
	) -> bool {
		match term {
			Term::Id(id) => self.assign_id(resource, id),
			Term::Literal(l) => self.assign_literal(resource, l),
		}
	}

	/// Assigns a term to all the interpreted resources.
	fn assign_terms(
		&mut self,
		mut f: impl FnMut(
			&Self,
			&Self::Resource,
		) -> Option<Term<Id<Self::Iri, Self::BlankId>, Self::Literal>>,
	) where
		Self: TraversableInterpretation,
	{
		let mut terms = Vec::new();
		for r in self.resources() {
			if let Some(term) = f(self, &r) {
				terms.push((r, term))
			}
		}

		for (r, term) in terms {
			self.assign_term(r, term);
		}
	}

	/// Generates and assign a node identifier for all the resources that don't
	/// have any term, using the given generator.
	fn generate_ids<N: Namespace<Id = Id<Self::Iri, Self::BlankId>>>(
		&mut self,
		namespace: &mut N,
		generator: &mut impl Generator<N>,
	) where
		Self: TraversableInterpretation + ReverseTermInterpretationMut,
	{
		self.assign_terms(|i, r| (!i.has_term(r)).then(|| Term::Id(generator.next(namespace))))
	}
}

impl<I: ReverseIdInterpretationMut + ReverseLiteralInterpretationMut> ReverseTermInterpretationMut
	for I
{
}

/// Iterator over the terms of an interpreted resource.
pub struct TermsOf<'a, I: 'a + ?Sized + ReverseTermInterpretation> {
	ids: IdsOf<'a, I>,
	literals: I::Literals<'a>,
}

impl<'a, I: 'a + ?Sized + ReverseTermInterpretation> Clone for TermsOf<'a, I> {
	fn clone(&self) -> Self {
		Self {
			ids: self.ids.clone(),
			literals: self.literals.clone(),
		}
	}
}

impl<'a, I: 'a + ?Sized + ReverseTermInterpretation> Copy for TermsOf<'a, I>
where
	I::Iris<'a>: Copy,
	I::BlankIds<'a>: Copy,
	I::Literals<'a>: Copy,
{
}

impl<'a, I: 'a + ?Sized + ReverseTermInterpretation> Iterator for TermsOf<'a, I> {
	type Item = UninterpretedTermRef<'a, I>;

	fn next(&mut self) -> Option<Self::Item> {
		self.ids
			.next()
			.map(Term::Id)
			.or_else(|| self.literals.next().map(Term::Literal))
	}
}

/// RDF interpretation function.
pub trait Interpret<I: Interpretation> {
	/// Interpreted form.
	type Interpreted;

	/// Interpret the given resource.
	fn interpret(self, interpretation: &mut I) -> Self::Interpreted;
}

impl<I: Interpretation, T: Interpret<I>> Interpret<I> for Option<T> {
	type Interpreted = Option<T::Interpreted>;

	fn interpret(self, interpretation: &mut I) -> Self::Interpreted {
		self.map(|t| t.interpret(interpretation))
	}
}

impl<I, B, T: IdInterpretationMut<I, B>> Interpret<T> for Id<I, B> {
	type Interpreted = T::Resource;

	fn interpret(self, interpretation: &mut T) -> Self::Interpreted {
		interpretation.interpret_id(self)
	}
}

impl<T, S, I: LiteralInterpretationMut<Self>> Interpret<I> for Literal<T, S> {
	type Interpreted = I::Resource;

	fn interpret(self, interpretation: &mut I) -> Self::Interpreted {
		interpretation.interpret_literal(self)
	}
}
