//! Resource interpretations.
use std::borrow::Cow;

use crate::{
	BlankId, CowId, CowLiteral, CowLocalTerm, CowTerm, IdRef, LiteralRef, LocalTermRef, TermRef,
};

mod r#impl;
pub use r#impl::*;

pub mod fallible;
pub use fallible::FallibleInterpretation;

use iref::Iri;

/// RDF resource interpretation.
pub trait Interpretation {
	type Resource;

	fn iri(&self, iri: &Iri) -> Option<Self::Resource>;

	fn literal<'a>(&self, literal: impl Into<LiteralRef<'a>>) -> Option<Self::Resource>;

	fn term<'a>(&self, term: impl Into<TermRef<'a>>) -> Option<Self::Resource> {
		match term.into() {
			TermRef::Iri(iri) => self.iri(iri),
			TermRef::Literal(l) => self.literal(l),
		}
	}
}

/// Interpretation that can return an iterator over the known RDF resources.
pub trait TraversableInterpretation: Interpretation {
	type Resources<'a>: Iterator<Item = &'a Self::Resource>
	where
		Self: 'a;

	fn resources(&self) -> Self::Resources<'_>;
}

/// Interpretation that can spawn fresh new resources.
pub trait GenerativeInterpretation: Interpretation {
	/// Create a new resource.
	fn new_resource(&mut self) -> Self::Resource;
}

/// Interpretation that can spawn fresh new resources.
pub trait ConstGenerativeInterpretation: Interpretation {
	/// Create a new resource.
	fn new_resource(&self) -> Self::Resource;
}

impl<I: ConstGenerativeInterpretation> GenerativeInterpretation for I {
	fn new_resource(&mut self) -> Self::Resource {
		ConstGenerativeInterpretation::new_resource(self)
	}
}

/// Mutable interpretation.
pub trait InterpretationMut: Interpretation {
	fn insert_iri<'a>(&mut self, iri: impl Into<Cow<'a, Iri>>) -> Self::Resource;

	fn insert_literal<'a>(&mut self, literal: impl Into<CowLiteral<'a>>) -> Self::Resource;

	fn insert_term<'a>(&mut self, term: impl Into<CowTerm<'a>>) -> Self::Resource {
		match term.into() {
			CowTerm::Iri(iri) => self.insert_iri(iri),
			CowTerm::Literal(literal) => self.insert_literal(literal),
		}
	}
}

/// Reverse interpretation.
pub trait ReverseInterpretation: Interpretation {
	type Iris<'a>: Iterator<Item = Cow<'a, Iri>>
	where
		Self: 'a;
	type Literals<'a>: Iterator<Item = CowLiteral<'a>>
	where
		Self: 'a;

	fn iris_of<'a>(&'a self, resource: &'a Self::Resource) -> Self::Iris<'a>;

	fn literals_of<'a>(&'a self, resource: &'a Self::Resource) -> Self::Literals<'a>;

	fn terms_of<'a>(&'a self, resource: &'a Self::Resource) -> TermsOf<'a, Self> {
		TermsOf {
			iris: self.iris_of(resource),
			literals: self.literals_of(resource),
		}
	}

	fn is_anonymous(&self, resource: &Self::Resource) -> bool {
		self.terms_of(resource).next().is_none()
	}
}

pub struct TermsOf<'a, I: 'a + ?Sized + ReverseInterpretation> {
	iris: I::Iris<'a>,
	literals: I::Literals<'a>,
}

impl<'a, I: 'a + ?Sized + ReverseInterpretation> Iterator for TermsOf<'a, I> {
	type Item = CowTerm<'a>;

	fn next(&mut self) -> Option<Self::Item> {
		self.iris
			.next()
			.map(CowTerm::Iri)
			.or_else(|| self.literals.next().map(CowTerm::Literal))
	}
}

pub trait LocalInterpretation: Interpretation {
	fn blank_id(&self, blank_id: &BlankId) -> Option<Self::Resource>;

	fn local_term<'a>(&self, term: impl Into<LocalTermRef<'a>>) -> Option<Self::Resource> {
		match term.into() {
			LocalTermRef::Anonymous(blank_id) => self.blank_id(blank_id),
			LocalTermRef::Named(term) => self.term(term),
		}
	}

	fn id<'a>(&self, id: impl Into<IdRef<'a>>) -> Option<Self::Resource> {
		match id.into() {
			IdRef::BlankId(blank_id) => self.blank_id(blank_id),
			IdRef::Iri(iri) => self.iri(iri),
		}
	}
}

pub trait LocalInterpretationMut: InterpretationMut {
	fn insert_blank_id<'a>(&mut self, blank_id: impl Into<Cow<'a, BlankId>>) -> Self::Resource;

	fn insert_local_term<'a>(&mut self, term: impl Into<CowLocalTerm<'a>>) -> Self::Resource {
		match term.into() {
			CowLocalTerm::Anonymous(blank_id) => self.insert_blank_id(blank_id),
			CowLocalTerm::Named(term) => self.insert_term(term),
		}
	}

	fn insert_id<'a>(&mut self, term: impl Into<CowId<'a>>) -> Self::Resource {
		match term.into() {
			CowId::BlankId(blank_id) => self.insert_blank_id(blank_id),
			CowId::Iri(iri) => self.insert_iri(iri),
		}
	}
}

pub trait ReverseLocalInterpretation: ReverseInterpretation {
	type BlankIds<'a>: Iterator<Item = Cow<'a, BlankId>>
	where
		Self: 'a;

	fn blank_ids_of<'a>(&'a self, resource: &'a Self::Resource) -> Self::BlankIds<'a>;

	fn local_terms_of<'a>(&'a self, resource: &'a Self::Resource) -> LocalTermsOf<'a, Self> {
		LocalTermsOf {
			terms: self.terms_of(resource),
			blank_ids: self.blank_ids_of(resource),
		}
	}
}

pub struct LocalTermsOf<'a, I: 'a + ?Sized + ReverseLocalInterpretation> {
	terms: TermsOf<'a, I>,
	blank_ids: I::BlankIds<'a>,
}

impl<'a, I: 'a + ?Sized + ReverseLocalInterpretation> Iterator for LocalTermsOf<'a, I> {
	type Item = CowLocalTerm<'a>;

	fn next(&mut self) -> Option<Self::Item> {
		self.terms
			.next()
			.map(CowLocalTerm::Named)
			.or_else(|| self.blank_ids.next().map(CowLocalTerm::Anonymous))
	}
}
