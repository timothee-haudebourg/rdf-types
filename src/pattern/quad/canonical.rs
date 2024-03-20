use crate::{
	pattern::{
		triple::{self, CanonicalTriplePattern},
		QuadPattern, ResourceOrVar, TriplePattern,
	},
	Quad, Triple,
};

/// Canonical triple pattern.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum CanonicalQuadPattern<T> {
	AnySubject(AnySubject<T>),
	GivenSubject(T, GivenSubject<T>),
}

impl<T> From<Triple<T>> for CanonicalQuadPattern<T> {
	fn from(value: Triple<T>) -> Self {
		Self::from_triple(value)
	}
}

impl<T> From<Quad<T>> for CanonicalQuadPattern<T> {
	fn from(value: Quad<T>) -> Self {
		Self::from_quad(value)
	}
}

impl<T> From<Triple<Option<T>>> for CanonicalQuadPattern<T> {
	fn from(value: Triple<Option<T>>) -> Self {
		Self::from_option_triple(value)
	}
}

impl<T> From<Quad<Option<T>>> for CanonicalQuadPattern<T> {
	fn from(value: Quad<Option<T>>) -> Self {
		Self::from_option_quad(value)
	}
}

impl<T, X: PartialEq> From<TriplePattern<T, X>> for CanonicalQuadPattern<T> {
	fn from(value: TriplePattern<T, X>) -> Self {
		Self::from_triple_pattern(value)
	}
}

impl<T, X: PartialEq> From<QuadPattern<T, X>> for CanonicalQuadPattern<T> {
	fn from(value: QuadPattern<T, X>) -> Self {
		Self::from_pattern(value)
	}
}

impl<T> CanonicalQuadPattern<T> {
	pub fn from_triple(triple: Triple<T>) -> Self {
		Self::GivenSubject(
			triple.0,
			GivenSubject::GivenPredicate(
				triple.1,
				GivenSubjectGivenPredicate::GivenObject(
					triple.2,
					GivenSubjectGivenPredicateGivenObject::AnyGraph,
				),
			),
		)
	}

	pub fn from_quad(quad: Quad<T>) -> Self {
		Self::GivenSubject(
			quad.0,
			GivenSubject::GivenPredicate(
				quad.1,
				GivenSubjectGivenPredicate::GivenObject(
					quad.2,
					GivenSubjectGivenPredicateGivenObject::GivenGraph(quad.3),
				),
			),
		)
	}

	pub fn from_option_triple(triple: Triple<Option<T>>) -> Self {
		match triple.0 {
			Some(s) => {
				Self::GivenSubject(s, GivenSubject::from_option_quad(triple.1, triple.2, None))
			}
			None => Self::AnySubject(AnySubject::from_option_quad(triple.1, triple.2, None)),
		}
	}

	pub fn from_option_quad(quad: Quad<Option<T>>) -> Self {
		match quad.0 {
			Some(s) => {
				Self::GivenSubject(s, GivenSubject::from_option_quad(quad.1, quad.2, quad.3))
			}
			None => Self::AnySubject(AnySubject::from_option_quad(quad.1, quad.2, quad.3)),
		}
	}

	pub fn from_triple_pattern<X: PartialEq>(pattern: TriplePattern<T, X>) -> Self {
		match pattern.0 {
			ResourceOrVar::Resource(s) => {
				Self::GivenSubject(s, GivenSubject::from_pattern(pattern.1, pattern.2, None))
			}
			ResourceOrVar::Var(s) => {
				Self::AnySubject(AnySubject::from_pattern(s, pattern.1, pattern.2, None))
			}
		}
	}

	pub fn from_pattern<X: PartialEq>(pattern: QuadPattern<T, X>) -> Self {
		match pattern.0 {
			ResourceOrVar::Resource(s) => Self::GivenSubject(
				s,
				GivenSubject::from_pattern(pattern.1, pattern.2, pattern.3),
			),
			ResourceOrVar::Var(s) => {
				Self::AnySubject(AnySubject::from_pattern(s, pattern.1, pattern.2, pattern.3))
			}
		}
	}

	pub fn subject(&self) -> PatternSubject<&T> {
		match self {
			Self::AnySubject(_) => PatternSubject::Any,
			Self::GivenSubject(id, _) => PatternSubject::Given(id),
		}
	}

	pub fn into_subject(self) -> PatternSubject<T> {
		match self {
			Self::AnySubject(_) => PatternSubject::Any,
			Self::GivenSubject(id, _) => PatternSubject::Given(id),
		}
	}

	pub fn predicate(&self) -> PatternPredicate<&T> {
		match self {
			Self::AnySubject(t) => t.predicate(),
			Self::GivenSubject(_, t) => t.predicate(),
		}
	}

	pub fn into_predicate(self) -> PatternPredicate<T> {
		match self {
			Self::AnySubject(t) => t.into_predicate(),
			Self::GivenSubject(_, t) => t.into_predicate(),
		}
	}

	pub fn object(&self) -> PatternObject<&T> {
		match self {
			Self::AnySubject(t) => t.object(),
			Self::GivenSubject(_, t) => t.object(),
		}
	}

	pub fn into_object(self) -> PatternObject<T> {
		match self {
			Self::AnySubject(t) => t.into_object(),
			Self::GivenSubject(_, t) => t.into_object(),
		}
	}

	pub fn graph(&self) -> PatternGraph<&T> {
		match self {
			Self::AnySubject(t) => t.graph(),
			Self::GivenSubject(_, t) => t.graph(),
		}
	}

	pub fn into_graph(self) -> PatternGraph<T> {
		match self {
			Self::AnySubject(t) => t.into_graph(),
			Self::GivenSubject(_, t) => t.into_graph(),
		}
	}

	pub fn into_triple(self) -> (CanonicalTriplePattern<T>, PatternGraph<T>) {
		match self {
			Self::AnySubject(t) => {
				let (u, g) = t.into_triple();
				(CanonicalTriplePattern::AnySubject(u), g)
			}
			Self::GivenSubject(id, t) => {
				let (u, g) = t.into_triple();
				(CanonicalTriplePattern::GivenSubject(id, u), g)
			}
		}
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum PatternSubject<T> {
	Any,
	Given(T),
}

impl<T> PatternSubject<T> {
	pub fn id(&self) -> Option<&T> {
		match self {
			Self::Any => None,
			Self::Given(id) => Some(id),
		}
	}

	pub fn into_id(self) -> Option<T> {
		match self {
			Self::Any => None,
			Self::Given(id) => Some(id),
		}
	}
}

impl<'a, T> PatternSubject<&'a T> {
	pub fn cloned(self) -> PatternSubject<T>
	where
		T: Clone,
	{
		match self {
			Self::Any => PatternSubject::Any,
			Self::Given(t) => PatternSubject::Given(t.clone()),
		}
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum PatternPredicate<T> {
	Any,
	SameAsSubject,
	Given(T),
}

impl<T> PatternPredicate<T> {
	pub fn id(&self) -> Option<&T> {
		match self {
			Self::Any => None,
			Self::SameAsSubject => None,
			Self::Given(id) => Some(id),
		}
	}

	pub fn into_id(self) -> Option<T> {
		match self {
			Self::Any => None,
			Self::SameAsSubject => None,
			Self::Given(id) => Some(id),
		}
	}
}

impl<'a, T> PatternPredicate<&'a T> {
	pub fn cloned(self) -> PatternPredicate<T>
	where
		T: Clone,
	{
		match self {
			Self::Any => PatternPredicate::Any,
			Self::SameAsSubject => PatternPredicate::SameAsSubject,
			Self::Given(t) => PatternPredicate::Given(t.clone()),
		}
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum PatternObject<T> {
	Any,
	SameAsSubject,
	SameAsPredicate,
	Given(T),
}

impl<T> PatternObject<T> {
	pub fn id(&self) -> Option<&T> {
		match self {
			Self::Given(id) => Some(id),
			_ => None,
		}
	}

	pub fn into_id(self) -> Option<T> {
		match self {
			Self::Given(id) => Some(id),
			_ => None,
		}
	}
}

impl<'a, T> PatternObject<&'a T> {
	pub fn cloned(self) -> PatternObject<T>
	where
		T: Clone,
	{
		match self {
			Self::Any => PatternObject::Any,
			Self::SameAsSubject => PatternObject::SameAsSubject,
			Self::SameAsPredicate => PatternObject::SameAsPredicate,
			Self::Given(t) => PatternObject::Given(t.clone()),
		}
	}
}

impl<T: PartialEq> PatternObject<T> {
	pub fn filter_triple(&self, triple: Triple<T>) -> bool {
		match self {
			Self::Any => true,
			Self::SameAsSubject => triple.2 == triple.0,
			Self::SameAsPredicate => triple.2 == triple.1,
			Self::Given(id) => triple.2 == *id,
		}
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum AnySubject<T> {
	AnyPredicate(AnySubjectAnyPredicate<T>),
	SameAsSubject(AnySubjectGivenPredicate<T>),
	GivenPredicate(T, AnySubjectGivenPredicate<T>),
}

impl<T> AnySubject<T> {
	pub fn from_option_quad(p: Option<T>, o: Option<T>, g: Option<Option<T>>) -> Self {
		match p {
			Some(p) => Self::GivenPredicate(p, AnySubjectGivenPredicate::from_option(o, g)),
			None => Self::AnyPredicate(AnySubjectAnyPredicate::from_option(o, g)),
		}
	}

	pub fn from_pattern<X: PartialEq>(
		s: X,
		p: ResourceOrVar<T, X>,
		o: ResourceOrVar<T, X>,
		g: Option<ResourceOrVar<T, X>>,
	) -> Self {
		match p {
			ResourceOrVar::Resource(p) => {
				Self::GivenPredicate(p, AnySubjectGivenPredicate::from_pattern(s, o, g))
			}
			ResourceOrVar::Var(p) => {
				if p == s {
					Self::SameAsSubject(AnySubjectGivenPredicate::from_pattern(s, o, g))
				} else {
					Self::AnyPredicate(AnySubjectAnyPredicate::from_pattern(s, p, o, g))
				}
			}
		}
	}

	pub fn predicate(&self) -> PatternPredicate<&T> {
		match self {
			Self::AnyPredicate(_) => PatternPredicate::Any,
			Self::SameAsSubject(_) => PatternPredicate::SameAsSubject,
			Self::GivenPredicate(id, _) => PatternPredicate::Given(id),
		}
	}

	pub fn into_predicate(self) -> PatternPredicate<T> {
		match self {
			Self::AnyPredicate(_) => PatternPredicate::Any,
			Self::SameAsSubject(_) => PatternPredicate::SameAsSubject,
			Self::GivenPredicate(id, _) => PatternPredicate::Given(id),
		}
	}

	pub fn object(&self) -> PatternObject<&T> {
		match self {
			Self::AnyPredicate(t) => t.object(),
			Self::SameAsSubject(t) => t.object(),
			Self::GivenPredicate(_, t) => t.object(),
		}
	}

	pub fn into_object(self) -> PatternObject<T> {
		match self {
			Self::AnyPredicate(t) => t.into_object(),
			Self::SameAsSubject(t) => t.into_object(),
			Self::GivenPredicate(_, t) => t.into_object(),
		}
	}

	pub fn graph(&self) -> PatternGraph<&T> {
		match self {
			Self::AnyPredicate(t) => t.graph(),
			Self::SameAsSubject(t) => t.graph(),
			Self::GivenPredicate(_, t) => t.graph(),
		}
	}

	pub fn into_graph(self) -> PatternGraph<T> {
		match self {
			Self::AnyPredicate(t) => t.into_graph(),
			Self::SameAsSubject(t) => t.into_graph(),
			Self::GivenPredicate(_, t) => t.into_graph(),
		}
	}

	pub fn into_triple(self) -> (triple::canonical::AnySubject<T>, PatternGraph<T>) {
		match self {
			Self::AnyPredicate(t) => {
				let (u, g) = t.into_triple();
				(triple::canonical::AnySubject::AnyPredicate(u), g)
			}
			Self::SameAsSubject(t) => {
				let (u, g) = t.into_triple();
				(triple::canonical::AnySubject::SameAsSubject(u), g)
			}
			Self::GivenPredicate(id, t) => {
				let (u, g) = t.into_triple();
				(triple::canonical::AnySubject::GivenPredicate(id, u), g)
			}
		}
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum AnySubjectAnyPredicate<T> {
	AnyObject(AnySubjectAnyPredicateAnyObject<T>),
	SameAsSubject(AnySubjectAnyPredicateGivenObject<T>),
	SameAsPredicate(AnySubjectAnyPredicateGivenObject<T>),
	GivenObject(T, AnySubjectAnyPredicateGivenObject<T>),
}

impl<T> AnySubjectAnyPredicate<T> {
	pub fn from_option(o: Option<T>, g: Option<Option<T>>) -> Self {
		match o {
			Some(o) => Self::GivenObject(o, AnySubjectAnyPredicateGivenObject::from_option(g)),
			None => Self::AnyObject(AnySubjectAnyPredicateAnyObject::from_option(g)),
		}
	}

	pub fn from_pattern<X: PartialEq>(
		s: X,
		p: X,
		o: ResourceOrVar<T, X>,
		g: Option<ResourceOrVar<T, X>>,
	) -> Self {
		match o {
			ResourceOrVar::Resource(o) => {
				Self::GivenObject(o, AnySubjectAnyPredicateGivenObject::from_pattern(s, p, g))
			}
			ResourceOrVar::Var(o) => {
				if o == s {
					Self::SameAsSubject(AnySubjectAnyPredicateGivenObject::from_pattern(s, p, g))
				} else if o == p {
					Self::SameAsPredicate(AnySubjectAnyPredicateGivenObject::from_pattern(s, p, g))
				} else {
					Self::AnyObject(AnySubjectAnyPredicateAnyObject::from_pattern(s, p, o, g))
				}
			}
		}
	}

	pub fn object(&self) -> PatternObject<&T> {
		match self {
			Self::AnyObject(_) => PatternObject::Any,
			Self::SameAsSubject(_) => PatternObject::SameAsSubject,
			Self::SameAsPredicate(_) => PatternObject::SameAsPredicate,
			Self::GivenObject(id, _) => PatternObject::Given(id),
		}
	}

	pub fn into_object(self) -> PatternObject<T> {
		match self {
			Self::AnyObject(_) => PatternObject::Any,
			Self::SameAsSubject(_) => PatternObject::SameAsSubject,
			Self::SameAsPredicate(_) => PatternObject::SameAsPredicate,
			Self::GivenObject(id, _) => PatternObject::Given(id),
		}
	}

	pub fn graph(&self) -> PatternGraph<&T> {
		match self {
			Self::AnyObject(g) => g.graph(),
			Self::SameAsSubject(g) => g.graph(),
			Self::SameAsPredicate(g) => g.graph(),
			Self::GivenObject(_, g) => g.graph(),
		}
	}

	pub fn into_graph(self) -> PatternGraph<T> {
		match self {
			Self::AnyObject(g) => g.into_graph(),
			Self::SameAsSubject(g) => g.into_graph(),
			Self::SameAsPredicate(g) => g.into_graph(),
			Self::GivenObject(_, g) => g.into_graph(),
		}
	}

	pub fn into_triple(
		self,
	) -> (
		triple::canonical::AnySubjectAnyPredicate<T>,
		PatternGraph<T>,
	) {
		match self {
			Self::AnyObject(t) => (
				triple::canonical::AnySubjectAnyPredicate::AnyObject,
				t.into_graph(),
			),
			Self::SameAsSubject(t) => (
				triple::canonical::AnySubjectAnyPredicate::SameAsSubject,
				t.into_graph(),
			),
			Self::SameAsPredicate(t) => (
				triple::canonical::AnySubjectAnyPredicate::SameAsPredicate,
				t.into_graph(),
			),
			Self::GivenObject(id, t) => (
				triple::canonical::AnySubjectAnyPredicate::GivenObject(id),
				t.into_graph(),
			),
		}
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum AnySubjectGivenPredicate<T> {
	AnyObject(AnySubjectGivenPredicateAnyObject<T>),
	SameAsSubject(AnySubjectGivenPredicateGivenObject<T>),
	GivenObject(T, AnySubjectGivenPredicateGivenObject<T>),
}

impl<T> AnySubjectGivenPredicate<T> {
	pub fn from_option(o: Option<T>, g: Option<Option<T>>) -> Self {
		match o {
			Some(o) => Self::GivenObject(o, AnySubjectGivenPredicateGivenObject::from_option(g)),
			None => Self::AnyObject(AnySubjectGivenPredicateAnyObject::from_option(g)),
		}
	}

	pub fn from_pattern<X: PartialEq>(
		s: X,
		o: ResourceOrVar<T, X>,
		g: Option<ResourceOrVar<T, X>>,
	) -> Self {
		match o {
			ResourceOrVar::Resource(o) => {
				Self::GivenObject(o, AnySubjectGivenPredicateGivenObject::from_pattern(s, g))
			}
			ResourceOrVar::Var(o) => {
				if o == s {
					Self::SameAsSubject(AnySubjectGivenPredicateGivenObject::from_pattern(s, g))
				} else {
					Self::AnyObject(AnySubjectGivenPredicateAnyObject::from_pattern(s, o, g))
				}
			}
		}
	}

	pub fn object(&self) -> PatternObject<&T> {
		match self {
			Self::AnyObject(_) => PatternObject::Any,
			Self::SameAsSubject(_) => PatternObject::SameAsSubject,
			Self::GivenObject(id, _) => PatternObject::Given(id),
		}
	}

	pub fn into_object(self) -> PatternObject<T> {
		match self {
			Self::AnyObject(_) => PatternObject::Any,
			Self::SameAsSubject(_) => PatternObject::SameAsSubject,
			Self::GivenObject(id, _) => PatternObject::Given(id),
		}
	}

	pub fn graph(&self) -> PatternGraph<&T> {
		match self {
			Self::AnyObject(g) => g.graph(),
			Self::SameAsSubject(g) => g.graph(),
			Self::GivenObject(_, g) => g.graph(),
		}
	}

	pub fn into_graph(self) -> PatternGraph<T> {
		match self {
			Self::AnyObject(g) => g.into_graph(),
			Self::SameAsSubject(g) => g.into_graph(),
			Self::GivenObject(_, g) => g.into_graph(),
		}
	}

	pub fn into_triple(
		self,
	) -> (
		triple::canonical::AnySubjectGivenPredicate<T>,
		PatternGraph<T>,
	) {
		match self {
			Self::AnyObject(t) => (
				triple::canonical::AnySubjectGivenPredicate::AnyObject,
				t.into_graph(),
			),
			Self::SameAsSubject(t) => (
				triple::canonical::AnySubjectGivenPredicate::SameAsSubject,
				t.into_graph(),
			),
			Self::GivenObject(id, t) => (
				triple::canonical::AnySubjectGivenPredicate::GivenObject(id),
				t.into_graph(),
			),
		}
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum GivenSubject<T> {
	AnyPredicate(GivenSubjectAnyPredicate<T>),
	GivenPredicate(T, GivenSubjectGivenPredicate<T>),
}

impl<T> GivenSubject<T> {
	pub fn from_option_quad(p: Option<T>, o: Option<T>, g: Option<Option<T>>) -> Self {
		match p {
			Some(p) => Self::GivenPredicate(p, GivenSubjectGivenPredicate::from_option(o, g)),
			None => Self::AnyPredicate(GivenSubjectAnyPredicate::from_option(o, g)),
		}
	}

	pub fn from_pattern<X: PartialEq>(
		p: ResourceOrVar<T, X>,
		o: ResourceOrVar<T, X>,
		g: Option<ResourceOrVar<T, X>>,
	) -> Self {
		match p {
			ResourceOrVar::Resource(p) => {
				Self::GivenPredicate(p, GivenSubjectGivenPredicate::from_pattern(o, g))
			}
			ResourceOrVar::Var(p) => {
				Self::AnyPredicate(GivenSubjectAnyPredicate::from_pattern(p, o, g))
			}
		}
	}

	pub fn predicate(&self) -> PatternPredicate<&T> {
		match self {
			Self::AnyPredicate(_) => PatternPredicate::Any,
			Self::GivenPredicate(id, _) => PatternPredicate::Given(id),
		}
	}

	pub fn into_predicate(self) -> PatternPredicate<T> {
		match self {
			Self::AnyPredicate(_) => PatternPredicate::Any,
			Self::GivenPredicate(id, _) => PatternPredicate::Given(id),
		}
	}

	pub fn object(&self) -> PatternObject<&T> {
		match self {
			Self::AnyPredicate(t) => t.object(),
			Self::GivenPredicate(_, t) => t.object(),
		}
	}

	pub fn into_object(self) -> PatternObject<T> {
		match self {
			Self::AnyPredicate(t) => t.into_object(),
			Self::GivenPredicate(_, t) => t.into_object(),
		}
	}

	pub fn graph(&self) -> PatternGraph<&T> {
		match self {
			Self::AnyPredicate(t) => t.graph(),
			Self::GivenPredicate(_, t) => t.graph(),
		}
	}

	pub fn into_graph(self) -> PatternGraph<T> {
		match self {
			Self::AnyPredicate(t) => t.into_graph(),
			Self::GivenPredicate(_, t) => t.into_graph(),
		}
	}

	pub fn into_triple(self) -> (triple::canonical::GivenSubject<T>, PatternGraph<T>) {
		match self {
			Self::AnyPredicate(t) => {
				let (u, g) = t.into_triple();
				(triple::canonical::GivenSubject::AnyPredicate(u), g)
			}
			Self::GivenPredicate(id, t) => {
				let (u, g) = t.into_triple();
				(triple::canonical::GivenSubject::GivenPredicate(id, u), g)
			}
		}
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum GivenSubjectAnyPredicate<T> {
	AnyObject(GivenSubjectAnyPredicateAnyObject<T>),
	SameAsPredicate(GivenSubjectAnyPredicateGivenObject<T>),
	GivenObject(T, GivenSubjectAnyPredicateGivenObject<T>),
}

impl<T> GivenSubjectAnyPredicate<T> {
	pub fn from_option(o: Option<T>, g: Option<Option<T>>) -> Self {
		match o {
			Some(o) => Self::GivenObject(o, GivenSubjectAnyPredicateGivenObject::from_option(g)),
			None => Self::AnyObject(GivenSubjectAnyPredicateAnyObject::from_option(g)),
		}
	}

	pub fn from_pattern<X: PartialEq>(
		p: X,
		o: ResourceOrVar<T, X>,
		g: Option<ResourceOrVar<T, X>>,
	) -> Self {
		match o {
			ResourceOrVar::Resource(o) => {
				Self::GivenObject(o, GivenSubjectAnyPredicateGivenObject::from_pattern(p, g))
			}
			ResourceOrVar::Var(o) => {
				if p == o {
					Self::SameAsPredicate(GivenSubjectAnyPredicateGivenObject::from_pattern(p, g))
				} else {
					Self::AnyObject(GivenSubjectAnyPredicateAnyObject::from_pattern(p, o, g))
				}
			}
		}
	}

	pub fn object(&self) -> PatternObject<&T> {
		match self {
			Self::AnyObject(_) => PatternObject::Any,
			Self::SameAsPredicate(_) => PatternObject::SameAsPredicate,
			Self::GivenObject(id, _) => PatternObject::Given(id),
		}
	}

	pub fn into_object(self) -> PatternObject<T> {
		match self {
			Self::AnyObject(_) => PatternObject::Any,
			Self::SameAsPredicate(_) => PatternObject::SameAsPredicate,
			Self::GivenObject(id, _) => PatternObject::Given(id),
		}
	}

	pub fn graph(&self) -> PatternGraph<&T> {
		match self {
			Self::AnyObject(t) => t.graph(),
			Self::SameAsPredicate(t) => t.graph(),
			Self::GivenObject(_, t) => t.graph(),
		}
	}

	pub fn into_graph(self) -> PatternGraph<T> {
		match self {
			Self::AnyObject(t) => t.into_graph(),
			Self::SameAsPredicate(t) => t.into_graph(),
			Self::GivenObject(_, t) => t.into_graph(),
		}
	}

	pub fn into_triple(
		self,
	) -> (
		triple::canonical::GivenSubjectAnyPredicate<T>,
		PatternGraph<T>,
	) {
		match self {
			Self::AnyObject(t) => (
				triple::canonical::GivenSubjectAnyPredicate::AnyObject,
				t.into_graph(),
			),
			Self::SameAsPredicate(t) => (
				triple::canonical::GivenSubjectAnyPredicate::SameAsPredicate,
				t.into_graph(),
			),
			Self::GivenObject(id, t) => (
				triple::canonical::GivenSubjectAnyPredicate::GivenObject(id),
				t.into_graph(),
			),
		}
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum GivenSubjectGivenPredicate<T> {
	AnyObject(GivenSubjectGivenPredicateAnyObject<T>),
	GivenObject(T, GivenSubjectGivenPredicateGivenObject<T>),
}

impl<T> GivenSubjectGivenPredicate<T> {
	pub fn from_option(o: Option<T>, g: Option<Option<T>>) -> Self {
		match o {
			Some(o) => Self::GivenObject(o, GivenSubjectGivenPredicateGivenObject::from_option(g)),
			None => Self::AnyObject(GivenSubjectGivenPredicateAnyObject::from_option(g)),
		}
	}

	pub fn from_pattern<X: PartialEq>(
		o: ResourceOrVar<T, X>,
		g: Option<ResourceOrVar<T, X>>,
	) -> Self {
		match o {
			ResourceOrVar::Resource(o) => {
				Self::GivenObject(o, GivenSubjectGivenPredicateGivenObject::from_pattern(g))
			}
			ResourceOrVar::Var(o) => {
				Self::AnyObject(GivenSubjectGivenPredicateAnyObject::from_pattern(o, g))
			}
		}
	}

	pub fn object(&self) -> PatternObject<&T> {
		match self {
			Self::AnyObject(_) => PatternObject::Any,
			Self::GivenObject(id, _) => PatternObject::Given(id),
		}
	}

	pub fn into_object(self) -> PatternObject<T> {
		match self {
			Self::AnyObject(_) => PatternObject::Any,
			Self::GivenObject(id, _) => PatternObject::Given(id),
		}
	}

	pub fn graph(&self) -> PatternGraph<&T> {
		match self {
			Self::AnyObject(t) => t.graph(),
			Self::GivenObject(_, t) => t.graph(),
		}
	}

	pub fn into_graph(self) -> PatternGraph<T> {
		match self {
			Self::AnyObject(t) => t.into_graph(),
			Self::GivenObject(_, t) => t.into_graph(),
		}
	}

	pub fn into_triple(
		self,
	) -> (
		triple::canonical::GivenSubjectGivenPredicate<T>,
		PatternGraph<T>,
	) {
		match self {
			Self::AnyObject(t) => (
				triple::canonical::GivenSubjectGivenPredicate::AnyObject,
				t.into_graph(),
			),
			Self::GivenObject(id, t) => (
				triple::canonical::GivenSubjectGivenPredicate::GivenObject(id),
				t.into_graph(),
			),
		}
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum PatternGraph<T> {
	Any,
	SameAsSubject,
	SameAsPredicate,
	SameAsObject,
	Given(Option<T>),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum AnySubjectAnyPredicateAnyObject<T> {
	AnyGraph,
	SameAsSubject,
	SameAsPredicate,
	SameAsObject,
	GivenGraph(Option<T>),
}

impl<T> AnySubjectAnyPredicateAnyObject<T> {
	pub fn from_option(g: Option<Option<T>>) -> Self {
		match g {
			Some(g) => Self::GivenGraph(g),
			None => Self::AnyGraph,
		}
	}

	pub fn from_pattern<X: PartialEq>(s: X, p: X, o: X, g: Option<ResourceOrVar<T, X>>) -> Self {
		match g {
			None => Self::GivenGraph(None),
			Some(ResourceOrVar::Resource(g)) => Self::GivenGraph(Some(g)),
			Some(ResourceOrVar::Var(g)) => {
				if g == s {
					Self::SameAsSubject
				} else if g == p {
					Self::SameAsPredicate
				} else if g == o {
					Self::SameAsObject
				} else {
					Self::AnyGraph
				}
			}
		}
	}

	pub fn graph(&self) -> PatternGraph<&T> {
		match self {
			Self::AnyGraph => PatternGraph::Any,
			Self::SameAsSubject => PatternGraph::SameAsSubject,
			Self::SameAsPredicate => PatternGraph::SameAsPredicate,
			Self::SameAsObject => PatternGraph::SameAsObject,
			Self::GivenGraph(g) => PatternGraph::Given(g.as_ref()),
		}
	}

	pub fn into_graph(self) -> PatternGraph<T> {
		match self {
			Self::AnyGraph => PatternGraph::Any,
			Self::SameAsSubject => PatternGraph::SameAsSubject,
			Self::SameAsPredicate => PatternGraph::SameAsPredicate,
			Self::SameAsObject => PatternGraph::SameAsObject,
			Self::GivenGraph(g) => PatternGraph::Given(g),
		}
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum GivenSubjectAnyPredicateAnyObject<T> {
	AnyGraph,
	SameAsPredicate,
	SameAsObject,
	GivenGraph(Option<T>),
}

impl<T> GivenSubjectAnyPredicateAnyObject<T> {
	pub fn from_option(g: Option<Option<T>>) -> Self {
		match g {
			Some(g) => Self::GivenGraph(g),
			None => Self::AnyGraph,
		}
	}

	pub fn from_pattern<X: PartialEq>(p: X, o: X, g: Option<ResourceOrVar<T, X>>) -> Self {
		match g {
			None => Self::GivenGraph(None),
			Some(ResourceOrVar::Resource(g)) => Self::GivenGraph(Some(g)),
			Some(ResourceOrVar::Var(g)) => {
				if g == p {
					Self::SameAsPredicate
				} else if g == o {
					Self::SameAsObject
				} else {
					Self::AnyGraph
				}
			}
		}
	}

	pub fn graph(&self) -> PatternGraph<&T> {
		match self {
			Self::AnyGraph => PatternGraph::Any,
			Self::SameAsPredicate => PatternGraph::SameAsPredicate,
			Self::SameAsObject => PatternGraph::SameAsObject,
			Self::GivenGraph(g) => PatternGraph::Given(g.as_ref()),
		}
	}

	pub fn into_graph(self) -> PatternGraph<T> {
		match self {
			Self::AnyGraph => PatternGraph::Any,
			Self::SameAsPredicate => PatternGraph::SameAsPredicate,
			Self::SameAsObject => PatternGraph::SameAsObject,
			Self::GivenGraph(g) => PatternGraph::Given(g),
		}
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum AnySubjectGivenPredicateAnyObject<T> {
	AnyGraph,
	SameAsSubject,
	SameAsObject,
	GivenGraph(Option<T>),
}

impl<T> AnySubjectGivenPredicateAnyObject<T> {
	pub fn from_option(g: Option<Option<T>>) -> Self {
		match g {
			Some(g) => Self::GivenGraph(g),
			None => Self::AnyGraph,
		}
	}

	pub fn from_pattern<X: PartialEq>(s: X, o: X, g: Option<ResourceOrVar<T, X>>) -> Self {
		match g {
			None => Self::GivenGraph(None),
			Some(ResourceOrVar::Resource(g)) => Self::GivenGraph(Some(g)),
			Some(ResourceOrVar::Var(g)) => {
				if g == s {
					Self::SameAsSubject
				} else if g == o {
					Self::SameAsObject
				} else {
					Self::AnyGraph
				}
			}
		}
	}

	pub fn graph(&self) -> PatternGraph<&T> {
		match self {
			Self::AnyGraph => PatternGraph::Any,
			Self::SameAsSubject => PatternGraph::SameAsSubject,
			Self::SameAsObject => PatternGraph::SameAsObject,
			Self::GivenGraph(g) => PatternGraph::Given(g.as_ref()),
		}
	}

	pub fn into_graph(self) -> PatternGraph<T> {
		match self {
			Self::AnyGraph => PatternGraph::Any,
			Self::SameAsSubject => PatternGraph::SameAsSubject,
			Self::SameAsObject => PatternGraph::SameAsObject,
			Self::GivenGraph(g) => PatternGraph::Given(g),
		}
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum AnySubjectAnyPredicateGivenObject<T> {
	AnyGraph,
	SameAsSubject,
	SameAsPredicate,
	GivenGraph(Option<T>),
}

impl<T> AnySubjectAnyPredicateGivenObject<T> {
	pub fn from_option(g: Option<Option<T>>) -> Self {
		match g {
			Some(g) => Self::GivenGraph(g),
			None => Self::AnyGraph,
		}
	}

	pub fn from_pattern<X: PartialEq>(s: X, p: X, g: Option<ResourceOrVar<T, X>>) -> Self {
		match g {
			None => Self::GivenGraph(None),
			Some(ResourceOrVar::Resource(g)) => Self::GivenGraph(Some(g)),
			Some(ResourceOrVar::Var(g)) => {
				if g == s {
					Self::SameAsSubject
				} else if g == p {
					Self::SameAsPredicate
				} else {
					Self::AnyGraph
				}
			}
		}
	}

	pub fn graph(&self) -> PatternGraph<&T> {
		match self {
			Self::AnyGraph => PatternGraph::Any,
			Self::SameAsSubject => PatternGraph::SameAsSubject,
			Self::SameAsPredicate => PatternGraph::SameAsPredicate,
			Self::GivenGraph(g) => PatternGraph::Given(g.as_ref()),
		}
	}

	pub fn into_graph(self) -> PatternGraph<T> {
		match self {
			Self::AnyGraph => PatternGraph::Any,
			Self::SameAsSubject => PatternGraph::SameAsSubject,
			Self::SameAsPredicate => PatternGraph::SameAsPredicate,
			Self::GivenGraph(g) => PatternGraph::Given(g),
		}
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum AnySubjectGivenPredicateGivenObject<T> {
	AnyGraph,
	SameAsSubject,
	GivenGraph(Option<T>),
}

impl<T> AnySubjectGivenPredicateGivenObject<T> {
	pub fn from_option(g: Option<Option<T>>) -> Self {
		match g {
			Some(g) => Self::GivenGraph(g),
			None => Self::AnyGraph,
		}
	}

	pub fn from_pattern<X: PartialEq>(s: X, g: Option<ResourceOrVar<T, X>>) -> Self {
		match g {
			None => Self::GivenGraph(None),
			Some(ResourceOrVar::Resource(g)) => Self::GivenGraph(Some(g)),
			Some(ResourceOrVar::Var(g)) => {
				if g == s {
					Self::SameAsSubject
				} else {
					Self::AnyGraph
				}
			}
		}
	}

	pub fn graph(&self) -> PatternGraph<&T> {
		match self {
			Self::AnyGraph => PatternGraph::Any,
			Self::SameAsSubject => PatternGraph::SameAsSubject,
			Self::GivenGraph(g) => PatternGraph::Given(g.as_ref()),
		}
	}

	pub fn into_graph(self) -> PatternGraph<T> {
		match self {
			Self::AnyGraph => PatternGraph::Any,
			Self::SameAsSubject => PatternGraph::SameAsSubject,
			Self::GivenGraph(g) => PatternGraph::Given(g),
		}
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum GivenSubjectAnyPredicateGivenObject<T> {
	AnyGraph,
	SameAsPredicate,
	GivenGraph(Option<T>),
}

impl<T> GivenSubjectAnyPredicateGivenObject<T> {
	pub fn from_option(g: Option<Option<T>>) -> Self {
		match g {
			Some(g) => Self::GivenGraph(g),
			None => Self::AnyGraph,
		}
	}

	pub fn from_pattern<X: PartialEq>(p: X, g: Option<ResourceOrVar<T, X>>) -> Self {
		match g {
			None => Self::GivenGraph(None),
			Some(ResourceOrVar::Resource(g)) => Self::GivenGraph(Some(g)),
			Some(ResourceOrVar::Var(g)) => {
				if g == p {
					Self::SameAsPredicate
				} else {
					Self::AnyGraph
				}
			}
		}
	}

	pub fn graph(&self) -> PatternGraph<&T> {
		match self {
			Self::AnyGraph => PatternGraph::Any,
			Self::SameAsPredicate => PatternGraph::SameAsPredicate,
			Self::GivenGraph(g) => PatternGraph::Given(g.as_ref()),
		}
	}

	pub fn into_graph(self) -> PatternGraph<T> {
		match self {
			Self::AnyGraph => PatternGraph::Any,
			Self::SameAsPredicate => PatternGraph::SameAsPredicate,
			Self::GivenGraph(g) => PatternGraph::Given(g),
		}
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum GivenSubjectGivenPredicateAnyObject<T> {
	AnyGraph,
	SameAsObject,
	GivenGraph(Option<T>),
}

impl<T> GivenSubjectGivenPredicateAnyObject<T> {
	pub fn from_option(g: Option<Option<T>>) -> Self {
		match g {
			Some(g) => Self::GivenGraph(g),
			None => Self::AnyGraph,
		}
	}

	pub fn from_pattern<X: PartialEq>(o: X, g: Option<ResourceOrVar<T, X>>) -> Self {
		match g {
			None => Self::GivenGraph(None),
			Some(ResourceOrVar::Resource(g)) => Self::GivenGraph(Some(g)),
			Some(ResourceOrVar::Var(g)) => {
				if g == o {
					Self::SameAsObject
				} else {
					Self::AnyGraph
				}
			}
		}
	}

	pub fn graph(&self) -> PatternGraph<&T> {
		match self {
			Self::AnyGraph => PatternGraph::Any,
			Self::SameAsObject => PatternGraph::SameAsObject,
			Self::GivenGraph(g) => PatternGraph::Given(g.as_ref()),
		}
	}

	pub fn into_graph(self) -> PatternGraph<T> {
		match self {
			Self::AnyGraph => PatternGraph::Any,
			Self::SameAsObject => PatternGraph::SameAsObject,
			Self::GivenGraph(g) => PatternGraph::Given(g),
		}
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum GivenSubjectGivenPredicateGivenObject<T> {
	AnyGraph,
	GivenGraph(Option<T>),
}

impl<T> GivenSubjectGivenPredicateGivenObject<T> {
	pub fn from_option(g: Option<Option<T>>) -> Self {
		match g {
			Some(g) => Self::GivenGraph(g),
			None => Self::AnyGraph,
		}
	}

	pub fn from_pattern<X>(g: Option<ResourceOrVar<T, X>>) -> Self {
		match g {
			None => Self::GivenGraph(None),
			Some(ResourceOrVar::Resource(g)) => Self::GivenGraph(Some(g)),
			Some(ResourceOrVar::Var(_)) => Self::AnyGraph,
		}
	}

	pub fn graph(&self) -> PatternGraph<&T> {
		match self {
			Self::AnyGraph => PatternGraph::Any,
			Self::GivenGraph(g) => PatternGraph::Given(g.as_ref()),
		}
	}

	pub fn into_graph(self) -> PatternGraph<T> {
		match self {
			Self::AnyGraph => PatternGraph::Any,
			Self::GivenGraph(g) => PatternGraph::Given(g),
		}
	}
}
