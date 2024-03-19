use iref::Iri;

use crate::{
	vocabulary::{BlankIdVocabulary, IriVocabulary},
	BlankId, Generator, Id, Literal, LiteralRef, Quad, Term, Vocabulary, VocabularyMut,
};

use super::{
	IdInterpretation, IdInterpretationMut, IdsOf, LiteralInterpretation, LiteralInterpretationMut,
	ReverseBlankIdInterpretation, ReverseIdInterpretation, ReverseIdInterpretationMut,
	ReverseIriInterpretation, ReverseLiteralInterpretation, TraversableInterpretation,
	UninterpretedGrdfQuadRef, UninterpretedTermRef,
};

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

	fn lexical_term_interpretation(
		&self,
		vocabulary: &impl Vocabulary<Iri = I, BlankId = B, Literal = L>,
		term: Term<Id<&Iri, &BlankId>, LiteralRef<I>>,
	) -> Option<Self::Resource> {
		match term {
			Term::Id(id) => self.lexical_id_interpretation(vocabulary, id),
			Term::Literal(l) => self.lexical_literal_interpretation(vocabulary, l),
		}
	}
}

impl<I, B, L, T: IdInterpretation<I, B> + LiteralInterpretation<L>> TermInterpretation<I, B, L>
	for T
{
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

	fn interpret_lexical_term(
		&mut self,
		vocabulary: &mut impl VocabularyMut<Iri = I, BlankId = B, Literal = L>,
		term: Term<Id<&Iri, &BlankId>, LiteralRef<I>>,
	) -> Self::Resource {
		match term {
			Term::Id(id) => self.interpret_lexical_id(vocabulary, id),
			Term::Literal(l) => self.interpret_lexical_literal(vocabulary, l),
		}
	}

	fn interpret_owned_lexical_term(
		&mut self,
		vocabulary: &mut impl VocabularyMut<Iri = I, BlankId = B, Literal = L>,
		term: Term<Id, Literal<I>>,
	) -> Self::Resource {
		match term {
			Term::Id(id) => self.interpret_owned_lexical_id(vocabulary, id),
			Term::Literal(l) => self.interpret_owned_lexical_literal(vocabulary, l),
		}
	}

	fn interpret_full_lexical_term(
		&mut self,
		vocabulary: &mut impl VocabularyMut<Iri = I, BlankId = B, Literal = L>,
		term: Term,
	) -> Self::Resource {
		match term {
			Term::Id(id) => self.interpret_owned_lexical_id(vocabulary, id),
			Term::Literal(l) => self.interpret_full_lexical_literal(vocabulary, l),
		}
	}
}

impl<I, B, L, T: IdInterpretationMut<I, B> + LiteralInterpretationMut<L>>
	TermInterpretationMut<I, B, L> for T
{
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
	fn assign_literal(&mut self, resource: &Self::Resource, literal: Self::Literal) -> bool;
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
		resource: &Self::Resource,
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
		Self::Resource: Clone,
		Self: TraversableInterpretation,
	{
		let mut terms = Vec::new();
		for r in self.resources() {
			if let Some(term) = f(self, r) {
				terms.push((r.clone(), term))
			}
		}

		for (r, term) in terms {
			self.assign_term(&r, term);
		}
	}

	/// Generates and assign a node identifier for all the resources that don't
	/// have any term, using the given generator.
	fn generate_ids<V: IriVocabulary + BlankIdVocabulary>(
		&mut self,
		vocabulary: &mut V,
		generator: &mut impl Generator<V>,
	) where
		Self::Resource: Clone,
		Self: TraversableInterpretation
			+ ReverseTermInterpretationMut<Iri = V::Iri, BlankId = V::BlankId>,
	{
		self.assign_terms(|i, r| (!i.has_term(r)).then(|| Term::Id(generator.next(vocabulary))))
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
