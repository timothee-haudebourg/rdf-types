use std::{borrow::Cow, cell::RefCell};

use iref::Iri;

use crate::{
	interpretation::{
		ConstGenerativeInterpretation, LocalInterpretation, ReverseInterpretation,
		ReverseLocalInterpretation,
	},
	BlankId, CowLiteral, Interpretation, InterpretationMut, LiteralRef, LocalTerm,
};

use super::LocalGenerator;

pub struct LocalGeneratorInterpretation<G>(RefCell<G>);

impl<G> LocalGeneratorInterpretation<G> {
	pub fn new(generator: G) -> Self {
		Self(RefCell::new(generator))
	}

	pub fn into_generator(self) -> G {
		self.0.into_inner()
	}
}

impl<G> Interpretation for LocalGeneratorInterpretation<G> {
	type Resource = LocalTerm;

	fn iri(&self, iri: &Iri) -> Option<Self::Resource> {
		Some(iri.to_owned().into())
	}

	fn literal<'a>(&self, literal: impl Into<LiteralRef<'a>>) -> Option<Self::Resource> {
		Some(literal.into().to_owned().into())
	}
}

impl<G> LocalInterpretation for LocalGeneratorInterpretation<G> {
	fn blank_id(&self, blank_id: &BlankId) -> Option<Self::Resource> {
		Some(blank_id.to_owned().into())
	}
}

impl<G> InterpretationMut for LocalGeneratorInterpretation<G> {
	fn insert_iri<'a>(&mut self, iri: impl Into<Cow<'a, Iri>>) -> Self::Resource {
		iri.into().into_owned().into()
	}

	fn insert_literal<'a>(&mut self, literal: impl Into<CowLiteral<'a>>) -> Self::Resource {
		literal.into().into_owned().into()
	}
}

impl<G> ReverseInterpretation for LocalGeneratorInterpretation<G> {
	type Iris<'a>
		= std::option::IntoIter<Cow<'a, Iri>>
	where
		Self: 'a;
	type Literals<'a>
		= std::option::IntoIter<CowLiteral<'a>>
	where
		Self: 'a;

	fn iris_of<'a>(&'a self, resource: &'a Self::Resource) -> Self::Iris<'a> {
		resource.as_iri().map(Cow::Borrowed).into_iter()
	}

	fn literals_of<'a>(&'a self, resource: &'a Self::Resource) -> Self::Literals<'a> {
		resource.as_literal().map(LiteralRef::into_cow).into_iter()
	}
}

impl<G> ReverseLocalInterpretation for LocalGeneratorInterpretation<G> {
	type BlankIds<'a>
		= std::option::IntoIter<Cow<'a, BlankId>>
	where
		Self: 'a;

	fn blank_ids_of<'a>(&'a self, resource: &'a Self::Resource) -> Self::BlankIds<'a> {
		resource.as_blank_id().map(Cow::Borrowed).into_iter()
	}
}

impl<G: LocalGenerator> ConstGenerativeInterpretation for LocalGeneratorInterpretation<G> {
	fn new_resource(&self) -> Self::Resource {
		let mut generator = self.0.borrow_mut();
		generator.next_local_term()
	}
}
