use std::{cmp::Ordering, fmt};

use iref::{Iri, IriBuf};

use crate::{
	Id, InsertIntoVocabulary, InsertedIntoVocabulary, Literal, Object, ObjectRef, Quad, RdfDisplay,
	SubjectRef,
};

#[cfg(feature = "contextual")]
use contextual::{DisplayWithContext, WithContext};

#[cfg(feature = "contextual")]
use crate::RdfDisplayWithContext;

/// Type definitions for RDF types with metadata.
#[cfg(feature = "meta")]
use locspan_derive::*;

/// RDF triple.
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
pub struct Triple<S = Id, P = IriBuf, O = Object>(pub S, pub P, pub O);

/// RDF triple reference.
pub type TripleRef<'a, L = Literal> = Triple<SubjectRef<'a>, Iri<'a>, ObjectRef<'a, L>>;

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

impl<L> Triple<Id, IriBuf, Object<Id, L>> {
	pub fn as_triple_ref(&self) -> TripleRef<L> {
		Triple(
			self.0.as_subject_ref(),
			self.1.as_iri(),
			self.2.as_object_ref(),
		)
	}
}

impl<'a, L> TripleRef<'a, L> {
	pub fn into_owned(self) -> Triple<Id, IriBuf, Object<Id, L>>
	where
		L: Clone,
	{
		Triple(self.0.into_owned(), self.1.to_owned(), self.2.into_owned())
	}
}

impl<V, S: InsertIntoVocabulary<V>, P: InsertIntoVocabulary<V>, O: InsertIntoVocabulary<V>>
	InsertIntoVocabulary<V> for Triple<S, P, O>
{
	type Inserted = Triple<S::Inserted, P::Inserted, O::Inserted>;

	fn insert_into_vocabulary(self, vocabulary: &mut V) -> Self::Inserted {
		Triple(
			self.0.insert_into_vocabulary(vocabulary),
			self.1.insert_into_vocabulary(vocabulary),
			self.2.insert_into_vocabulary(vocabulary),
		)
	}
}

impl<
		V,
		S: InsertedIntoVocabulary<V>,
		P: InsertedIntoVocabulary<V>,
		O: InsertedIntoVocabulary<V>,
	> InsertedIntoVocabulary<V> for Triple<S, P, O>
{
	type Inserted = Triple<S::Inserted, P::Inserted, O::Inserted>;

	fn inserted_into_vocabulary(&self, vocabulary: &mut V) -> Self::Inserted {
		Triple(
			self.0.inserted_into_vocabulary(vocabulary),
			self.1.inserted_into_vocabulary(vocabulary),
			self.2.inserted_into_vocabulary(vocabulary),
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
