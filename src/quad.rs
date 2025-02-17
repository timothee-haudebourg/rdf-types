use std::{cmp::Ordering, fmt};

use iref::{Iri, IriBuf};

use crate::{Id, IdRef, LocalTerm, LocalTermRef, RdfDisplay, Triple};

/// Lexical RDF quad.
pub type RdfQuad = Quad<Id, IriBuf, LocalTerm, Id>;

/// Lexical RDF quad reference.
pub type RdfQuadRef<'a> = Quad<IdRef<'a>, &'a Iri, LocalTermRef<'a>, IdRef<'a>>;

/// RDF quad.
#[derive(Clone, Copy, Eq, Ord, Hash, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Quad<S = LocalTerm, P = S, O = S, G = S>(pub S, pub P, pub O, pub Option<G>);

impl<S, P, O, G> Quad<S, P, O, G> {
	#[deprecated(since = "0.18.4", note = "please use `as_ref` instead")]
	pub fn borrow_components(&self) -> Quad<&S, &P, &O, &G> {
		self.as_ref()
	}

	/// Borrows each component of the quad.
	pub fn as_ref(&self) -> Quad<&S, &P, &O, &G> {
		Quad(&self.0, &self.1, &self.2, self.3.as_ref())
	}
}

impl<S, P, O, G> Quad<&S, &P, &O, &G> {
	pub fn cloned(&self) -> Quad<S, P, O, G>
	where
		S: Clone,
		P: Clone,
		O: Clone,
		G: Clone,
	{
		Quad(
			self.0.clone(),
			self.1.clone(),
			self.2.clone(),
			self.3.cloned(),
		)
	}

	pub fn into_cloned(self) -> Quad<S, P, O, G>
	where
		S: Clone,
		P: Clone,
		O: Clone,
		G: Clone,
	{
		Quad(
			self.0.clone(),
			self.1.clone(),
			self.2.clone(),
			self.3.cloned(),
		)
	}
}

impl<S, P, O, G> Quad<&S, &P, &O, &G> {
	pub fn copied(&self) -> Quad<S, P, O, G>
	where
		S: Copy,
		P: Copy,
		O: Copy,
		G: Copy,
	{
		Quad(*self.0, *self.1, *self.2, self.3.copied())
	}

	pub fn into_copied(self) -> Quad<S, P, O, G>
	where
		S: Copy,
		P: Copy,
		O: Copy,
		G: Copy,
	{
		Quad(*self.0, *self.1, *self.2, self.3.copied())
	}
}

impl RdfQuad {
	pub fn as_lexical_quad_ref(&self) -> RdfQuadRef {
		Quad(
			self.0.as_ref(),
			self.1.as_iri(),
			self.2.as_ref(),
			self.3.as_ref().map(Id::as_ref),
		)
	}
}

impl RdfQuadRef<'_> {
	pub fn into_owned(self) -> RdfQuad {
		Quad(
			self.0.to_owned(),
			self.1.to_owned(),
			self.2.to_owned(),
			self.3.map(IdRef::into_owned),
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

	pub fn with_graph(self, g: Option<G>) -> Self {
		Self(self.0, self.1, self.2, g)
	}

	/// Maps every quad component with the given functions, one for each
	/// component.
	pub fn map_all<S2, P2, O2, G2>(
		self,
		s: impl FnOnce(S) -> S2,
		p: impl FnOnce(P) -> P2,
		o: impl FnOnce(O) -> O2,
		g: impl FnOnce(Option<G>) -> Option<G2>,
	) -> Quad<S2, P2, O2, G2> {
		Quad(s(self.0), p(self.1), o(self.2), g(self.3))
	}
}

impl<T> Quad<T, T, T, T> {
	/// Maps the components with the given function.
	pub fn map<U>(self, mut f: impl FnMut(T) -> U) -> Quad<U, U, U, U> {
		Quad(f(self.0), f(self.1), f(self.2), self.3.map(f))
	}
}

/// Type that can turn a `Quad<S, P, O, G>` into a `Quad`.
pub trait TryExportQuad<S, P, O, G> {
	type Error;

	fn try_export_quad(&self, quad: Quad<S, P, O, G>) -> Result<RdfQuad, Self::Error>;
}

/// Error returned when calling [`try_extract_from_vocabulary`][1] on a
/// [`Quad`].
///
/// [1]: TryExtractFromVocabulary::try_extract_from_vocabulary
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
