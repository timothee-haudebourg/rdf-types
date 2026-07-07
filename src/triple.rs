//! RDF triples.
use into_owned_trait::IntoOwned;

use crate::Quad;
use std::{cmp::Ordering, fmt};

/// RDF triple.
///
/// A triple is a (subject, predicate, object) tuple of resources. See
/// [`Quad`] for a triple additionally located in a named graph.
#[derive(Clone, Copy, Eq, Ord, Hash, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Triple<S, P = S, O = S>(pub S, pub P, pub O);

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

	/// Turns the triple into a quad with the given `graph` component.
	pub fn into_quad<G>(self, graph: Option<G>) -> Quad<S, P, O, G> {
		Quad(self.0, self.1, self.2, graph)
	}

	/// Maps the subject with the given function.
	pub fn map_subject<U>(self, f: impl FnOnce(S) -> U) -> Triple<U, P, O> {
		Triple(f(self.0), self.1, self.2)
	}

	/// Maps the predicate with the given function.
	pub fn map_predicate<U>(self, f: impl FnOnce(P) -> U) -> Triple<S, U, O> {
		Triple(self.0, f(self.1), self.2)
	}

	/// Maps the object with the given function.
	pub fn map_object<U>(self, f: impl FnOnce(O) -> U) -> Triple<S, P, U> {
		Triple(self.0, self.1, f(self.2))
	}

	/// Maps every triple component with the given functions, one for each
	/// component.
	pub fn map_all<S2, P2, O2>(
		self,
		s: impl FnOnce(S) -> S2,
		p: impl FnOnce(P) -> P2,
		o: impl FnOnce(O) -> O2,
	) -> Triple<S2, P2, O2> {
		Triple(s(self.0), p(self.1), o(self.2))
	}

	/// Borrows each component of the triple.
	pub fn as_ref(&self) -> Triple<&S, &P, &O> {
		Triple(&self.0, &self.1, &self.2)
	}
}

impl<S, P, O> Triple<&S, &P, &O> {
	/// Clones each borrowed component of the triple.
	pub fn cloned(&self) -> Triple<S, P, O>
	where
		S: Clone,
		P: Clone,
		O: Clone,
	{
		Triple(self.0.clone(), self.1.clone(), self.2.clone())
	}

	/// Copies each borrowed component of the triple.
	pub fn copied(&self) -> Triple<S, P, O>
	where
		S: Copy,
		P: Copy,
		O: Copy,
	{
		Triple(*self.0, *self.1, *self.2)
	}
}

impl<T> Triple<T, T, T> {
	/// Maps the components with the given function.
	pub fn map<U>(self, mut f: impl FnMut(T) -> U) -> Triple<U, U, U> {
		Triple(f(self.0), f(self.1), f(self.2))
	}
}

impl<S, P, O> IntoOwned for Triple<S, P, O>
where
	S: IntoOwned,
	P: IntoOwned,
	O: IntoOwned,
{
	type Owned = Triple<S::Owned, P::Owned, O::Owned>;

	fn into_owned(self) -> Self::Owned {
		Triple(
			self.0.into_owned(),
			self.1.into_owned(),
			self.2.into_owned(),
		)
	}
}

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

impl<S: fmt::Display, P: fmt::Display, O: fmt::Display> fmt::Display for Triple<S, P, O> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "({}, {}, {})", self.0, self.1, self.2)
	}
}
