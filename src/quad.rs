use std::{cmp::Ordering, fmt};

use iref::{Iri, IriBuf};

use crate::{
	GraphLabel, GraphLabelRef, Id, InsertIntoVocabulary, InsertedIntoVocabulary, Literal, Object,
	ObjectRef, RdfDisplay, SubjectRef, Triple, TryExportFromVocabulary,
};

#[cfg(feature = "contextual")]
use contextual::{DisplayWithContext, WithContext};

#[cfg(feature = "contextual")]
use crate::RdfDisplayWithContext;

/// Type definitions for RDF types with metadata.
#[cfg(feature = "meta")]
use locspan_derive::*;

/// RDF quad.
#[derive(Clone, Copy, Eq, Ord, Hash, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
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

/// Standard RDF quad reference.
pub type QuadRef<'a, L = Literal> =
	Quad<SubjectRef<'a>, Iri<'a>, ObjectRef<'a, L>, GraphLabelRef<'a>>;

impl<L> Quad<Id, IriBuf, Object<Id, L>, GraphLabel> {
	pub fn as_quad_ref(&self) -> QuadRef<L> {
		Quad(
			self.0.as_subject_ref(),
			self.1.as_iri(),
			self.2.as_object_ref(),
			self.3.as_ref().map(GraphLabel::as_graph_label_ref),
		)
	}
}

impl<'a, L> QuadRef<'a, L> {
	pub fn into_owned(self) -> Quad<Id, IriBuf, Object<Id, L>, GraphLabel>
	where
		L: Clone,
	{
		Quad(
			self.0.into_owned(),
			self.1.to_owned(),
			self.2.into_owned(),
			self.3.map(GraphLabelRef::into_owned),
		)
	}
}

impl<
		V,
		S: InsertIntoVocabulary<V>,
		P: InsertIntoVocabulary<V>,
		O: InsertIntoVocabulary<V>,
		G: InsertIntoVocabulary<V>,
	> InsertIntoVocabulary<V> for Quad<S, P, O, G>
{
	type Inserted = Quad<S::Inserted, P::Inserted, O::Inserted, G::Inserted>;

	fn insert_into_vocabulary(self, vocabulary: &mut V) -> Self::Inserted {
		Quad(
			self.0.insert_into_vocabulary(vocabulary),
			self.1.insert_into_vocabulary(vocabulary),
			self.2.insert_into_vocabulary(vocabulary),
			self.3.insert_into_vocabulary(vocabulary),
		)
	}
}

impl<
		V,
		S: InsertedIntoVocabulary<V>,
		P: InsertedIntoVocabulary<V>,
		O: InsertedIntoVocabulary<V>,
		G: InsertedIntoVocabulary<V>,
	> InsertedIntoVocabulary<V> for Quad<S, P, O, G>
{
	type Inserted = Quad<S::Inserted, P::Inserted, O::Inserted, G::Inserted>;

	fn inserted_into_vocabulary(&self, vocabulary: &mut V) -> Self::Inserted {
		Quad(
			self.0.inserted_into_vocabulary(vocabulary),
			self.1.inserted_into_vocabulary(vocabulary),
			self.2.inserted_into_vocabulary(vocabulary),
			self.3.inserted_into_vocabulary(vocabulary),
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

/// Type that can turn a `Quad<S, P, O, G>` into a `Quad`.
pub trait TryExportQuad<S, P, O, G> {
	type Error;

	fn try_export_quad(&self, quad: Quad<S, P, O, G>) -> Result<Quad, Self::Error>;
}

#[derive(Debug, thiserror::Error)]
pub enum QuadExportFailed<S, P, O, G> {
	#[error("invalid subject: {0}")]
	Subject(S),

	#[error("invalid predicate: {0}")]
	Predicate(P),

	#[error("invalid object: {0}")]
	Object(O),

	#[error("invalid graph label: {0}")]
	Graph(G),
}

impl<
		V,
		S: TryExportFromVocabulary<V>,
		P: TryExportFromVocabulary<V>,
		O: TryExportFromVocabulary<V>,
		G: TryExportFromVocabulary<V>,
	> TryExportFromVocabulary<V> for Quad<S, P, O, G>
{
	type Output = Quad<S::Output, P::Output, O::Output, G::Output>;
	type Error = QuadExportFailed<S::Error, P::Error, O::Error, G::Error>;

	fn try_export_from_vocabulary(self, vocabulary: &V) -> Result<Self::Output, Self::Error> {
		Ok(Quad(
			self.0
				.try_export_from_vocabulary(vocabulary)
				.map_err(QuadExportFailed::Subject)?,
			self.1
				.try_export_from_vocabulary(vocabulary)
				.map_err(QuadExportFailed::Predicate)?,
			self.2
				.try_export_from_vocabulary(vocabulary)
				.map_err(QuadExportFailed::Object)?,
			self.3
				.try_export_from_vocabulary(vocabulary)
				.map_err(QuadExportFailed::Graph)?,
		))
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
