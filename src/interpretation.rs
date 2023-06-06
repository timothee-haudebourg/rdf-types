use iref::{Iri, IriBuf};
use langtag::LanguageTagBuf;

use crate::{
	literal, BlankId, BlankIdBuf, BlankIdVocabulary, BlankIdVocabularyMut, Generator, Id,
	IriVocabulary, IriVocabularyMut, LanguageTagVocabularyMut, Literal, LiteralVocabulary,
	LiteralVocabularyMut, Namespace, Term,
};

mod indexed;

pub use indexed::*;

/// RDF resource interpretation.
pub trait Interpretation {
	/// Resource identifier type.
	type Resource;

	/// Interpreted resource iterator.
	type Resources<'a>: 'a + Iterator<Item = Self::Resource>
	where
		Self: 'a;

	/// Returns an iterator over the interpreted resources.
	fn resources(&self) -> Self::Resources<'_>;
}

/// IRI Interpretation.
pub trait IriInterpretation<I>: Interpretation {
	/// Returns the interpretation of the given IRI, if any.
	fn iri_interpretation(&self, iri: &I) -> Option<Self::Resource>;

	fn lexical_iri_interpretation(
		&self,
		vocabulary: &impl IriVocabulary<Iri = I>,
		iri: Iri,
	) -> Option<Self::Resource> {
		vocabulary
			.get(iri)
			.and_then(|iri| self.iri_interpretation(&iri))
	}
}

/// Blank node identifier interpretation.
pub trait BlankIdInterpretation<B>: Interpretation {
	/// Returns the interpretation of the given blank node identifier, if any.
	fn blank_id_interpretation(&self, blank_id: &B) -> Option<Self::Resource>;

	fn lexical_blank_id_interpretation(
		&self,
		vocabulary: &impl BlankIdVocabulary<BlankId = B>,
		blank_id: &BlankId,
	) -> Option<Self::Resource> {
		vocabulary
			.get_blank_id(blank_id)
			.and_then(|blank_id| self.blank_id_interpretation(&blank_id))
	}
}

/// Node identifier interpretation.
pub trait IdInterpretation<I>: Interpretation {
	/// Returns the interpretation of the given node identifier, if any.
	fn id_interpretation(&self, id: &I) -> Option<Self::Resource>;
}

impl<I, B, T: IriInterpretation<I> + BlankIdInterpretation<B>> IdInterpretation<Id<I, B>> for T {
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
		id: Id<Iri, &BlankId>,
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
		id: Id<Iri, &BlankId>,
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
pub trait TermInterpretation<I = Id<IriBuf, BlankIdBuf>, L = Literal>:
	IdInterpretation<I> + LiteralInterpretation<L>
{
	/// Returns the interpretation of the given term, if any.
	fn term_interpretation(&self, term: &Term<I, L>) -> Option<Self::Resource> {
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
		term: Term<Id<Iri, &BlankId>, &Literal<V::Type, V::Value>>,
	) -> Option<Self::Resource>;
}

impl<V: LiteralVocabulary, I: LexicalIdInterpretation<V> + LiteralInterpretation<V::Literal>>
	LexicalTermInterpretation<V> for I
{
	fn lexical_term_interpretation(
		&self,
		vocabulary: &V,
		term: Term<Id<Iri, &BlankId>, &Literal<V::Type, V::Value>>,
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
		iri: Iri,
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
pub trait IdInterpretationMut<I>: Interpretation {
	/// Interprets the given identifier.
	fn interpret_id(&mut self, id: I) -> Self::Resource;
}

impl<I, B, T: IriInterpretationMut<I> + BlankIdInterpretationMut<B>> IdInterpretationMut<Id<I, B>>
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
	fn interpret_lexical_id(&mut self, vocabulary: &mut V, id: Id<Iri, &BlankId>)
		-> Self::Resource;

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
		id: Id<Iri, &BlankId>,
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

pub trait TermInterpretationMut<I = Id<IriBuf, BlankIdBuf>, L = Literal>:
	IdInterpretationMut<I> + LiteralInterpretationMut<L>
{
	fn interpret_term(&mut self, term: Term<I, L>) -> Self::Resource {
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
		term: Term<Id<Iri, &BlankId>, &Literal<V::Type, V::Value>>,
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
			Id<Iri, &BlankId>,
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
pub trait ReverseIdInterpretation: Interpretation {
	type Iri;
	type BlankId;

	type Iris<'a>: Iterator<Item = &'a Self::Iri>
	where
		Self: 'a,
		Self::Iri: 'a;
	type BlankIds<'a>: Iterator<Item = &'a Self::BlankId>
	where
		Self: 'a,
		Self::BlankId: 'a;

	/// Returns an iterator over the IRIs of the given resource.
	fn iris_of(&self, id: &Self::Resource) -> Self::Iris<'_>;

	/// Returns an iterator over the blank node identifiers of the given resource.
	fn blank_ids_of(&self, id: &Self::Resource) -> Self::BlankIds<'_>;

	fn ids_of(&self, id: &Self::Resource) -> IdsOf<Self> {
		IdsOf {
			iris: self.iris_of(id),
			blanks: self.blank_ids_of(id),
		}
	}
}

pub struct IdsOf<'a, I: 'a + ?Sized + ReverseIdInterpretation> {
	iris: I::Iris<'a>,
	blanks: I::BlankIds<'a>,
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

pub trait ReverseTermInterpretation: Interpretation {
	type IdRef<'a>
	where
		Self: 'a;
	type LiteralRef<'a>
	where
		Self: 'a;

	type Ids<'a>: Iterator<Item = Self::IdRef<'a>>
	where
		Self: 'a;
	type Literals<'a>: Iterator<Item = Self::LiteralRef<'a>>
	where
		Self: 'a;

	fn ids_of(&self, id: &Self::Resource) -> Self::Ids<'_>;

	fn literals_of(&self, id: &Self::Resource) -> Self::Literals<'_>;

	fn terms_of(&self, id: &Self::Resource) -> TermsOf<Self> {
		TermsOf {
			ids: self.ids_of(id),
			literals: self.literals_of(id),
		}
	}

	fn term_of(&self, id: &Self::Resource) -> Option<Term<Self::IdRef<'_>, Self::LiteralRef<'_>>> {
		self.terms_of(id).next()
	}

	fn has_term(&self, id: &Self::Resource) -> bool {
		self.term_of(id).is_some()
	}
}

/// Mutable reverse term identifier interpretation.
///
/// Used to associate terms to resources.
pub trait ReverseTermInterpretationMut: ReverseTermInterpretation {
	type Id;
	type Literal;

	/// Assigns the given id to the given resource.
	fn assign_id(&mut self, resource: Self::Resource, id: Self::Id) -> bool;

	/// Assigns the given literal to the given resource.
	fn assign_literal(&mut self, resource: Self::Resource, literal: Self::Literal) -> bool;

	/// Assigns the given term to the given resource.
	fn assign_term(
		&mut self,
		resource: Self::Resource,
		term: Term<Self::Id, Self::Literal>,
	) -> bool {
		match term {
			Term::Id(id) => self.assign_id(resource, id),
			Term::Literal(l) => self.assign_literal(resource, l),
		}
	}

	/// Assigns a term to all the interpreted resources.
	fn assign_terms(
		&mut self,
		mut f: impl FnMut(&Self, &Self::Resource) -> Option<Term<Self::Id, Self::Literal>>,
	) {
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
	fn generate_ids<N: Namespace<Id = Self::Id>>(
		&mut self,
		namespace: &mut N,
		generator: &mut impl Generator<N>,
	) where
		Self: ReverseTermInterpretationMut,
	{
		self.assign_terms(|i, r| (!i.has_term(r)).then(|| Term::Id(generator.next(namespace))))
	}
}

/// Iterator over the terms of an interpreted resource.
pub struct TermsOf<'a, I: 'a + ?Sized + ReverseTermInterpretation> {
	ids: I::Ids<'a>,
	literals: I::Literals<'a>,
}

impl<'a, I: 'a + ?Sized + ReverseTermInterpretation> Iterator for TermsOf<'a, I> {
	type Item = Term<I::IdRef<'a>, I::LiteralRef<'a>>;

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

impl<I, B, T: IdInterpretationMut<Self>> Interpret<T> for Id<I, B> {
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
