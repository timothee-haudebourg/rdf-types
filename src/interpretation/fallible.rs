use core::fmt;
use std::{borrow::Cow, convert::Infallible};

use iref::Iri;

use super::{GenerativeInterpretation, GroundInterpretation};
use crate::{
	interpretation::{
		Interpretation, InterpretationMut, ReverseGroundInterpretation, ReverseInterpretation,
		TraversableInterpretation,
	},
	utils::InfallibleIterator,
	BlankId, CowGroundTerm, CowId, CowLiteral, CowTerm, GroundInterpretationMut, GroundTermRef,
	IdRef, LiteralRef, TermRef,
};

/// Fallible ground interpretation.
pub trait FallibleGroundInterpretation {
	/// Resource type.
	type Resource;

	/// Error type.
	type Error: fmt::Debug + fmt::Display;

	fn try_iri(&self, iri: &Iri) -> Result<Option<Self::Resource>, Self::Error>;

	fn try_literal<'a>(
		&self,
		literal: impl Into<LiteralRef<'a>>,
	) -> Result<Option<Self::Resource>, Self::Error>;

	fn try_ground_term<'a>(
		&self,
		term: impl Into<GroundTermRef<'a>>,
	) -> Result<Option<Self::Resource>, Self::Error> {
		match term.into() {
			GroundTermRef::Iri(iri) => self.try_iri(iri),
			GroundTermRef::Literal(l) => self.try_literal(l),
		}
	}
}

/// Any non-fallible interpretation can be used as fallible, with the
/// [`Infallible`] error type.
impl<I: GroundInterpretation> FallibleGroundInterpretation for I {
	type Resource = I::Resource;
	type Error = Infallible;

	fn try_iri(&self, iri: &Iri) -> Result<Option<Self::Resource>, Self::Error> {
		Ok(self.iri(iri))
	}

	fn try_literal<'a>(
		&self,
		literal: impl Into<LiteralRef<'a>>,
	) -> Result<Option<Self::Resource>, Self::Error> {
		Ok(self.literal(literal))
	}

	fn try_ground_term<'a>(
		&self,
		term: impl Into<GroundTermRef<'a>>,
	) -> Result<Option<Self::Resource>, Self::Error> {
		Ok(self.ground_term(term))
	}
}

/// Traversable fallible interpretation.
///
/// Provides the `try_resources` method to (try to) iterate over all the
/// resources of the interpretation.
pub trait TraversableFallibleInterpretation: FallibleGroundInterpretation {
	/// Fallible iterator over all the resources of the interpretation.
	type TryResources<'a>: Iterator<Item = Result<&'a Self::Resource, Self::Error>>
	where
		Self: 'a;

	/// Returns a fallible iterator over all the resources of the
	/// interpretation.
	fn try_resources(&self) -> Self::TryResources<'_>;
}

/// Any non-fallible traversable interpretation can be used as fallible, with
/// the [`Infallible`] error type.
impl<I: TraversableInterpretation> TraversableFallibleInterpretation for I {
	type TryResources<'a>
		= InfallibleIterator<I::Resources<'a>>
	where
		Self: 'a;

	fn try_resources(&self) -> Self::TryResources<'_> {
		InfallibleIterator(self.resources())
	}
}

/// Fallible generative interpretation.
pub trait FallibleGenerativeInterpretation: FallibleGroundInterpretation {
	/// Tries to create a new fresh resource.
	fn try_new_resource(&mut self) -> Result<Self::Resource, Self::Error>;
}

/// Any non-fallible generative interpretation can be used as fallible, with the
/// [`Infallible`] error type.
impl<I: GenerativeInterpretation> FallibleGenerativeInterpretation for I {
	fn try_new_resource(&mut self) -> Result<Self::Resource, Self::Error> {
		Ok(self.new_resource())
	}
}

/// Mutable fallible interpretation.
pub trait FallibleGroundInterpretationMut: FallibleGroundInterpretation {
	fn try_insert_iri<'a>(
		&mut self,
		iri: impl Into<Cow<'a, Iri>>,
	) -> Result<Self::Resource, Self::Error>;

	fn try_insert_literal<'a>(
		&mut self,
		literal: impl Into<CowLiteral<'a>>,
	) -> Result<Self::Resource, Self::Error>;

	fn try_insert_ground_term<'a>(
		&mut self,
		term: impl Into<CowGroundTerm<'a>>,
	) -> Result<Self::Resource, Self::Error> {
		match term.into() {
			CowGroundTerm::Iri(iri) => self.try_insert_iri(iri),
			CowGroundTerm::Literal(literal) => self.try_insert_literal(literal),
		}
	}
}

/// Any mutable ground interpretation can be used as fallible, with the
/// [`Infallible`] error type.
impl<I: GroundInterpretationMut> FallibleGroundInterpretationMut for I {
	fn try_insert_iri<'a>(
		&mut self,
		iri: impl Into<Cow<'a, Iri>>,
	) -> Result<Self::Resource, Self::Error> {
		Ok(self.insert_iri(iri))
	}

	fn try_insert_literal<'a>(
		&mut self,
		literal: impl Into<CowLiteral<'a>>,
	) -> Result<Self::Resource, Self::Error> {
		Ok(self.insert_literal(literal))
	}

	fn try_insert_ground_term<'a>(
		&mut self,
		term: impl Into<CowGroundTerm<'a>>,
	) -> Result<Self::Resource, Self::Error> {
		Ok(self.insert_ground_term(term))
	}
}

/// Fallible reverse ground interpretation.
pub trait FallibleReverseGroundInterpretation: FallibleGroundInterpretation {
	type TryIris<'a>: Iterator<Item = Result<Cow<'a, Iri>, Self::Error>>
	where
		Self: 'a;
	type TryLiterals<'a>: Iterator<Item = Result<CowLiteral<'a>, Self::Error>>
	where
		Self: 'a;

	fn try_iris_of<'a>(&'a self, resource: &'a Self::Resource) -> Self::TryIris<'a>;

	fn try_literals_of<'a>(&'a self, resource: &'a Self::Resource) -> Self::TryLiterals<'a>;

	fn try_ground_terms_of<'a>(
		&'a self,
		resource: &'a Self::Resource,
	) -> TryGroundTermsOf<'a, Self> {
		TryGroundTermsOf {
			iris: self.try_iris_of(resource),
			literals: self.try_literals_of(resource),
		}
	}

	fn try_is_anonymous(&self, resource: &Self::Resource) -> Result<bool, Self::Error> {
		Ok(self
			.try_ground_terms_of(resource)
			.next()
			.transpose()?
			.is_none())
	}
}

impl<I: ReverseGroundInterpretation> FallibleReverseGroundInterpretation for I {
	type TryIris<'a>
		= InfallibleIterator<I::Iris<'a>>
	where
		Self: 'a;

	type TryLiterals<'a>
		= InfallibleIterator<I::Literals<'a>>
	where
		Self: 'a;

	fn try_iris_of<'a>(&'a self, resource: &'a Self::Resource) -> Self::TryIris<'a> {
		InfallibleIterator(self.iris_of(resource))
	}

	fn try_literals_of<'a>(&'a self, resource: &'a Self::Resource) -> Self::TryLiterals<'a> {
		InfallibleIterator(self.literals_of(resource))
	}

	fn try_is_anonymous(&self, resource: &Self::Resource) -> Result<bool, Self::Error> {
		Ok(self.is_anonymous(resource))
	}
}

pub struct TryGroundTermsOf<'a, I: 'a + ?Sized + FallibleReverseGroundInterpretation> {
	iris: I::TryIris<'a>,
	literals: I::TryLiterals<'a>,
}

impl<'a, I: 'a + ?Sized + FallibleReverseGroundInterpretation> Iterator
	for TryGroundTermsOf<'a, I>
{
	type Item = Result<CowGroundTerm<'a>, I::Error>;

	fn next(&mut self) -> Option<Self::Item> {
		self.iris
			.next()
			.map(|r| r.map(CowGroundTerm::Iri))
			.or_else(|| self.literals.next().map(|r| r.map(CowGroundTerm::Literal)))
	}
}

/// Fallible interpretation.
///
/// Same as a fallible ground interpretation, but also interprets blank node
/// identifiers.
pub trait FallibleInterpretation: FallibleGroundInterpretation {
	fn try_blank_id(&self, blank_id: &BlankId) -> Result<Option<Self::Resource>, Self::Error>;

	fn try_term<'a>(
		&self,
		term: impl Into<TermRef<'a>>,
	) -> Result<Option<Self::Resource>, Self::Error> {
		match term.into() {
			TermRef::BlankId(blank_id) => self.try_blank_id(blank_id),
			TermRef::Ground(term) => self.try_ground_term(term),
		}
	}

	fn try_id<'a>(&self, id: impl Into<IdRef<'a>>) -> Result<Option<Self::Resource>, Self::Error> {
		match id.into() {
			IdRef::BlankId(blank_id) => self.try_blank_id(blank_id),
			IdRef::Iri(iri) => self.try_iri(iri),
		}
	}
}

/// Any non-fallible interpretation is can be used a fallible, with the
/// [`Infallible`] error type.
impl<I: Interpretation> FallibleInterpretation for I {
	fn try_blank_id(&self, blank_id: &BlankId) -> Result<Option<Self::Resource>, Self::Error> {
		Ok(self.blank_id(blank_id))
	}

	fn try_term<'a>(
		&self,
		term: impl Into<TermRef<'a>>,
	) -> Result<Option<Self::Resource>, Self::Error> {
		Ok(self.term(term))
	}

	fn try_id<'a>(&self, id: impl Into<IdRef<'a>>) -> Result<Option<Self::Resource>, Self::Error> {
		Ok(self.id(id))
	}
}

/// Mutable fallible interpretation.
pub trait FallibleInterpretationMut:
	FallibleInterpretation + FallibleGroundInterpretationMut
{
	fn try_insert_blank_id<'a>(
		&mut self,
		blank_id: impl Into<Cow<'a, BlankId>>,
	) -> Result<Self::Resource, Self::Error>;

	fn try_insert_term<'a>(
		&mut self,
		term: impl Into<CowTerm<'a>>,
	) -> Result<Self::Resource, Self::Error> {
		match term.into() {
			CowTerm::BlankId(blank_id) => self.try_insert_blank_id(blank_id),
			CowTerm::Ground(term) => self.try_insert_ground_term(term),
		}
	}

	fn try_insert_id<'a>(
		&mut self,
		term: impl Into<CowId<'a>>,
	) -> Result<Self::Resource, Self::Error> {
		match term.into() {
			CowId::BlankId(blank_id) => self.try_insert_blank_id(blank_id),
			CowId::Iri(iri) => self.try_insert_iri(iri),
		}
	}
}

/// Any non-fallible mutable interpretation can be used as fallible, with the
/// [`Infallible`] error type.
impl<I: InterpretationMut> FallibleInterpretationMut for I {
	fn try_insert_blank_id<'a>(
		&mut self,
		blank_id: impl Into<Cow<'a, BlankId>>,
	) -> Result<Self::Resource, Self::Error> {
		Ok(self.insert_blank_id(blank_id))
	}

	fn try_insert_term<'a>(
		&mut self,
		term: impl Into<CowTerm<'a>>,
	) -> Result<Self::Resource, Self::Error> {
		Ok(self.insert_term(term))
	}

	fn try_insert_id<'a>(
		&mut self,
		term: impl Into<CowId<'a>>,
	) -> Result<Self::Resource, Self::Error> {
		Ok(self.insert_id(term))
	}
}

/// Fallible reverse interpretation.
pub trait FallibleReverseInterpretation: FallibleReverseGroundInterpretation {
	type TryBlankIds<'a>: Iterator<Item = Result<Cow<'a, BlankId>, Self::Error>>
	where
		Self: 'a;

	fn try_blank_ids_of<'a>(&'a self, resource: &'a Self::Resource) -> Self::TryBlankIds<'a>;

	fn try_terms_of<'a>(&'a self, resource: &'a Self::Resource) -> TryTermsOf<'a, Self> {
		TryTermsOf {
			terms: self.try_ground_terms_of(resource),
			blank_ids: self.try_blank_ids_of(resource),
		}
	}
}

/// Any non-fallible reverse interpretation can be used as fallible, with the
/// [`Infallible`] error type.
impl<I: ReverseInterpretation> FallibleReverseInterpretation for I {
	type TryBlankIds<'a>
		= InfallibleIterator<I::BlankIds<'a>>
	where
		Self: 'a;

	fn try_blank_ids_of<'a>(&'a self, resource: &'a Self::Resource) -> Self::TryBlankIds<'a> {
		InfallibleIterator(self.blank_ids_of(resource))
	}
}

pub struct TryTermsOf<'a, I: 'a + ?Sized + FallibleReverseInterpretation> {
	terms: TryGroundTermsOf<'a, I>,
	blank_ids: I::TryBlankIds<'a>,
}

impl<'a, I: 'a + ?Sized + FallibleReverseInterpretation> Iterator for TryTermsOf<'a, I> {
	type Item = Result<CowTerm<'a>, I::Error>;

	fn next(&mut self) -> Option<Self::Item> {
		self.terms
			.next()
			.map(|r| r.map(CowTerm::Ground))
			.or_else(|| self.blank_ids.next().map(|r| r.map(CowTerm::BlankId)))
	}
}
