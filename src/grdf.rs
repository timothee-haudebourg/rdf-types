use crate::{Id, Literal, Quad, Term, Triple};

/// gRDF quad.
///
/// A quad where each component is a [`Term`].
pub type GrdfQuad<I = Id, L = Literal> = Quad<Term<I, L>>;

impl<I, B, L> Quad<Id<I, B>, I, Term<Id<I, B>, L>, Id<I, B>> {
	pub fn into_grdf(self) -> GrdfQuad<Id<I, B>, L> {
		self.map_subject(|s| Term::Id(s))
			.map_predicate(|p| Term::Id(Id::Iri(p)))
			.map_graph(|g| g.map(Term::Id))
	}
}

/// gRDF triple.
///
/// A triple where each component is a [`Term`].
pub type GrdfTriple<I, L> = Triple<Term<I, L>>;

impl<I, B, L> Triple<Id<I, B>, I, Term<Id<I, B>, L>> {
	pub fn into_grdf(self) -> GrdfTriple<Id<I, B>, L> {
		self.map_subject(|s| Term::Id(s))
			.map_predicate(|p| Term::Id(Id::Iri(p)))
	}
}
