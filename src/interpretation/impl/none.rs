use std::borrow::Cow;

use iref::Iri;

use crate::{
	interpretation::{Interpretation, ReverseInterpretation},
	CowLiteral, InterpretationMut, LiteralRef, LocalTerm, Term,
};

impl Interpretation for () {
	type Resource = LocalTerm;

	fn iri<'a>(&self, iri: &'a Iri) -> Option<LocalTerm> {
		Some(LocalTerm::iri(iri.to_owned()))
	}

	fn literal<'a>(&self, literal: impl Into<LiteralRef<'a>>) -> Option<LocalTerm> {
		Some(LocalTerm::literal(literal.into().to_owned()))
	}
}

impl InterpretationMut for () {
	fn insert_iri<'a>(&mut self, iri: impl Into<Cow<'a, Iri>>) -> Self::Resource {
		LocalTerm::iri(iri.into().into_owned())
	}

	fn insert_literal<'a>(&mut self, literal: impl Into<CowLiteral<'a>>) -> Self::Resource {
		LocalTerm::literal(literal.into().into_owned())
	}
}

impl ReverseInterpretation for () {
	type Iris<'a> = std::option::IntoIter<Cow<'a, Iri>>;
	type Literals<'a> = std::option::IntoIter<CowLiteral<'a>>;

	fn iris_of<'a>(&'a self, resource: &'a Self::Resource) -> Self::Iris<'a> {
		match resource {
			LocalTerm::Named(Term::Iri(iri)) => Some(Cow::Borrowed(iri.as_iri())).into_iter(),
			_ => None.into_iter(),
		}
	}

	fn literals_of<'a>(&'a self, resource: &'a Self::Resource) -> Self::Literals<'a> {
		match resource {
			LocalTerm::Named(Term::Literal(l)) => Some(l.as_cow()).into_iter(),
			_ => None.into_iter(),
		}
	}
}
