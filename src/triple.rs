use std::{cmp::Ordering, fmt};

use iref::{Iri, IriBuf};

use crate::{
	vocabulary::{EmbedIntoVocabulary, EmbeddedIntoVocabulary},
	Id, LexicalObjectRef, LexicalSubjectRef, Object, Quad, RdfDisplay, Term,
};

#[cfg(feature = "contextual")]
use contextual::{DisplayWithContext, WithContext};

#[cfg(feature = "contextual")]
use crate::RdfDisplayWithContext;

/// Lexical RDF triple.
pub type LexicalTriple = Triple<Id, IriBuf, Object>;

/// Lexical RDF triple reference.
pub type LexicalTripleRef<'a> = Triple<LexicalSubjectRef<'a>, &'a Iri, LexicalObjectRef<'a>>;

/// RDF triple.
#[derive(Clone, Copy, Eq, Ord, Hash, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Triple<S = Term, P = S, O = S>(pub S, pub P, pub O);

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

	/// Borrows each component of the triple.
	pub fn as_ref(&self) -> Triple<&S, &P, &O> {
		Triple(&self.0, &self.1, &self.2)
	}
}

impl<'s, 'p, 'o, S, P, O> Triple<&'s S, &'p P, &'o O> {
	pub fn cloned(&self) -> Triple<S, P, O>
	where
		S: Clone,
		P: Clone,
		O: Clone,
	{
		Triple(self.0.clone(), self.1.clone(), self.2.clone())
	}

	pub fn into_cloned(self) -> Triple<S, P, O>
	where
		S: Clone,
		P: Clone,
		O: Clone,
	{
		Triple(self.0.clone(), self.1.clone(), self.2.clone())
	}
}

impl<'s, 'p, 'o, S, P, O> Triple<&'s S, &'p P, &'o O> {
	pub fn copied(&self) -> Triple<S, P, O>
	where
		S: Copy,
		P: Copy,
		O: Copy,
	{
		Triple(*self.0, *self.1, *self.2)
	}

	pub fn into_copied(self) -> Triple<S, P, O>
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

impl LexicalTriple {
	pub fn as_lexical_triple_ref(&self) -> LexicalTripleRef {
		Triple(
			self.0.as_lexical_subject_ref(),
			self.1.as_iri(),
			self.2.as_lexical_object_ref(),
		)
	}
}

impl<'a> LexicalTripleRef<'a> {
	pub fn into_owned(self) -> LexicalTriple {
		Triple(self.0.into_owned(), self.1.to_owned(), self.2.into_owned())
	}
}

impl<V, S: EmbedIntoVocabulary<V>, P: EmbedIntoVocabulary<V>, O: EmbedIntoVocabulary<V>>
	EmbedIntoVocabulary<V> for Triple<S, P, O>
{
	type Embedded = Triple<S::Embedded, P::Embedded, O::Embedded>;

	fn embed_into_vocabulary(self, vocabulary: &mut V) -> Self::Embedded {
		Triple(
			self.0.embed_into_vocabulary(vocabulary),
			self.1.embed_into_vocabulary(vocabulary),
			self.2.embed_into_vocabulary(vocabulary),
		)
	}
}

impl<
		V,
		S: EmbeddedIntoVocabulary<V>,
		P: EmbeddedIntoVocabulary<V>,
		O: EmbeddedIntoVocabulary<V>,
	> EmbeddedIntoVocabulary<V> for Triple<S, P, O>
{
	type Embedded = Triple<S::Embedded, P::Embedded, O::Embedded>;

	fn inserted_into_vocabulary(&self, vocabulary: &mut V) -> Self::Embedded {
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
