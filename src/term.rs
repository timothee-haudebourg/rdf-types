use crate::{BlankId, BlankIdBuf, Literal};
use iref::{Iri, IriBuf};
use std::fmt;

/// gRDF term.
///
/// Either a blank node identifier, IRI or literal value.
#[derive(Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Debug)]
pub enum Term {
	/// Blank node identifier.
	Blank(BlankIdBuf),

	/// IRI.
	Iri(IriBuf),

	/// Literal value.
	Literal(Literal),
}

impl Term {
	pub fn as_term_ref(&self) -> TermRef {
		match self {
			Self::Blank(id) => TermRef::Blank(id),
			Self::Iri(iri) => TermRef::Iri(iri.as_iri()),
			Self::Literal(lit) => TermRef::Literal(lit),
		}
	}

	pub fn as_object_ref(&self) -> TermRef {
		self.as_term_ref()
	}

	pub fn is_blank(&self) -> bool {
		matches!(self, Self::Blank(_))
	}

	pub fn is_iri(&self) -> bool {
		matches!(self, Self::Iri(_))
	}

	pub fn is_literal(&self) -> bool {
		matches!(self, Self::Literal(_))
	}

	pub fn as_blank(&self) -> Option<&BlankId> {
		match self {
			Self::Blank(id) => Some(id),
			_ => None,
		}
	}

	pub fn as_iri(&self) -> Option<Iri> {
		match self {
			Self::Iri(iri) => Some(iri.as_iri()),
			_ => None,
		}
	}

	pub fn as_literal(&self) -> Option<&Literal> {
		match self {
			Self::Literal(lit) => Some(lit),
			_ => None,
		}
	}
}

impl fmt::Display for Term {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			Self::Blank(id) => id.fmt(f),
			Self::Iri(iri) => write!(f, "<{}>", iri),
			Self::Literal(lit) => lit.fmt(f),
		}
	}
}

/// gRDF term reference.
///
/// Either a blank node identifier, IRI or literal value.
#[derive(Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Debug)]
pub enum TermRef<'a> {
	/// Blank node identifier.
	Blank(&'a BlankId),

	/// IRI.
	Iri(Iri<'a>),

	/// Literal value.
	Literal(&'a Literal),
}

impl<'a> TermRef<'a> {
	pub fn is_blank(&self) -> bool {
		matches!(self, Self::Blank(_))
	}

	pub fn is_iri(&self) -> bool {
		matches!(self, Self::Iri(_))
	}

	pub fn is_literal(&self) -> bool {
		matches!(self, Self::Literal(_))
	}

	pub fn as_blank(&self) -> Option<&'a BlankId> {
		match self {
			Self::Blank(id) => Some(id),
			_ => None,
		}
	}

	pub fn as_iri(&self) -> Option<Iri<'a>> {
		match self {
			Self::Iri(iri) => Some(*iri),
			_ => None,
		}
	}

	pub fn as_literal(&self) -> Option<&'a Literal> {
		match self {
			Self::Literal(lit) => Some(lit),
			_ => None,
		}
	}
}

impl<'a> From<&'a Term> for TermRef<'a> {
	fn from(t: &'a Term) -> Self {
		t.as_term_ref()
	}
}

impl<'a> fmt::Display for TermRef<'a> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			Self::Blank(id) => id.fmt(f),
			Self::Iri(iri) => write!(f, "<{}>", iri),
			Self::Literal(lit) => lit.fmt(f),
		}
	}
}

/// RDF Subject.
///
/// Either a blank node identifier or an IRI.
#[derive(Clone, Debug)]
pub enum Subject {
	/// Blank node identifier.
	Blank(BlankIdBuf),

	/// IRI.
	Iri(IriBuf),
}

impl Subject {
	pub fn as_subject_ref(&self) -> SubjectRef {
		match self {
			Self::Blank(id) => SubjectRef::Blank(id),
			Self::Iri(iri) => SubjectRef::Iri(iri.as_iri()),
		}
	}

	pub fn as_graph_label_ref(&self) -> GraphLabelRef {
		self.as_subject_ref()
	}

	pub fn is_blank(&self) -> bool {
		matches!(self, Self::Blank(_))
	}

	pub fn is_iri(&self) -> bool {
		matches!(self, Self::Iri(_))
	}

	pub fn as_blank(&self) -> Option<&BlankId> {
		match self {
			Self::Blank(id) => Some(id),
			_ => None,
		}
	}

	pub fn as_iri(&self) -> Option<Iri> {
		match self {
			Self::Iri(iri) => Some(iri.as_iri()),
			_ => None,
		}
	}

	pub fn into_term(self) -> Term {
		match self {
			Self::Blank(id) => Term::Blank(id),
			Self::Iri(iri) => Term::Iri(iri),
		}
	}

	pub fn as_term_ref(&self) -> TermRef {
		match self {
			Self::Blank(id) => TermRef::Blank(id),
			Self::Iri(iri) => TermRef::Iri(iri.as_iri()),
		}
	}
}

impl fmt::Display for Subject {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			Self::Blank(id) => id.fmt(f),
			Self::Iri(iri) => write!(f, "<{}>", iri),
		}
	}
}

/// gRDF subject or graph label reference.
///
/// Either a blank node identifier or an IRI.
#[derive(Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Debug)]
pub enum SubjectRef<'a> {
	/// Blank node identifier.
	Blank(&'a BlankId),

	/// IRI.
	Iri(Iri<'a>),
}

impl<'a> SubjectRef<'a> {
	pub fn is_blank(&self) -> bool {
		matches!(self, Self::Blank(_))
	}

	pub fn is_iri(&self) -> bool {
		matches!(self, Self::Iri(_))
	}

	pub fn as_blank(&self) -> Option<&'a BlankId> {
		match self {
			Self::Blank(id) => Some(id),
			_ => None,
		}
	}

	pub fn as_iri(&self) -> Option<Iri<'a>> {
		match self {
			Self::Iri(iri) => Some(*iri),
			_ => None,
		}
	}

	pub fn into_term_ref(self) -> TermRef<'a> {
		match self {
			Self::Blank(id) => TermRef::Blank(id),
			Self::Iri(iri) => TermRef::Iri(iri),
		}
	}
}

impl<'a> From<&'a Subject> for SubjectRef<'a> {
	fn from(t: &'a Subject) -> Self {
		t.as_subject_ref()
	}
}

impl<'a> fmt::Display for SubjectRef<'a> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			Self::Blank(id) => id.fmt(f),
			Self::Iri(iri) => write!(f, "<{}>", iri),
		}
	}
}

/// RDF Object.
pub type Object = Term;

/// RDF Object reference.
pub type ObjectRef<'a> = Term;

/// RDF Graph Label.
pub type GraphLabel = Subject;

/// RDF Graph Label reference.
pub type GraphLabelRef<'a> = SubjectRef<'a>;
