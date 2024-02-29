//! Resource interpretations.
use crate::{Id, Literal, Quad, Term};

mod r#impl;
pub use r#impl::*;

mod iri;
pub use iri::*;

mod blank_id;
pub use blank_id::*;

mod literal;
pub use literal::*;

mod id;
pub use id::*;

mod term;
pub use term::*;

pub mod fallible;
pub use fallible::FallibleInterpretation;

/// RDF resource interpretation.
pub trait Interpretation {
	/// Resource identifier type.
	type Resource;
}

impl<'a, I: Interpretation> Interpretation for &'a I {
	type Resource = I::Resource;
}

impl<'a, I: Interpretation> Interpretation for &'a mut I {
	type Resource = I::Resource;
}

pub trait TraversableInterpretation: Interpretation {
	/// Interpreted resource iterator.
	type Resources<'a>: Iterator<Item = &'a Self::Resource>
	where
		Self: 'a;

	/// Returns an iterator over the interpreted resources.
	fn resources(&self) -> Self::Resources<'_>;
}

impl<'i, I: TraversableInterpretation> TraversableInterpretation for &'i I {
	type Resources<'a> = I::Resources<'a> where Self: 'a;

	fn resources(&self) -> Self::Resources<'_> {
		I::resources(*self)
	}
}

impl<'i, I: TraversableInterpretation> TraversableInterpretation for &'i mut I {
	type Resources<'a> = I::Resources<'a> where Self: 'a;

	fn resources(&self) -> Self::Resources<'_> {
		I::resources(*self)
	}
}

/// Mutable RDF resource interpretation.
pub trait InterpretationMut<V>: Interpretation {
	/// Creates a new resource.
	fn new_resource(&mut self, vocabulary: &mut V) -> Self::Resource;
}

impl<'t, V, T: InterpretationMut<V>> InterpretationMut<V> for &'t mut T {
	fn new_resource(&mut self, vocabulary: &mut V) -> Self::Resource {
		T::new_resource(*self, vocabulary)
	}
}

pub type UninterpretedIdRef<'a, I> =
	Id<&'a <I as ReverseIriInterpretation>::Iri, &'a <I as ReverseBlankIdInterpretation>::BlankId>;

pub type UninterpretedTermRef<'a, I> =
	Term<UninterpretedIdRef<'a, I>, &'a <I as ReverseLiteralInterpretation>::Literal>;

pub type UninterpretedQuadRef<'a, I> = Quad<
	UninterpretedIdRef<'a, I>,
	&'a <I as ReverseIriInterpretation>::Iri,
	UninterpretedTermRef<'a, I>,
	UninterpretedIdRef<'a, I>,
>;

pub type UninterpretedGrdfQuadRef<'a, I> = Quad<
	UninterpretedTermRef<'a, I>,
	UninterpretedTermRef<'a, I>,
	UninterpretedTermRef<'a, I>,
	UninterpretedTermRef<'a, I>,
>;

/// RDF interpretation function.
pub trait Interpret<I: Interpretation> {
	/// Interpreted form.
	type Interpreted;

	/// Interpret the given resource.
	fn interpret(self, interpretation: &mut I) -> Self::Interpreted;
}

impl<I: Interpretation, T: Interpret<I>> Interpret<I> for Option<T> {
	type Interpreted = Option<T::Interpreted>;

	fn interpret(self, interpretation: &mut I) -> Self::Interpreted {
		self.map(|t| t.interpret(interpretation))
	}
}

impl<I, B, T: IdInterpretationMut<I, B>> Interpret<T> for Id<I, B> {
	type Interpreted = T::Resource;

	fn interpret(self, interpretation: &mut T) -> Self::Interpreted {
		interpretation.interpret_id(self)
	}
}

impl<T, I: LiteralInterpretationMut<Self>> Interpret<I> for Literal<T> {
	type Interpreted = I::Resource;

	fn interpret(self, interpretation: &mut I) -> Self::Interpreted {
		interpretation.interpret_literal(self)
	}
}
