//! This is a utility library providing common types
//! when dealing with RDF data:
//! blank node identifier, literal, subject, predicate, object,
//! graph label, gRDF term, triple and quad.
//!
//! The optional feature `loc` provides compatibility
//! with the `locspan` crate to locate every sub-component
//! of a term.
use iref::{Iri, IriBuf};
use std::cmp::Ordering;
use std::fmt;

#[cfg(feature = "contextual")]
use contextual::{DisplayWithContext, WithContext};

/// Type definitions for RDF types with metadata.
#[cfg(feature = "meta")]
use locspan_derive::*;

mod blankid;
mod display;
pub mod generator;
mod literal;
mod term;
pub mod vocabulary;

#[cfg(feature = "meta")]
pub mod meta;

pub use blankid::*;
pub use display::*;
pub use generator::Generator;
pub use literal::*;
pub use term::*;
pub use vocabulary::{
	BlankIdVocabulary, BlankIdVocabularyMut, IndexVocabulary, IriVocabulary, IriVocabularyMut,
	NoVocabulary, Vocabulary, VocabularyMut,
};

#[cfg(feature = "meta")]
pub use generator::MetaGenerator;

/// RDF triple.
#[derive(Clone, Copy, Eq, Ord, Hash, Debug)]
#[cfg_attr(
	feature = "meta",
	derive(
		StrippedPartialEq,
		StrippedEq,
		StrippedPartialOrd,
		StrippedOrd,
		StrippedHash
	)
)]
pub struct Triple<S = Id, P = IriBuf, O = Object>(pub S, pub P, pub O);

impl<S1: PartialEq<S2>, P1: PartialEq<P2>, O1: PartialEq<O2>, S2, P2, O2>
	PartialEq<Triple<S2, P2, O2>> for Triple<S1, P1, O1>
{
	fn eq(&self, other: &Triple<S2, P2, O2>) -> bool {
		self.0 == other.0 && self.1 == other.1 && self.2 == other.2
	}
}

impl<S1: PartialOrd<S2>, P1: PartialOrd<P2>, O1: PartialOrd<O2>, S2, P2, O2>
	PartialOrd<Triple<S2, P2, O2>> for Triple<S1, P1, O1>
{
	fn partial_cmp(&self, other: &Triple<S2, P2, O2>) -> Option<Ordering> {
		match self.0.partial_cmp(&other.0) {
			Some(Ordering::Equal) => match self.1.partial_cmp(&other.1) {
				Some(Ordering::Equal) => self.2.partial_cmp(&other.2),
				cmp => cmp,
			},
			cmp => cmp,
		}
	}
}

impl Triple {
	pub fn into_grdf(self) -> GrdfTriple {
		Triple(self.0.into_term(), Term::Id(Id::Iri(self.1)), self.2)
	}

	pub fn as_grdf(&self) -> GrdfTripleRef {
		Triple(
			self.0.as_term_ref(),
			TermRef::Id(Id::Iri(self.1.as_iri())),
			self.2.as_term_ref(),
		)
	}
}

impl<S, P, O> Triple<S, P, O> {
	/// Creates a new triple.
	pub fn new(subject: S, predicate: P, object: O) -> Self {
		Self(subject, predicate, object)
	}

	/// Returns a reference to the subject of the triple,
	/// the first component.
	pub fn subject(&self) -> &S {
		&self.0
	}

	/// Returns a mutable reference to the subject of the triple,
	/// the first component.
	pub fn subject_mut(&mut self) -> &mut S {
		&mut self.0
	}

	/// Turns the triple into its subject,
	/// the first component.
	pub fn into_subject(self) -> S {
		self.0
	}

	/// Returns a reference to the predicate of the triple,
	/// the second component.
	pub fn predicate(&self) -> &P {
		&self.1
	}

	/// Returns a mutable reference to the predicate of the triple,
	/// the second component.
	pub fn predicate_mut(&mut self) -> &mut P {
		&mut self.1
	}

	/// Turns the triple into its predicate,
	/// the second component.
	pub fn into_predicate(self) -> P {
		self.1
	}

	/// Returns a reference to the object of the triple,
	/// the third component.
	pub fn object(&self) -> &O {
		&self.2
	}

	/// Returns a mutable reference to the object of the triple,
	/// the third component.
	pub fn object_mut(&mut self) -> &mut O {
		&mut self.2
	}

	/// Turns the triple into its object,
	/// the third component.
	pub fn into_object(self) -> O {
		self.2
	}

	/// Turns the triple into a tuple
	pub fn into_parts(self) -> (S, P, O) {
		(self.0, self.1, self.2)
	}

	/// Turns the triple into a quad with the given `graph` component.
	pub fn into_quad<G>(self, graph: Option<G>) -> Quad<S, P, O, G> {
		Quad(self.0, self.1, self.2, graph)
	}

	/// Maps the subject with the given function.
	pub fn map_subject<U>(self, f: impl FnOnce(S) -> U) -> Triple<U, P, O> {
		Triple(f(self.0), self.1, self.2)
	}

	/// Maps the subject with the given function.
	pub fn map_predicate<U>(self, f: impl FnOnce(P) -> U) -> Triple<S, U, O> {
		Triple(self.0, f(self.1), self.2)
	}

	/// Maps the subject with the given function.
	pub fn map_object<U>(self, f: impl FnOnce(O) -> U) -> Triple<S, P, U> {
		Triple(self.0, self.1, f(self.2))
	}
}

impl<S, L> Triple<Id, IriBuf, Object<IriBuf, BlankIdBuf, Literal<S, IriBuf, L>>> {
	#[allow(clippy::type_complexity)]
	pub fn inserted_into<V: VocabularyMut>(
		&self,
		vocabulary: &mut V,
	) -> Triple<Id<V::Iri, V::BlankId>, V::Iri, Object<V::Iri, V::BlankId, Literal<S, V::Iri, L>>>
	where
		S: Clone,
		L: Clone,
	{
		Triple(
			self.0.inserted_into(vocabulary),
			vocabulary.insert(self.1.as_iri()),
			self.2.inserted_into(vocabulary),
		)
	}

	#[allow(clippy::type_complexity)]
	pub fn insert_into<V: VocabularyMut>(
		self,
		vocabulary: &mut V,
	) -> Triple<Id<V::Iri, V::BlankId>, V::Iri, Object<V::Iri, V::BlankId, Literal<S, V::Iri, L>>> {
		Triple(
			self.0.insert_into(vocabulary),
			vocabulary.insert(self.1.as_iri()),
			self.2.insert_into(vocabulary),
		)
	}
}

impl<S, L>
	Quad<
		Term<IriBuf, BlankIdBuf, Literal<S, IriBuf, L>>,
		Term<IriBuf, BlankIdBuf, Literal<S, IriBuf, L>>,
		Term<IriBuf, BlankIdBuf, Literal<S, IriBuf, L>>,
	>
{
	#[allow(clippy::type_complexity)]
	pub fn inserted_into<V: VocabularyMut>(
		&self,
		vocabulary: &mut V,
	) -> Triple<
		Term<V::Iri, V::BlankId, Literal<S, V::Iri, L>>,
		Term<V::Iri, V::BlankId, Literal<S, V::Iri, L>>,
		Term<V::Iri, V::BlankId, Literal<S, V::Iri, L>>,
	>
	where
		S: Clone,
		L: Clone,
	{
		Triple(
			self.0.inserted_into(vocabulary),
			self.1.inserted_into(vocabulary),
			self.2.inserted_into(vocabulary),
		)
	}

	#[allow(clippy::type_complexity)]
	pub fn insert_into<V: VocabularyMut>(
		self,
		vocabulary: &mut V,
	) -> Triple<
		Term<V::Iri, V::BlankId, Literal<S, V::Iri, L>>,
		Term<V::Iri, V::BlankId, Literal<S, V::Iri, L>>,
		Term<V::Iri, V::BlankId, Literal<S, V::Iri, L>>,
	> {
		Triple(
			self.0.insert_into(vocabulary),
			self.1.insert_into(vocabulary),
			self.2.insert_into(vocabulary),
		)
	}
}

impl<S: RdfDisplay, P: RdfDisplay, O: RdfDisplay> fmt::Display for Triple<S, P, O> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(
			f,
			"{} {} {}",
			self.0.rdf_display(),
			self.1.rdf_display(),
			self.2.rdf_display()
		)
	}
}

impl<S: RdfDisplay, P: RdfDisplay, O: RdfDisplay> RdfDisplay for Triple<S, P, O> {
	fn rdf_fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(
			f,
			"{} {} {}",
			self.0.rdf_display(),
			self.1.rdf_display(),
			self.2.rdf_display()
		)
	}
}

#[cfg(feature = "contextual")]
impl<S: RdfDisplayWithContext<V>, P: RdfDisplayWithContext<V>, O: RdfDisplayWithContext<V>, V>
	DisplayWithContext<V> for Triple<S, P, O>
{
	fn fmt_with(&self, vocabulary: &V, f: &mut fmt::Formatter) -> fmt::Result {
		write!(
			f,
			"{} {} {}",
			self.0.with(vocabulary).rdf_display(),
			self.1.with(vocabulary).rdf_display(),
			self.2.with(vocabulary).rdf_display()
		)
	}
}

#[cfg(feature = "contextual")]
impl<S: RdfDisplayWithContext<V>, P: RdfDisplayWithContext<V>, O: RdfDisplayWithContext<V>, V>
	RdfDisplayWithContext<V> for Triple<S, P, O>
{
	fn rdf_fmt_with(&self, vocabulary: &V, f: &mut fmt::Formatter) -> fmt::Result {
		write!(
			f,
			"{} {} {}",
			self.0.with(vocabulary).rdf_display(),
			self.1.with(vocabulary).rdf_display(),
			self.2.with(vocabulary).rdf_display()
		)
	}
}

/// RDF triple reference.
pub type TripleRef<'a> = Triple<IdRef<'a>, Iri<'a>, ObjectRef<'a>>;

/// gRDF triple.
pub type GrdfTriple = Triple<Term, Term, Term>;

/// gRDF triple reference.
pub type GrdfTripleRef<'a> = Triple<TermRef<'a>, TermRef<'a>, TermRef<'a>>;

/// RDF quad.
#[derive(Clone, Copy, Eq, Ord, Hash, Debug)]
#[cfg_attr(
	feature = "meta",
	derive(
		StrippedPartialEq,
		StrippedEq,
		StrippedPartialOrd,
		StrippedOrd,
		StrippedHash
	)
)]
pub struct Quad<S = Id, P = IriBuf, O = Object, G = GraphLabel>(pub S, pub P, pub O, pub Option<G>);

impl Quad {
	pub fn into_grdf(self) -> GrdfQuad {
		Quad(
			self.0.into_term(),
			Term::Id(Id::Iri(self.1)),
			self.2,
			self.3.map(GraphLabel::into_term),
		)
	}

	pub fn as_grdf(&self) -> GrdfQuadRef {
		Quad(
			self.0.as_term_ref(),
			TermRef::Id(Id::Iri(self.1.as_iri())),
			self.2.as_term_ref(),
			self.3.as_ref().map(GraphLabel::as_term_ref),
		)
	}
}

impl<S, L> Quad<Id, IriBuf, Object<IriBuf, BlankIdBuf, Literal<S, IriBuf, L>>, GraphLabel> {
	#[allow(clippy::type_complexity)]
	pub fn inserted_into<V: VocabularyMut>(
		&self,
		vocabulary: &mut V,
	) -> Quad<
		Id<V::Iri, V::BlankId>,
		V::Iri,
		Object<V::Iri, V::BlankId, Literal<S, V::Iri, L>>,
		GraphLabel<V::Iri, V::BlankId>,
	>
	where
		S: Clone,
		L: Clone,
	{
		Quad(
			self.0.inserted_into(vocabulary),
			vocabulary.insert(self.1.as_iri()),
			self.2.inserted_into(vocabulary),
			self.3.as_ref().map(|g| g.inserted_into(vocabulary)),
		)
	}

	#[allow(clippy::type_complexity)]
	pub fn insert_into<V: VocabularyMut>(
		self,
		vocabulary: &mut V,
	) -> Quad<
		Id<V::Iri, V::BlankId>,
		V::Iri,
		Object<V::Iri, V::BlankId, Literal<S, V::Iri, L>>,
		GraphLabel<V::Iri, V::BlankId>,
	> {
		Quad(
			self.0.insert_into(vocabulary),
			vocabulary.insert(self.1.as_iri()),
			self.2.insert_into(vocabulary),
			self.3.map(|g| g.insert_into(vocabulary)),
		)
	}
}

impl<S, L>
	Quad<
		Term<IriBuf, BlankIdBuf, Literal<S, IriBuf, L>>,
		Term<IriBuf, BlankIdBuf, Literal<S, IriBuf, L>>,
		Term<IriBuf, BlankIdBuf, Literal<S, IriBuf, L>>,
		Term<IriBuf, BlankIdBuf, Literal<S, IriBuf, L>>,
	>
{
	#[allow(clippy::type_complexity)]
	pub fn inserted_into<V: VocabularyMut>(
		&self,
		vocabulary: &mut V,
	) -> Quad<
		Term<V::Iri, V::BlankId, Literal<S, V::Iri, L>>,
		Term<V::Iri, V::BlankId, Literal<S, V::Iri, L>>,
		Term<V::Iri, V::BlankId, Literal<S, V::Iri, L>>,
		Term<V::Iri, V::BlankId, Literal<S, V::Iri, L>>,
	>
	where
		S: Clone,
		L: Clone,
	{
		Quad(
			self.0.inserted_into(vocabulary),
			self.1.inserted_into(vocabulary),
			self.2.inserted_into(vocabulary),
			self.3.as_ref().map(|g| g.inserted_into(vocabulary)),
		)
	}

	#[allow(clippy::type_complexity)]
	pub fn insert_into<V: VocabularyMut>(
		self,
		vocabulary: &mut V,
	) -> Quad<
		Term<V::Iri, V::BlankId, Literal<S, V::Iri, L>>,
		Term<V::Iri, V::BlankId, Literal<S, V::Iri, L>>,
		Term<V::Iri, V::BlankId, Literal<S, V::Iri, L>>,
		Term<V::Iri, V::BlankId, Literal<S, V::Iri, L>>,
	> {
		Quad(
			self.0.insert_into(vocabulary),
			self.1.insert_into(vocabulary),
			self.2.insert_into(vocabulary),
			self.3.map(|g| g.insert_into(vocabulary)),
		)
	}
}

impl<S, P, O, G> Quad<S, P, O, G> {
	/// Creates a new quad.
	pub fn new(subject: S, predicate: P, object: O, graph: Option<G>) -> Self {
		Self(subject, predicate, object, graph)
	}

	/// Returns a reference to the subject of the quad,
	/// the first component.
	pub fn subject(&self) -> &S {
		&self.0
	}

	/// Returns a mutable reference to the subject of the quad,
	/// the first component.
	pub fn subject_mut(&mut self) -> &mut S {
		&mut self.0
	}

	/// Turns the quad into its subject,
	/// the first component.
	pub fn into_subject(self) -> S {
		self.0
	}

	/// Returns a reference to the predicate of the quad,
	/// the second component.
	pub fn predicate(&self) -> &P {
		&self.1
	}

	/// Returns a mutable reference to the predicate of the quad,
	/// the second component.
	pub fn predicate_mut(&mut self) -> &mut P {
		&mut self.1
	}

	/// Turns the quad into its predicate,
	/// the second component.
	pub fn into_predicate(self) -> P {
		self.1
	}

	/// Returns a reference to the object of the quad,
	/// the third component.
	pub fn object(&self) -> &O {
		&self.2
	}

	/// Returns a mutable reference to the object of the quad,
	/// the third component.
	pub fn object_mut(&mut self) -> &mut O {
		&mut self.2
	}

	/// Turns the quad into its object,
	/// the third component.
	pub fn into_object(self) -> O {
		self.2
	}

	/// Returns a reference to the graph of the quad,
	/// the fourth component.
	pub fn graph(&self) -> Option<&G> {
		self.3.as_ref()
	}

	/// Returns a mutable reference to the graph of the quad,
	/// the fourth component.
	pub fn graph_mut(&mut self) -> Option<&mut G> {
		self.3.as_mut()
	}

	/// Turns the quad into its graph,
	/// the fourth component.
	pub fn into_graph(self) -> Option<G> {
		self.3
	}

	pub fn into_parts(self) -> (S, P, O, Option<G>) {
		(self.0, self.1, self.2, self.3)
	}

	/// Turns this quad into a triple and its graph component.
	pub fn into_triple(self) -> (Triple<S, P, O>, Option<G>) {
		(Triple(self.0, self.1, self.2), self.3)
	}

	/// Maps the subject with the given function.
	pub fn map_subject<U>(self, f: impl FnOnce(S) -> U) -> Quad<U, P, O, G> {
		Quad(f(self.0), self.1, self.2, self.3)
	}

	/// Maps the subject with the given function.
	pub fn map_predicate<U>(self, f: impl FnOnce(P) -> U) -> Quad<S, U, O, G> {
		Quad(self.0, f(self.1), self.2, self.3)
	}

	/// Maps the subject with the given function.
	pub fn map_object<U>(self, f: impl FnOnce(O) -> U) -> Quad<S, P, U, G> {
		Quad(self.0, self.1, f(self.2), self.3)
	}

	/// Maps the graph with the given function.
	pub fn map_graph<U>(self, f: impl FnOnce(Option<G>) -> Option<U>) -> Quad<S, P, O, U> {
		Quad(self.0, self.1, self.2, f(self.3))
	}
}

impl<
		S1: PartialEq<S2>,
		P1: PartialEq<P2>,
		O1: PartialEq<O2>,
		G1: PartialEq<G2>,
		S2,
		P2,
		O2,
		G2,
	> PartialEq<Quad<S2, P2, O2, G2>> for Quad<S1, P1, O1, G1>
{
	fn eq(&self, other: &Quad<S2, P2, O2, G2>) -> bool {
		self.0 == other.0
			&& self.1 == other.1
			&& self.2 == other.2
			&& match (&self.3, &other.3) {
				(Some(a), Some(b)) => a == b,
				(None, None) => true,
				_ => false,
			}
	}
}

impl<
		S1: PartialOrd<S2>,
		P1: PartialOrd<P2>,
		O1: PartialOrd<O2>,
		G1: PartialOrd<G2>,
		S2,
		P2,
		O2,
		G2,
	> PartialOrd<Quad<S2, P2, O2, G2>> for Quad<S1, P1, O1, G1>
{
	fn partial_cmp(&self, other: &Quad<S2, P2, O2, G2>) -> Option<Ordering> {
		match self.0.partial_cmp(&other.0) {
			Some(Ordering::Equal) => match self.1.partial_cmp(&other.1) {
				Some(Ordering::Equal) => match self.2.partial_cmp(&other.2) {
					Some(Ordering::Equal) => match (&self.3, &other.3) {
						(Some(a), Some(b)) => a.partial_cmp(b),
						(Some(_), None) => Some(Ordering::Greater),
						(None, Some(_)) => Some(Ordering::Less),
						(None, None) => Some(Ordering::Equal),
					},
					cmp => cmp,
				},
				cmp => cmp,
			},
			cmp => cmp,
		}
	}
}

impl<S: RdfDisplay, P: RdfDisplay, O: RdfDisplay, G: RdfDisplay> fmt::Display for Quad<S, P, O, G> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self.graph() {
			Some(graph) => write!(
				f,
				"{} {} {} {}",
				self.0.rdf_display(),
				self.1.rdf_display(),
				self.2.rdf_display(),
				graph.rdf_display()
			),
			None => write!(
				f,
				"{} {} {}",
				self.0.rdf_display(),
				self.1.rdf_display(),
				self.2.rdf_display()
			),
		}
	}
}

impl<S: RdfDisplay, P: RdfDisplay, O: RdfDisplay, G: RdfDisplay> RdfDisplay for Quad<S, P, O, G> {
	fn rdf_fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self.graph() {
			Some(graph) => write!(
				f,
				"{} {} {} {}",
				self.0.rdf_display(),
				self.1.rdf_display(),
				self.2.rdf_display(),
				graph.rdf_display()
			),
			None => write!(
				f,
				"{} {} {}",
				self.0.rdf_display(),
				self.1.rdf_display(),
				self.2.rdf_display()
			),
		}
	}
}

#[cfg(feature = "contextual")]
impl<
		S: RdfDisplayWithContext<V>,
		P: RdfDisplayWithContext<V>,
		O: RdfDisplayWithContext<V>,
		G: RdfDisplayWithContext<V>,
		V,
	> DisplayWithContext<V> for Quad<S, P, O, G>
{
	fn fmt_with(&self, vocabulary: &V, f: &mut fmt::Formatter) -> fmt::Result {
		match self.graph() {
			Some(graph) => write!(
				f,
				"{} {} {} {}",
				self.0.with(vocabulary).rdf_display(),
				self.1.with(vocabulary).rdf_display(),
				self.2.with(vocabulary).rdf_display(),
				graph.with(vocabulary).rdf_display()
			),
			None => write!(
				f,
				"{} {} {}",
				self.0.with(vocabulary).rdf_display(),
				self.1.with(vocabulary).rdf_display(),
				self.2.with(vocabulary).rdf_display()
			),
		}
	}
}

#[cfg(feature = "contextual")]
impl<
		S: RdfDisplayWithContext<V>,
		P: RdfDisplayWithContext<V>,
		O: RdfDisplayWithContext<V>,
		G: RdfDisplayWithContext<V>,
		V,
	> RdfDisplayWithContext<V> for Quad<S, P, O, G>
{
	fn rdf_fmt_with(&self, vocabulary: &V, f: &mut fmt::Formatter) -> fmt::Result {
		match self.graph() {
			Some(graph) => write!(
				f,
				"{} {} {} {}",
				self.0.with(vocabulary).rdf_display(),
				self.1.with(vocabulary).rdf_display(),
				self.2.with(vocabulary).rdf_display(),
				graph.with(vocabulary).rdf_display()
			),
			None => write!(
				f,
				"{} {} {}",
				self.0.with(vocabulary).rdf_display(),
				self.1.with(vocabulary).rdf_display(),
				self.2.with(vocabulary).rdf_display()
			),
		}
	}
}

impl Quad {
	#[inline(always)]
	pub fn as_quad_ref(&self) -> QuadRef {
		Quad(
			self.0.as_subject_ref(),
			self.1.as_iri(),
			self.2.as_object_ref(),
			self.3.as_ref().map(GraphLabel::as_graph_label_ref),
		)
	}
}

impl<'a> From<QuadRef<'a>> for Quad {
	#[inline(always)]
	fn from(q: QuadRef<'a>) -> Self {
		q.into_owned()
	}
}

impl GrdfQuad {
	#[inline(always)]
	pub fn as_grdf_quad_ref(&self) -> GrdfQuadRef {
		Quad(
			self.0.as_term_ref(),
			self.1.as_term_ref(),
			self.2.as_term_ref(),
			self.3.as_ref().map(Term::as_term_ref),
		)
	}
}

impl<'a> From<GrdfQuadRef<'a>> for GrdfQuad {
	#[inline(always)]
	fn from(q: GrdfQuadRef<'a>) -> Self {
		q.into_owned()
	}
}

/// RDF quad reference.
pub type QuadRef<'a> = Quad<IdRef<'a>, Iri<'a>, ObjectRef<'a>, GraphLabelRef<'a>>;

impl<'a> QuadRef<'a> {
	#[inline(always)]
	pub fn into_owned(self) -> Quad {
		Quad(
			self.0.into_owned(),
			self.1.to_owned(),
			self.2.into_owned(),
			self.3.map(GraphLabelRef::into_owned),
		)
	}
}

impl<'a> From<&'a Quad> for QuadRef<'a> {
	#[inline(always)]
	fn from(q: &'a Quad) -> Self {
		q.as_quad_ref()
	}
}

impl<'a> From<Quad<&'a Id, &'a IriBuf, &'a Object, &'a GraphLabel>> for QuadRef<'a> {
	#[inline(always)]
	fn from(Quad(s, p, o, g): Quad<&'a Id, &'a IriBuf, &'a Object, &'a GraphLabel>) -> Self {
		Quad(
			s.as_subject_ref(),
			p.as_iri(),
			o.as_object_ref(),
			g.map(GraphLabel::as_graph_label_ref),
		)
	}
}

/// gRDF quad.
pub type GrdfQuad = Quad<Term, Term, Term, Term>;

/// gRDF quad reference.
pub type GrdfQuadRef<'a> = Quad<TermRef<'a>, TermRef<'a>, TermRef<'a>, TermRef<'a>>;

impl<'a> GrdfQuadRef<'a> {
	#[inline(always)]
	pub fn into_owned(self) -> GrdfQuad {
		Quad(
			self.0.into_owned(),
			self.1.into_owned(),
			self.2.into_owned(),
			self.3.map(Term::into_owned),
		)
	}
}

impl<'a> From<&'a GrdfQuad> for GrdfQuadRef<'a> {
	#[inline(always)]
	fn from(q: &'a GrdfQuad) -> Self {
		q.as_grdf_quad_ref()
	}
}

impl<'a> From<Quad<&'a Term, &'a Term, &'a Term, &'a Term>> for GrdfQuadRef<'a> {
	#[inline(always)]
	fn from(Quad(s, p, o, g): Quad<&'a Term, &'a Term, &'a Term, &'a Term>) -> Self {
		Quad(
			s.as_term_ref(),
			p.as_term_ref(),
			o.as_term_ref(),
			g.map(Term::as_term_ref),
		)
	}
}
