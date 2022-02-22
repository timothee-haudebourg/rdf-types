//! This is a utility library providing common types
//! when dealing with RDF data:
//! blank node identifier, literal, subject, predicate, object,
//! graph label, gRDF term, triple and quad.
//!
//! The optional feature `loc` provides compatibility
//! with the `locspan` crate to locate every sub-component
//! of a term.
use iref::{IriRef, IriRefBuf};
use std::fmt;

mod blankid;
mod literal;
mod term;

#[cfg(features = "loc")]
pub mod loc;

pub use blankid::*;
pub use literal::*;
pub use term::*;

/// RDF triple.
pub struct Triple<S = Subject, P = IriRefBuf, O = Object>(pub S, pub P, pub O);

impl<S, P, O> Triple<S, P, O> {
	/// Returns a reference to the subject of the triple,
	/// the first component.
	pub fn subject(&self) -> &S {
		&self.0
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

	/// Turns the triple into its object,
	/// the third component.
	pub fn into_object(self) -> O {
		self.2
	}

	/// Turns the triple into a tuple
	pub fn into_parts(self) -> (S, P, O) {
		(self.0, self.1, self.2)
	}
}

impl<S: fmt::Display, P: fmt::Display, O: fmt::Display> fmt::Display for Triple<S, P, O> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{} {} {}", self.0, self.1, self.2)
	}
}

/// RDF triple reference.
pub type TripleRef<'a> = Triple<SubjectRef<'a>, IriRef<'a>, ObjectRef<'a>>;

/// RDF quad.
pub struct Quad<S = Subject, P = IriRefBuf, O = Object, G = GraphLabel>(
	pub S,
	pub P,
	pub O,
	pub Option<G>,
);

impl<S, P, O, G> Quad<S, P, O, G> {
	/// Returns a reference to the subject of the quad,
	/// the first component.
	pub fn subject(&self) -> &S {
		&self.0
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

	/// Turns the quad into its graph,
	/// the fourth component.
	pub fn into_graph(self) -> Option<G> {
		self.3
	}

	pub fn into_parts(self) -> (S, P, O, Option<G>) {
		(self.0, self.1, self.2, self.3)
	}
}

impl<S: fmt::Display, P: fmt::Display, O: fmt::Display, G: fmt::Display> fmt::Display
	for Quad<S, P, O, G>
{
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self.graph() {
			Some(graph) => write!(f, "{} {} {} {}", self.0, self.1, self.2, graph),
			None => write!(f, "{} {} {}", self.0, self.1, self.2),
		}
	}
}

/// RDF quad reference.
pub type QuadRef<'a> = Quad<SubjectRef<'a>, IriRef<'a>, ObjectRef<'a>, GraphLabelRef<'a>>;
