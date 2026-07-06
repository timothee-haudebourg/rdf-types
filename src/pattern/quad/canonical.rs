use std::borrow::Cow;

use replace_with::replace_with_or_abort_and_return;

use crate::{
	Quad, Triple,
	pattern::{
		Pattern, QuadPattern, TriplePattern,
		triple::{self, CanonicalTriplePattern},
	},
};

/// Canonical triple pattern.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum CanonicalQuadPattern<T> {
	AnySubject(AnySubject<T>),
	GivenSubject(T, GivenSubject<T>),
}

impl<T> CanonicalQuadPattern<T> {
	pub const ANY: Self = Self::AnySubject(AnySubject::AnyPredicate(
		AnySubjectAnyPredicate::AnyObject(AnySubjectAnyPredicateAnyObject::AnyGraph),
	));

	pub fn from_graph(graph: Option<T>) -> Self {
		Self::AnySubject(AnySubject::AnyPredicate(AnySubjectAnyPredicate::AnyObject(
			AnySubjectAnyPredicateAnyObject::GivenGraph(graph),
		)))
	}

	pub fn matches(&self, quad: Quad<T>) -> bool
	where
		T: PartialEq,
	{
		match self {
			Self::AnySubject(p) => p.matches(quad),
			Self::GivenSubject(s, p) => quad.subject() == s && p.matches(quad),
		}
	}

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
			Pattern::Ground(s) => {
				Self::GivenSubject(s, GivenSubject::from_pattern(pattern.1, pattern.2, None))
			}
			Pattern::Var(s) => {
				Self::AnySubject(AnySubject::from_pattern(s, pattern.1, pattern.2, None))
			}
		}
	}

	pub fn from_pattern<X: PartialEq>(pattern: QuadPattern<T, X>) -> Self {
		match pattern.0 {
			Pattern::Ground(s) => Self::GivenSubject(
				s,
				GivenSubject::from_pattern(pattern.1, pattern.2, pattern.3),
			),
			Pattern::Var(s) => {
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

	pub fn into_parts(
		self,
	) -> (
		PatternSubject<T>,
		PatternPredicate<T>,
		PatternObject<T>,
		PatternGraph<T>,
	) {
		match self {
			Self::AnySubject(pog) => {
				let (p, o, g) = pog.into_parts();
				(PatternSubject::Any, p, o, g)
			}
			Self::GivenSubject(s, pog) => {
				let (p, o, g) = pog.into_parts();
				(PatternSubject::Given(s), p, o, g)
			}
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

	pub fn as_ref(&self) -> CanonicalQuadPattern<&T> {
		match self {
			Self::AnySubject(pog) => CanonicalQuadPattern::AnySubject(pog.as_ref()),
			Self::GivenSubject(s, pog) => CanonicalQuadPattern::GivenSubject(s, pog.as_ref()),
		}
	}

	pub fn map<U>(self, mut f: impl FnMut(T) -> U) -> CanonicalQuadPattern<U> {
		match self {
			Self::AnySubject(pog) => CanonicalQuadPattern::AnySubject(pog.map(f)),
			Self::GivenSubject(s, pog) => CanonicalQuadPattern::GivenSubject(f(s), pog.map(f)),
		}
	}

	pub fn map2<U, V>(
		self,
		mut f: impl FnMut(T) -> (U, V),
	) -> (CanonicalQuadPattern<U>, CanonicalQuadPattern<V>) {
		match self {
			Self::AnySubject(pog) => {
				let (pog_u, pog_v) = pog.map2(f);
				(
					CanonicalQuadPattern::AnySubject(pog_u),
					CanonicalQuadPattern::AnySubject(pog_v),
				)
			}
			Self::GivenSubject(s, pog) => {
				let (u, v) = f(s);
				let (pog_u, pog_v) = pog.map2(f);
				(
					CanonicalQuadPattern::GivenSubject(u, pog_u),
					CanonicalQuadPattern::GivenSubject(v, pog_v),
				)
			}
		}
	}

	pub fn set_subject(&mut self, s: T) -> PatternSubject<T>
	where
		T: Clone,
	{
		replace_with_or_abort_and_return(self, |this| match this {
			Self::AnySubject(p) => (
				PatternSubject::Any,
				Self::GivenSubject(s.clone(), p.set_subject(s)),
			),
			Self::GivenSubject(old_s, p) => {
				(PatternSubject::Given(old_s), Self::GivenSubject(s, p))
			}
		})
	}

	pub fn with_subject(mut self, s: T) -> Self
	where
		T: Clone,
	{
		self.set_subject(s);
		self
	}

	pub fn set_predicate(&mut self, p: T) -> PatternPredicate<T>
	where
		T: Clone,
	{
		replace_with_or_abort_and_return(self, |this| match this {
			Self::AnySubject(AnySubject::SameAsSubject(rest)) => (
				PatternPredicate::SameAsSubject,
				Self::GivenSubject(
					p.clone(),
					GivenSubject::GivenPredicate(p.clone(), rest.set_subject(p)),
				),
			),
			Self::AnySubject(mut current) => {
				let old_p = current.set_predicate(p);
				(old_p, Self::AnySubject(current))
			}
			Self::GivenSubject(s, mut current) => {
				let old_p = current.set_predicate(p);
				(old_p, Self::GivenSubject(s, current))
			}
		})
	}

	pub fn with_predicate(mut self, p: T) -> Self
	where
		T: Clone,
	{
		self.set_predicate(p);
		self
	}

	pub fn set_object(&mut self, o: T) -> PatternObject<T>
	where
		T: Clone,
	{
		replace_with_or_abort_and_return(self, |this| match this {
			Self::AnySubject(AnySubject::AnyPredicate(AnySubjectAnyPredicate::SameAsSubject(
				g,
			))) => (
				PatternObject::SameAsSubject,
				Self::GivenSubject(
					o.clone(),
					GivenSubject::AnyPredicate(GivenSubjectAnyPredicate::GivenObject(
						o.clone(),
						g.set_subject(o),
					)),
				),
			),
			Self::AnySubject(AnySubject::SameAsSubject(
				AnySubjectGivenPredicate::SameAsSubject(g),
			)) => (
				PatternObject::SameAsSubject,
				Self::GivenSubject(
					o.clone(),
					GivenSubject::GivenPredicate(
						o.clone(),
						GivenSubjectGivenPredicate::GivenObject(o.clone(), g.set_subject(o)),
					),
				),
			),
			Self::AnySubject(mut current) => {
				let old_o = current.set_object(o);
				(old_o, Self::AnySubject(current))
			}
			Self::GivenSubject(s, mut current) => {
				let old_o = current.set_object(o);
				(old_o, Self::GivenSubject(s, current))
			}
		})
	}

	pub fn with_object(mut self, o: T) -> Self
	where
		T: Clone,
	{
		self.set_object(o);
		self
	}

	pub fn set_graph(&mut self, g: Option<T>) -> PatternGraph<T> {
		match self {
			Self::AnySubject(rest) => rest.set_graph(g),
			Self::GivenSubject(_, rest) => rest.set_graph(g),
		}
	}

	pub fn with_graph(mut self, g: Option<T>) -> Self {
		self.set_graph(g);
		self
	}
}

impl<T> From<Triple<T>> for CanonicalQuadPattern<T> {
	fn from(value: Triple<T>) -> Self {
		Self::from_triple(value)
	}
}

impl<'a, T: ToOwned> From<Triple<&'a T>> for CanonicalQuadPattern<Cow<'a, T>> {
	fn from(value: Triple<&'a T>) -> Self {
		Self::from_triple(value.map(Cow::Borrowed))
	}
}

impl<T> From<Quad<T>> for CanonicalQuadPattern<T> {
	fn from(value: Quad<T>) -> Self {
		Self::from_quad(value)
	}
}

impl<'a, T: ToOwned> From<Quad<&'a T>> for CanonicalQuadPattern<Cow<'a, T>> {
	fn from(value: Quad<&'a T>) -> Self {
		Self::from_quad(value.map(Cow::Borrowed))
	}
}

impl<T> From<Triple<Option<T>>> for CanonicalQuadPattern<T> {
	fn from(value: Triple<Option<T>>) -> Self {
		Self::from_option_triple(value)
	}
}

impl<'a, T: ToOwned> From<Triple<Option<&'a T>>> for CanonicalQuadPattern<Cow<'a, T>> {
	fn from(value: Triple<Option<&'a T>>) -> Self {
		Self::from_option_triple(value.map(|r| r.map(Cow::Borrowed)))
	}
}

impl<T> From<Quad<Option<T>>> for CanonicalQuadPattern<T> {
	fn from(value: Quad<Option<T>>) -> Self {
		Self::from_option_quad(value)
	}
}

impl<'a, T: ToOwned> From<Quad<Option<&'a T>>> for CanonicalQuadPattern<Cow<'a, T>> {
	fn from(value: Quad<Option<&'a T>>) -> Self {
		Self::from_option_quad(value.map(|r| r.map(Cow::Borrowed)))
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

impl<'a, T: ToOwned> From<CanonicalQuadPattern<&'a T>> for CanonicalQuadPattern<Cow<'a, T>> {
	fn from(value: CanonicalQuadPattern<&'a T>) -> Self {
		value.map(Cow::Borrowed)
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

impl<T> PatternSubject<&T> {
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

impl<T> PatternPredicate<&T> {
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

impl<T> PatternObject<&T> {
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
		p: Pattern<T, X>,
		o: Pattern<T, X>,
		g: Option<Pattern<T, X>>,
	) -> Self {
		match p {
			Pattern::Ground(p) => {
				Self::GivenPredicate(p, AnySubjectGivenPredicate::from_pattern(s, o, g))
			}
			Pattern::Var(p) => {
				if p == s {
					Self::SameAsSubject(AnySubjectGivenPredicate::from_pattern(s, o, g))
				} else {
					Self::AnyPredicate(AnySubjectAnyPredicate::from_pattern(s, p, o, g))
				}
			}
		}
	}

	pub fn matches(&self, quad: Quad<T>) -> bool
	where
		T: PartialEq,
	{
		match self {
			Self::AnyPredicate(pattern) => pattern.matches(quad),
			Self::SameAsSubject(pattern) => {
				quad.predicate() == quad.subject() && pattern.matches(quad)
			}
			Self::GivenPredicate(p, pattern) => quad.predicate() == p && pattern.matches(quad),
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

	pub fn as_ref(&self) -> AnySubject<&T> {
		match self {
			Self::AnyPredicate(og) => AnySubject::AnyPredicate(og.as_ref()),
			Self::SameAsSubject(og) => AnySubject::SameAsSubject(og.as_ref()),
			Self::GivenPredicate(p, og) => AnySubject::GivenPredicate(p, og.as_ref()),
		}
	}

	pub fn map<U>(self, mut f: impl FnMut(T) -> U) -> AnySubject<U> {
		match self {
			Self::AnyPredicate(og) => AnySubject::AnyPredicate(og.map(f)),
			Self::SameAsSubject(og) => AnySubject::SameAsSubject(og.map(f)),
			Self::GivenPredicate(p, og) => AnySubject::GivenPredicate(f(p), og.map(f)),
		}
	}

	pub fn map2<U, V>(self, mut f: impl FnMut(T) -> (U, V)) -> (AnySubject<U>, AnySubject<V>) {
		match self {
			Self::AnyPredicate(og) => {
				let (og_u, og_v) = og.map2(f);
				(
					AnySubject::AnyPredicate(og_u),
					AnySubject::AnyPredicate(og_v),
				)
			}
			Self::SameAsSubject(og) => {
				let (og_u, og_v) = og.map2(f);
				(
					AnySubject::SameAsSubject(og_u),
					AnySubject::SameAsSubject(og_v),
				)
			}
			Self::GivenPredicate(p, og) => {
				let (u, v) = f(p);
				let (og_u, og_v) = og.map2(f);
				(
					AnySubject::GivenPredicate(u, og_u),
					AnySubject::GivenPredicate(v, og_v),
				)
			}
		}
	}

	pub fn into_triple(self) -> (triple::AnySubject<T>, PatternGraph<T>) {
		match self {
			Self::AnyPredicate(t) => {
				let (u, g) = t.into_triple();
				(triple::AnySubject::AnyPredicate(u), g)
			}
			Self::SameAsSubject(t) => {
				let (u, g) = t.into_triple();
				(triple::AnySubject::SameAsSubject(u), g)
			}
			Self::GivenPredicate(id, t) => {
				let (u, g) = t.into_triple();
				(triple::AnySubject::GivenPredicate(id, u), g)
			}
		}
	}

	pub fn into_parts(self) -> (PatternPredicate<T>, PatternObject<T>, PatternGraph<T>) {
		match self {
			Self::AnyPredicate(og) => {
				let (o, g) = og.into_parts();
				(PatternPredicate::Any, o, g)
			}
			Self::SameAsSubject(og) => {
				let (o, g) = og.into_parts();
				(PatternPredicate::SameAsSubject, o, g)
			}
			Self::GivenPredicate(p, og) => {
				let (o, g) = og.into_parts();
				(PatternPredicate::Given(p), o, g)
			}
		}
	}

	pub fn set_subject(self, s: T) -> GivenSubject<T>
	where
		T: Clone,
	{
		match self {
			Self::AnyPredicate(pog) => GivenSubject::AnyPredicate(pog.set_subject(s)),
			Self::SameAsSubject(pog) => GivenSubject::GivenPredicate(s.clone(), pog.set_subject(s)),
			Self::GivenPredicate(p, pog) => GivenSubject::GivenPredicate(p, pog.set_subject(s)),
		}
	}

	pub fn set_predicate(&mut self, p: T) -> PatternPredicate<T>
	where
		T: Clone,
	{
		replace_with_or_abort_and_return(self, |this| match this {
			Self::AnyPredicate(AnySubjectAnyPredicate::SameAsPredicate(g)) => (
				PatternPredicate::Any,
				Self::GivenPredicate(
					p.clone(),
					AnySubjectGivenPredicate::GivenObject(p.clone(), g.set_predicate(p)),
				),
			),
			Self::AnyPredicate(AnySubjectAnyPredicate::AnyObject(g)) => (
				PatternPredicate::Any,
				Self::GivenPredicate(
					p.clone(),
					AnySubjectGivenPredicate::AnyObject(g.set_predicate(p)),
				),
			),
			Self::AnyPredicate(AnySubjectAnyPredicate::SameAsSubject(g)) => (
				PatternPredicate::Any,
				Self::GivenPredicate(
					p.clone(),
					AnySubjectGivenPredicate::SameAsSubject(g.set_predicate(p)),
				),
			),
			Self::AnyPredicate(AnySubjectAnyPredicate::GivenObject(o, g)) => (
				PatternPredicate::Any,
				Self::GivenPredicate(
					p.clone(),
					AnySubjectGivenPredicate::GivenObject(o, g.set_predicate(p)),
				),
			),
			Self::SameAsSubject(AnySubjectGivenPredicate::AnyObject(g)) => (
				PatternPredicate::SameAsSubject,
				Self::GivenPredicate(p, AnySubjectGivenPredicate::AnyObject(g)),
			),
			Self::SameAsSubject(AnySubjectGivenPredicate::SameAsSubject(g)) => (
				PatternPredicate::SameAsSubject,
				Self::GivenPredicate(p, AnySubjectGivenPredicate::SameAsSubject(g)),
			),
			Self::SameAsSubject(AnySubjectGivenPredicate::GivenObject(o, g)) => (
				PatternPredicate::SameAsSubject,
				Self::GivenPredicate(p, AnySubjectGivenPredicate::GivenObject(o, g)),
			),
			Self::GivenPredicate(old_p, rest) => (
				PatternPredicate::Given(old_p),
				Self::GivenPredicate(p, rest),
			),
		})
	}

	pub fn set_object(&mut self, o: T) -> PatternObject<T>
	where
		T: Clone,
	{
		replace_with_or_abort_and_return(self, |this| match this {
			Self::AnyPredicate(AnySubjectAnyPredicate::SameAsPredicate(g)) => (
				PatternObject::Any,
				Self::GivenPredicate(
					o.clone(),
					AnySubjectGivenPredicate::GivenObject(o.clone(), g.set_predicate(o)),
				),
			),
			Self::AnyPredicate(mut current) => {
				let old_o = current.set_object(o);
				(old_o, Self::AnyPredicate(current))
			}
			Self::SameAsSubject(AnySubjectGivenPredicate::SameAsSubject(g)) => (
				PatternObject::SameAsSubject,
				Self::GivenPredicate(o.clone(), AnySubjectGivenPredicate::GivenObject(o, g)),
			),
			Self::SameAsSubject(mut current) => {
				let old_o = current.set_object(o);
				(old_o, Self::SameAsSubject(current))
			}
			Self::GivenPredicate(p, mut current) => {
				let old_o = current.set_object(o);
				(old_o, Self::GivenPredicate(p, current))
			}
		})
	}

	pub fn set_graph(&mut self, g: Option<T>) -> PatternGraph<T> {
		match self {
			Self::AnyPredicate(rest) => rest.set_graph(g),
			Self::SameAsSubject(rest) => rest.set_graph(g),
			Self::GivenPredicate(_, rest) => rest.set_graph(g),
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
		o: Pattern<T, X>,
		g: Option<Pattern<T, X>>,
	) -> Self {
		match o {
			Pattern::Ground(o) => {
				Self::GivenObject(o, AnySubjectAnyPredicateGivenObject::from_pattern(s, p, g))
			}
			Pattern::Var(o) => {
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

	pub fn matches(&self, quad: Quad<T>) -> bool
	where
		T: PartialEq,
	{
		match self {
			Self::AnyObject(pattern) => pattern.matches(quad),
			Self::SameAsSubject(pattern) => {
				quad.object() == quad.subject() && pattern.matches(quad)
			}
			Self::SameAsPredicate(pattern) => {
				quad.object() == quad.predicate() && pattern.matches(quad)
			}
			Self::GivenObject(o, pattern) => quad.object() == o && pattern.matches(quad),
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

	pub fn as_ref(&self) -> AnySubjectAnyPredicate<&T> {
		match self {
			Self::AnyObject(g) => AnySubjectAnyPredicate::AnyObject(g.as_ref()),
			Self::SameAsSubject(g) => AnySubjectAnyPredicate::SameAsSubject(g.as_ref()),
			Self::SameAsPredicate(g) => AnySubjectAnyPredicate::SameAsPredicate(g.as_ref()),
			Self::GivenObject(o, g) => AnySubjectAnyPredicate::GivenObject(o, g.as_ref()),
		}
	}

	pub fn map<U>(self, mut f: impl FnMut(T) -> U) -> AnySubjectAnyPredicate<U> {
		match self {
			Self::AnyObject(g) => AnySubjectAnyPredicate::AnyObject(g.map(f)),
			Self::SameAsSubject(g) => AnySubjectAnyPredicate::SameAsSubject(g.map(f)),
			Self::SameAsPredicate(g) => AnySubjectAnyPredicate::SameAsPredicate(g.map(f)),
			Self::GivenObject(o, g) => AnySubjectAnyPredicate::GivenObject(f(o), g.map(f)),
		}
	}

	pub fn map2<U, V>(
		self,
		mut f: impl FnMut(T) -> (U, V),
	) -> (AnySubjectAnyPredicate<U>, AnySubjectAnyPredicate<V>) {
		match self {
			Self::AnyObject(og) => {
				let (og_u, og_v) = og.map2(f);
				(
					AnySubjectAnyPredicate::AnyObject(og_u),
					AnySubjectAnyPredicate::AnyObject(og_v),
				)
			}
			Self::SameAsSubject(og) => {
				let (og_u, og_v) = og.map2(f);
				(
					AnySubjectAnyPredicate::SameAsSubject(og_u),
					AnySubjectAnyPredicate::SameAsSubject(og_v),
				)
			}
			Self::SameAsPredicate(og) => {
				let (og_u, og_v) = og.map2(f);
				(
					AnySubjectAnyPredicate::SameAsPredicate(og_u),
					AnySubjectAnyPredicate::SameAsPredicate(og_v),
				)
			}
			Self::GivenObject(o, og) => {
				let (u, v) = f(o);
				let (og_u, og_v) = og.map2(f);
				(
					AnySubjectAnyPredicate::GivenObject(u, og_u),
					AnySubjectAnyPredicate::GivenObject(v, og_v),
				)
			}
		}
	}

	pub fn into_triple(self) -> (triple::AnySubjectAnyPredicate<T>, PatternGraph<T>) {
		match self {
			Self::AnyObject(t) => (triple::AnySubjectAnyPredicate::AnyObject, t.into_graph()),
			Self::SameAsSubject(t) => (
				triple::AnySubjectAnyPredicate::SameAsSubject,
				t.into_graph(),
			),
			Self::SameAsPredicate(t) => (
				triple::AnySubjectAnyPredicate::SameAsPredicate,
				t.into_graph(),
			),
			Self::GivenObject(id, t) => (
				triple::AnySubjectAnyPredicate::GivenObject(id),
				t.into_graph(),
			),
		}
	}

	pub fn into_parts(self) -> (PatternObject<T>, PatternGraph<T>) {
		match self {
			Self::AnyObject(g) => (PatternObject::Any, g.into_graph()),
			Self::SameAsSubject(g) => (PatternObject::SameAsSubject, g.into_graph()),
			Self::SameAsPredicate(g) => (PatternObject::SameAsPredicate, g.into_graph()),
			Self::GivenObject(o, g) => (PatternObject::Given(o), g.into_graph()),
		}
	}

	pub fn set_subject(self, s: T) -> GivenSubjectAnyPredicate<T>
	where
		T: Clone,
	{
		match self {
			Self::AnyObject(g) => GivenSubjectAnyPredicate::AnyObject(g.set_subject(s)),
			Self::SameAsSubject(g) => {
				GivenSubjectAnyPredicate::GivenObject(s.clone(), g.set_subject(s))
			}
			Self::SameAsPredicate(g) => GivenSubjectAnyPredicate::SameAsPredicate(g.set_subject(s)),
			Self::GivenObject(o, g) => GivenSubjectAnyPredicate::GivenObject(o, g.set_subject(s)),
		}
	}

	pub fn set_object(&mut self, o: T) -> PatternObject<T>
	where
		T: Clone,
	{
		replace_with_or_abort_and_return(self, |this| match this {
			Self::AnyObject(g) => (
				PatternObject::Any,
				Self::GivenObject(o.clone(), g.set_object(o)),
			),
			Self::SameAsSubject(g) => (PatternObject::SameAsSubject, Self::GivenObject(o, g)),
			Self::SameAsPredicate(g) => (PatternObject::SameAsPredicate, Self::GivenObject(o, g)),
			Self::GivenObject(old_o, g) => (PatternObject::Given(old_o), Self::GivenObject(o, g)),
		})
	}

	pub fn set_graph(&mut self, g: Option<T>) -> PatternGraph<T> {
		match self {
			Self::AnyObject(rest) => rest.set_graph(g),
			Self::SameAsSubject(rest) => rest.set_graph(g),
			Self::SameAsPredicate(rest) => rest.set_graph(g),
			Self::GivenObject(_, rest) => rest.set_graph(g),
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

	pub fn from_pattern<X: PartialEq>(s: X, o: Pattern<T, X>, g: Option<Pattern<T, X>>) -> Self {
		match o {
			Pattern::Ground(o) => {
				Self::GivenObject(o, AnySubjectGivenPredicateGivenObject::from_pattern(s, g))
			}
			Pattern::Var(o) => {
				if o == s {
					Self::SameAsSubject(AnySubjectGivenPredicateGivenObject::from_pattern(s, g))
				} else {
					Self::AnyObject(AnySubjectGivenPredicateAnyObject::from_pattern(s, o, g))
				}
			}
		}
	}

	pub fn matches(&self, quad: Quad<T>) -> bool
	where
		T: PartialEq,
	{
		match self {
			Self::AnyObject(pattern) => pattern.matches(quad),
			Self::SameAsSubject(pattern) => {
				quad.object() == quad.subject() && pattern.matches(quad)
			}
			Self::GivenObject(o, pattern) => quad.object() == o && pattern.matches(quad),
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

	pub fn as_ref(&self) -> AnySubjectGivenPredicate<&T> {
		match self {
			Self::AnyObject(g) => AnySubjectGivenPredicate::AnyObject(g.as_ref()),
			Self::SameAsSubject(g) => AnySubjectGivenPredicate::SameAsSubject(g.as_ref()),
			Self::GivenObject(o, g) => AnySubjectGivenPredicate::GivenObject(o, g.as_ref()),
		}
	}

	pub fn map<U>(self, mut f: impl FnMut(T) -> U) -> AnySubjectGivenPredicate<U> {
		match self {
			Self::AnyObject(g) => AnySubjectGivenPredicate::AnyObject(g.map(f)),
			Self::SameAsSubject(g) => AnySubjectGivenPredicate::SameAsSubject(g.map(f)),
			Self::GivenObject(o, g) => AnySubjectGivenPredicate::GivenObject(f(o), g.map(f)),
		}
	}

	pub fn map2<U, V>(
		self,
		mut f: impl FnMut(T) -> (U, V),
	) -> (AnySubjectGivenPredicate<U>, AnySubjectGivenPredicate<V>) {
		match self {
			Self::AnyObject(og) => {
				let (og_u, og_v) = og.map2(f);
				(
					AnySubjectGivenPredicate::AnyObject(og_u),
					AnySubjectGivenPredicate::AnyObject(og_v),
				)
			}
			Self::SameAsSubject(og) => {
				let (og_u, og_v) = og.map2(f);
				(
					AnySubjectGivenPredicate::SameAsSubject(og_u),
					AnySubjectGivenPredicate::SameAsSubject(og_v),
				)
			}
			Self::GivenObject(o, og) => {
				let (u, v) = f(o);
				let (og_u, og_v) = og.map2(f);
				(
					AnySubjectGivenPredicate::GivenObject(u, og_u),
					AnySubjectGivenPredicate::GivenObject(v, og_v),
				)
			}
		}
	}

	pub fn into_triple(self) -> (triple::AnySubjectGivenPredicate<T>, PatternGraph<T>) {
		match self {
			Self::AnyObject(t) => (triple::AnySubjectGivenPredicate::AnyObject, t.into_graph()),
			Self::SameAsSubject(t) => (
				triple::AnySubjectGivenPredicate::SameAsSubject,
				t.into_graph(),
			),
			Self::GivenObject(id, t) => (
				triple::AnySubjectGivenPredicate::GivenObject(id),
				t.into_graph(),
			),
		}
	}

	pub fn into_parts(self) -> (PatternObject<T>, PatternGraph<T>) {
		match self {
			Self::AnyObject(g) => (PatternObject::Any, g.into_graph()),
			Self::SameAsSubject(g) => (PatternObject::SameAsSubject, g.into_graph()),
			Self::GivenObject(o, g) => (PatternObject::Given(o), g.into_graph()),
		}
	}

	pub fn set_subject(self, s: T) -> GivenSubjectGivenPredicate<T>
	where
		T: Clone,
	{
		match self {
			Self::AnyObject(g) => GivenSubjectGivenPredicate::AnyObject(g.set_subject(s)),
			Self::SameAsSubject(g) => {
				GivenSubjectGivenPredicate::GivenObject(s.clone(), g.set_subject(s))
			}
			Self::GivenObject(o, g) => GivenSubjectGivenPredicate::GivenObject(o, g.set_subject(s)),
		}
	}

	pub fn set_object(&mut self, o: T) -> PatternObject<T>
	where
		T: Clone,
	{
		replace_with_or_abort_and_return(self, |this| match this {
			Self::AnyObject(g) => (
				PatternObject::Any,
				Self::GivenObject(o.clone(), g.set_object(o)),
			),
			Self::SameAsSubject(g) => (PatternObject::SameAsSubject, Self::GivenObject(o, g)),
			Self::GivenObject(old_o, g) => (PatternObject::Given(old_o), Self::GivenObject(o, g)),
		})
	}

	pub fn set_graph(&mut self, g: Option<T>) -> PatternGraph<T> {
		match self {
			Self::AnyObject(rest) => rest.set_graph(g),
			Self::SameAsSubject(rest) => rest.set_graph(g),
			Self::GivenObject(_, rest) => rest.set_graph(g),
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
		p: Pattern<T, X>,
		o: Pattern<T, X>,
		g: Option<Pattern<T, X>>,
	) -> Self {
		match p {
			Pattern::Ground(p) => {
				Self::GivenPredicate(p, GivenSubjectGivenPredicate::from_pattern(o, g))
			}
			Pattern::Var(p) => Self::AnyPredicate(GivenSubjectAnyPredicate::from_pattern(p, o, g)),
		}
	}

	pub fn matches(&self, quad: Quad<T>) -> bool
	where
		T: PartialEq,
	{
		match self {
			Self::AnyPredicate(pattern) => pattern.matches(quad),
			Self::GivenPredicate(p, pattern) => quad.predicate() == p && pattern.matches(quad),
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

	pub fn as_ref(&self) -> GivenSubject<&T> {
		match self {
			Self::AnyPredicate(og) => GivenSubject::AnyPredicate(og.as_ref()),
			Self::GivenPredicate(p, og) => GivenSubject::GivenPredicate(p, og.as_ref()),
		}
	}

	pub fn map<U>(self, mut f: impl FnMut(T) -> U) -> GivenSubject<U> {
		match self {
			Self::AnyPredicate(og) => GivenSubject::AnyPredicate(og.map(f)),
			Self::GivenPredicate(p, og) => GivenSubject::GivenPredicate(f(p), og.map(f)),
		}
	}

	pub fn map2<U, V>(self, mut f: impl FnMut(T) -> (U, V)) -> (GivenSubject<U>, GivenSubject<V>) {
		match self {
			Self::AnyPredicate(og) => {
				let (og_u, og_v) = og.map2(f);
				(
					GivenSubject::AnyPredicate(og_u),
					GivenSubject::AnyPredicate(og_v),
				)
			}
			Self::GivenPredicate(p, og) => {
				let (u, v) = f(p);
				let (og_u, og_v) = og.map2(f);
				(
					GivenSubject::GivenPredicate(u, og_u),
					GivenSubject::GivenPredicate(v, og_v),
				)
			}
		}
	}

	pub fn into_triple(self) -> (triple::GivenSubject<T>, PatternGraph<T>) {
		match self {
			Self::AnyPredicate(t) => {
				let (u, g) = t.into_triple();
				(triple::GivenSubject::AnyPredicate(u), g)
			}
			Self::GivenPredicate(id, t) => {
				let (u, g) = t.into_triple();
				(triple::GivenSubject::GivenPredicate(id, u), g)
			}
		}
	}

	pub fn into_parts(self) -> (PatternPredicate<T>, PatternObject<T>, PatternGraph<T>) {
		match self {
			Self::AnyPredicate(og) => {
				let (o, g) = og.into_parts();
				(PatternPredicate::Any, o, g)
			}
			Self::GivenPredicate(p, og) => {
				let (o, g) = og.into_parts();
				(PatternPredicate::Given(p), o, g)
			}
		}
	}

	pub fn set_predicate(&mut self, p: T) -> PatternPredicate<T>
	where
		T: Clone,
	{
		replace_with_or_abort_and_return(self, |this| match this {
			Self::AnyPredicate(GivenSubjectAnyPredicate::AnyObject(g)) => (
				PatternPredicate::Any,
				Self::GivenPredicate(
					p.clone(),
					GivenSubjectGivenPredicate::AnyObject(g.set_predicate(p)),
				),
			),
			Self::AnyPredicate(GivenSubjectAnyPredicate::SameAsPredicate(g)) => (
				PatternPredicate::Any,
				Self::GivenPredicate(
					p.clone(),
					GivenSubjectGivenPredicate::GivenObject(p.clone(), g.set_predicate(p)),
				),
			),
			Self::AnyPredicate(GivenSubjectAnyPredicate::GivenObject(o, g)) => (
				PatternPredicate::Any,
				Self::GivenPredicate(
					p.clone(),
					GivenSubjectGivenPredicate::GivenObject(o, g.set_predicate(p)),
				),
			),
			Self::GivenPredicate(old_p, rest) => (
				PatternPredicate::Given(old_p),
				Self::GivenPredicate(p, rest),
			),
		})
	}

	pub fn set_object(&mut self, o: T) -> PatternObject<T>
	where
		T: Clone,
	{
		replace_with_or_abort_and_return(self, |this| match this {
			Self::AnyPredicate(GivenSubjectAnyPredicate::SameAsPredicate(g)) => (
				PatternObject::Any,
				Self::GivenPredicate(
					o.clone(),
					GivenSubjectGivenPredicate::GivenObject(o.clone(), g.set_predicate(o)),
				),
			),
			Self::AnyPredicate(mut current) => {
				let old_o = current.set_object(o);
				(old_o, Self::AnyPredicate(current))
			}
			Self::GivenPredicate(p, mut current) => {
				let old_o = current.set_object(o);
				(old_o, Self::GivenPredicate(p, current))
			}
		})
	}

	pub fn set_graph(&mut self, g: Option<T>) -> PatternGraph<T> {
		match self {
			Self::AnyPredicate(rest) => rest.set_graph(g),
			Self::GivenPredicate(_, rest) => rest.set_graph(g),
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

	pub fn from_pattern<X: PartialEq>(p: X, o: Pattern<T, X>, g: Option<Pattern<T, X>>) -> Self {
		match o {
			Pattern::Ground(o) => {
				Self::GivenObject(o, GivenSubjectAnyPredicateGivenObject::from_pattern(p, g))
			}
			Pattern::Var(o) => {
				if p == o {
					Self::SameAsPredicate(GivenSubjectAnyPredicateGivenObject::from_pattern(p, g))
				} else {
					Self::AnyObject(GivenSubjectAnyPredicateAnyObject::from_pattern(p, o, g))
				}
			}
		}
	}

	pub fn matches(&self, quad: Quad<T>) -> bool
	where
		T: PartialEq,
	{
		match self {
			Self::AnyObject(pattern) => pattern.matches(quad),
			Self::SameAsPredicate(pattern) => {
				quad.object() == quad.predicate() && pattern.matches(quad)
			}
			Self::GivenObject(o, pattern) => quad.object() == o && pattern.matches(quad),
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

	pub fn as_ref(&self) -> GivenSubjectAnyPredicate<&T> {
		match self {
			Self::AnyObject(g) => GivenSubjectAnyPredicate::AnyObject(g.as_ref()),
			Self::SameAsPredicate(g) => GivenSubjectAnyPredicate::SameAsPredicate(g.as_ref()),
			Self::GivenObject(o, g) => GivenSubjectAnyPredicate::GivenObject(o, g.as_ref()),
		}
	}

	pub fn map<U>(self, mut f: impl FnMut(T) -> U) -> GivenSubjectAnyPredicate<U> {
		match self {
			Self::AnyObject(g) => GivenSubjectAnyPredicate::AnyObject(g.map(f)),
			Self::SameAsPredicate(g) => GivenSubjectAnyPredicate::SameAsPredicate(g.map(f)),
			Self::GivenObject(o, g) => GivenSubjectAnyPredicate::GivenObject(f(o), g.map(f)),
		}
	}

	pub fn map2<U, V>(
		self,
		mut f: impl FnMut(T) -> (U, V),
	) -> (GivenSubjectAnyPredicate<U>, GivenSubjectAnyPredicate<V>) {
		match self {
			Self::AnyObject(og) => {
				let (og_u, og_v) = og.map2(f);
				(
					GivenSubjectAnyPredicate::AnyObject(og_u),
					GivenSubjectAnyPredicate::AnyObject(og_v),
				)
			}
			Self::SameAsPredicate(og) => {
				let (og_u, og_v) = og.map2(f);
				(
					GivenSubjectAnyPredicate::SameAsPredicate(og_u),
					GivenSubjectAnyPredicate::SameAsPredicate(og_v),
				)
			}
			Self::GivenObject(o, og) => {
				let (u, v) = f(o);
				let (og_u, og_v) = og.map2(f);
				(
					GivenSubjectAnyPredicate::GivenObject(u, og_u),
					GivenSubjectAnyPredicate::GivenObject(v, og_v),
				)
			}
		}
	}

	pub fn into_triple(self) -> (triple::GivenSubjectAnyPredicate<T>, PatternGraph<T>) {
		match self {
			Self::AnyObject(t) => (triple::GivenSubjectAnyPredicate::AnyObject, t.into_graph()),
			Self::SameAsPredicate(t) => (
				triple::GivenSubjectAnyPredicate::SameAsPredicate,
				t.into_graph(),
			),
			Self::GivenObject(id, t) => (
				triple::GivenSubjectAnyPredicate::GivenObject(id),
				t.into_graph(),
			),
		}
	}

	pub fn into_parts(self) -> (PatternObject<T>, PatternGraph<T>) {
		match self {
			Self::AnyObject(g) => (PatternObject::Any, g.into_graph()),
			Self::SameAsPredicate(g) => (PatternObject::SameAsPredicate, g.into_graph()),
			Self::GivenObject(o, g) => (PatternObject::Given(o), g.into_graph()),
		}
	}

	pub fn set_object(&mut self, o: T) -> PatternObject<T>
	where
		T: Clone,
	{
		replace_with_or_abort_and_return(self, |this| match this {
			Self::AnyObject(g) => (
				PatternObject::Any,
				Self::GivenObject(o.clone(), g.set_object(o)),
			),
			Self::SameAsPredicate(g) => (PatternObject::SameAsPredicate, Self::GivenObject(o, g)),
			Self::GivenObject(old_o, g) => (PatternObject::Given(old_o), Self::GivenObject(o, g)),
		})
	}

	pub fn set_graph(&mut self, g: Option<T>) -> PatternGraph<T> {
		match self {
			Self::AnyObject(rest) => rest.set_graph(g),
			Self::SameAsPredicate(rest) => rest.set_graph(g),
			Self::GivenObject(_, rest) => rest.set_graph(g),
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

	pub fn from_pattern<X: PartialEq>(o: Pattern<T, X>, g: Option<Pattern<T, X>>) -> Self {
		match o {
			Pattern::Ground(o) => {
				Self::GivenObject(o, GivenSubjectGivenPredicateGivenObject::from_pattern(g))
			}
			Pattern::Var(o) => {
				Self::AnyObject(GivenSubjectGivenPredicateAnyObject::from_pattern(o, g))
			}
		}
	}

	pub fn matches(&self, quad: Quad<T>) -> bool
	where
		T: PartialEq,
	{
		match self {
			Self::AnyObject(pattern) => pattern.matches(quad),
			Self::GivenObject(o, pattern) => quad.object() == o && pattern.matches(quad),
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

	pub fn as_ref(&self) -> GivenSubjectGivenPredicate<&T> {
		match self {
			Self::AnyObject(g) => GivenSubjectGivenPredicate::AnyObject(g.as_ref()),
			Self::GivenObject(o, g) => GivenSubjectGivenPredicate::GivenObject(o, g.as_ref()),
		}
	}

	pub fn map<U>(self, mut f: impl FnMut(T) -> U) -> GivenSubjectGivenPredicate<U> {
		match self {
			Self::AnyObject(g) => GivenSubjectGivenPredicate::AnyObject(g.map(f)),
			Self::GivenObject(o, g) => GivenSubjectGivenPredicate::GivenObject(f(o), g.map(f)),
		}
	}

	pub fn map2<U, V>(
		self,
		mut f: impl FnMut(T) -> (U, V),
	) -> (GivenSubjectGivenPredicate<U>, GivenSubjectGivenPredicate<V>) {
		match self {
			Self::AnyObject(og) => {
				let (og_u, og_v) = og.map2(f);
				(
					GivenSubjectGivenPredicate::AnyObject(og_u),
					GivenSubjectGivenPredicate::AnyObject(og_v),
				)
			}
			Self::GivenObject(o, og) => {
				let (u, v) = f(o);
				let (og_u, og_v) = og.map2(f);
				(
					GivenSubjectGivenPredicate::GivenObject(u, og_u),
					GivenSubjectGivenPredicate::GivenObject(v, og_v),
				)
			}
		}
	}

	pub fn into_triple(self) -> (triple::GivenSubjectGivenPredicate<T>, PatternGraph<T>) {
		match self {
			Self::AnyObject(t) => (
				triple::GivenSubjectGivenPredicate::AnyObject,
				t.into_graph(),
			),
			Self::GivenObject(id, t) => (
				triple::GivenSubjectGivenPredicate::GivenObject(id),
				t.into_graph(),
			),
		}
	}

	pub fn into_parts(self) -> (PatternObject<T>, PatternGraph<T>) {
		match self {
			Self::AnyObject(g) => (PatternObject::Any, g.into_graph()),
			Self::GivenObject(o, g) => (PatternObject::Given(o), g.into_graph()),
		}
	}

	pub fn set_object(&mut self, o: T) -> PatternObject<T>
	where
		T: Clone,
	{
		replace_with_or_abort_and_return(self, |this| match this {
			Self::AnyObject(g) => (
				PatternObject::Any,
				Self::GivenObject(o.clone(), g.set_object(o)),
			),
			Self::GivenObject(old_o, g) => (PatternObject::Given(old_o), Self::GivenObject(o, g)),
		})
	}

	pub fn set_graph(&mut self, g: Option<T>) -> PatternGraph<T> {
		match self {
			Self::AnyObject(rest) => rest.set_graph(g),
			Self::GivenObject(_, rest) => rest.set_graph(g),
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

impl<T> PatternGraph<T> {
	pub fn id(&self) -> Option<&T> {
		match self {
			Self::Given(g) => g.as_ref(),
			_ => None,
		}
	}

	pub fn into_id(self) -> Option<T> {
		match self {
			Self::Given(g) => g,
			_ => None,
		}
	}
}

impl<T> PatternGraph<&T> {
	pub fn cloned(self) -> PatternGraph<T>
	where
		T: Clone,
	{
		match self {
			Self::Any => PatternGraph::Any,
			Self::SameAsSubject => PatternGraph::SameAsSubject,
			Self::SameAsPredicate => PatternGraph::SameAsPredicate,
			Self::SameAsObject => PatternGraph::SameAsObject,
			Self::Given(g) => PatternGraph::Given(g.cloned()),
		}
	}
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

	pub fn from_pattern<X: PartialEq>(s: X, p: X, o: X, g: Option<Pattern<T, X>>) -> Self {
		match g {
			None => Self::GivenGraph(None),
			Some(Pattern::Ground(g)) => Self::GivenGraph(Some(g)),
			Some(Pattern::Var(g)) => {
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

	pub fn matches(&self, quad: Quad<T>) -> bool
	where
		T: PartialEq,
	{
		match self {
			Self::AnyGraph => true,
			Self::SameAsSubject => quad.graph() == Some(quad.subject()),
			Self::SameAsPredicate => quad.graph() == Some(quad.predicate()),
			Self::SameAsObject => quad.graph() == Some(quad.object()),
			Self::GivenGraph(g) => quad.graph() == g.as_ref(),
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

	pub fn as_ref(&self) -> AnySubjectAnyPredicateAnyObject<&T> {
		match self {
			Self::AnyGraph => AnySubjectAnyPredicateAnyObject::AnyGraph,
			Self::SameAsSubject => AnySubjectAnyPredicateAnyObject::SameAsSubject,
			Self::SameAsPredicate => AnySubjectAnyPredicateAnyObject::SameAsPredicate,
			Self::SameAsObject => AnySubjectAnyPredicateAnyObject::SameAsObject,
			Self::GivenGraph(g) => AnySubjectAnyPredicateAnyObject::GivenGraph(g.as_ref()),
		}
	}

	pub fn map<U>(self, f: impl FnMut(T) -> U) -> AnySubjectAnyPredicateAnyObject<U> {
		match self {
			Self::AnyGraph => AnySubjectAnyPredicateAnyObject::AnyGraph,
			Self::SameAsSubject => AnySubjectAnyPredicateAnyObject::SameAsSubject,
			Self::SameAsPredicate => AnySubjectAnyPredicateAnyObject::SameAsPredicate,
			Self::SameAsObject => AnySubjectAnyPredicateAnyObject::SameAsObject,
			Self::GivenGraph(g) => AnySubjectAnyPredicateAnyObject::GivenGraph(g.map(f)),
		}
	}

	pub fn map2<U, V>(
		self,
		mut f: impl FnMut(T) -> (U, V),
	) -> (
		AnySubjectAnyPredicateAnyObject<U>,
		AnySubjectAnyPredicateAnyObject<V>,
	) {
		match self {
			Self::AnyGraph => (
				AnySubjectAnyPredicateAnyObject::AnyGraph,
				AnySubjectAnyPredicateAnyObject::AnyGraph,
			),
			Self::SameAsSubject => (
				AnySubjectAnyPredicateAnyObject::SameAsSubject,
				AnySubjectAnyPredicateAnyObject::SameAsSubject,
			),
			Self::SameAsPredicate => (
				AnySubjectAnyPredicateAnyObject::SameAsPredicate,
				AnySubjectAnyPredicateAnyObject::SameAsPredicate,
			),
			Self::SameAsObject => (
				AnySubjectAnyPredicateAnyObject::SameAsObject,
				AnySubjectAnyPredicateAnyObject::SameAsObject,
			),
			Self::GivenGraph(g) => {
				let (g_u, g_v) = match g {
					Some(g) => {
						let (g_u, g_v) = f(g);
						(Some(g_u), Some(g_v))
					}
					None => (None, None),
				};
				(
					AnySubjectAnyPredicateAnyObject::GivenGraph(g_u),
					AnySubjectAnyPredicateAnyObject::GivenGraph(g_v),
				)
			}
		}
	}

	pub fn set_subject(self, s: T) -> GivenSubjectAnyPredicateAnyObject<T> {
		match self {
			Self::AnyGraph => GivenSubjectAnyPredicateAnyObject::AnyGraph,
			Self::SameAsSubject => GivenSubjectAnyPredicateAnyObject::GivenGraph(Some(s)),
			Self::SameAsPredicate => GivenSubjectAnyPredicateAnyObject::SameAsPredicate,
			Self::SameAsObject => GivenSubjectAnyPredicateAnyObject::SameAsObject,
			Self::GivenGraph(g) => GivenSubjectAnyPredicateAnyObject::GivenGraph(g),
		}
	}

	pub fn set_predicate(self, p: T) -> AnySubjectGivenPredicateAnyObject<T> {
		match self {
			Self::AnyGraph => AnySubjectGivenPredicateAnyObject::AnyGraph,
			Self::SameAsSubject => AnySubjectGivenPredicateAnyObject::SameAsSubject,
			Self::SameAsPredicate => AnySubjectGivenPredicateAnyObject::GivenGraph(Some(p)),
			Self::SameAsObject => AnySubjectGivenPredicateAnyObject::SameAsObject,
			Self::GivenGraph(g) => AnySubjectGivenPredicateAnyObject::GivenGraph(g),
		}
	}

	pub fn set_object(self, o: T) -> AnySubjectAnyPredicateGivenObject<T> {
		match self {
			Self::AnyGraph => AnySubjectAnyPredicateGivenObject::AnyGraph,
			Self::SameAsSubject => AnySubjectAnyPredicateGivenObject::SameAsSubject,
			Self::SameAsPredicate => AnySubjectAnyPredicateGivenObject::SameAsPredicate,
			Self::SameAsObject => AnySubjectAnyPredicateGivenObject::GivenGraph(Some(o)),
			Self::GivenGraph(g) => AnySubjectAnyPredicateGivenObject::GivenGraph(g),
		}
	}

	pub fn set_graph(&mut self, g: Option<T>) -> PatternGraph<T> {
		std::mem::replace(self, Self::GivenGraph(g)).into_graph()
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

	pub fn from_pattern<X: PartialEq>(p: X, o: X, g: Option<Pattern<T, X>>) -> Self {
		match g {
			None => Self::GivenGraph(None),
			Some(Pattern::Ground(g)) => Self::GivenGraph(Some(g)),
			Some(Pattern::Var(g)) => {
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

	pub fn matches(&self, quad: Quad<T>) -> bool
	where
		T: PartialEq,
	{
		match self {
			Self::AnyGraph => true,
			Self::SameAsPredicate => quad.graph() == Some(quad.predicate()),
			Self::SameAsObject => quad.graph() == Some(quad.object()),
			Self::GivenGraph(g) => quad.graph() == g.as_ref(),
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

	pub fn as_ref(&self) -> GivenSubjectAnyPredicateAnyObject<&T> {
		match self {
			Self::AnyGraph => GivenSubjectAnyPredicateAnyObject::AnyGraph,
			Self::SameAsPredicate => GivenSubjectAnyPredicateAnyObject::SameAsPredicate,
			Self::SameAsObject => GivenSubjectAnyPredicateAnyObject::SameAsObject,
			Self::GivenGraph(g) => GivenSubjectAnyPredicateAnyObject::GivenGraph(g.as_ref()),
		}
	}

	pub fn map<U>(self, f: impl FnMut(T) -> U) -> GivenSubjectAnyPredicateAnyObject<U> {
		match self {
			Self::AnyGraph => GivenSubjectAnyPredicateAnyObject::AnyGraph,
			Self::SameAsPredicate => GivenSubjectAnyPredicateAnyObject::SameAsPredicate,
			Self::SameAsObject => GivenSubjectAnyPredicateAnyObject::SameAsObject,
			Self::GivenGraph(g) => GivenSubjectAnyPredicateAnyObject::GivenGraph(g.map(f)),
		}
	}

	pub fn map2<U, V>(
		self,
		mut f: impl FnMut(T) -> (U, V),
	) -> (
		GivenSubjectAnyPredicateAnyObject<U>,
		GivenSubjectAnyPredicateAnyObject<V>,
	) {
		match self {
			Self::AnyGraph => (
				GivenSubjectAnyPredicateAnyObject::AnyGraph,
				GivenSubjectAnyPredicateAnyObject::AnyGraph,
			),
			Self::SameAsPredicate => (
				GivenSubjectAnyPredicateAnyObject::SameAsPredicate,
				GivenSubjectAnyPredicateAnyObject::SameAsPredicate,
			),
			Self::SameAsObject => (
				GivenSubjectAnyPredicateAnyObject::SameAsObject,
				GivenSubjectAnyPredicateAnyObject::SameAsObject,
			),
			Self::GivenGraph(g) => {
				let (g_u, g_v) = match g {
					Some(g) => {
						let (g_u, g_v) = f(g);
						(Some(g_u), Some(g_v))
					}
					None => (None, None),
				};
				(
					GivenSubjectAnyPredicateAnyObject::GivenGraph(g_u),
					GivenSubjectAnyPredicateAnyObject::GivenGraph(g_v),
				)
			}
		}
	}

	pub fn set_predicate(self, p: T) -> GivenSubjectGivenPredicateAnyObject<T> {
		match self {
			Self::AnyGraph => GivenSubjectGivenPredicateAnyObject::AnyGraph,
			Self::SameAsPredicate => GivenSubjectGivenPredicateAnyObject::GivenGraph(Some(p)),
			Self::SameAsObject => GivenSubjectGivenPredicateAnyObject::SameAsObject,
			Self::GivenGraph(g) => GivenSubjectGivenPredicateAnyObject::GivenGraph(g),
		}
	}

	pub fn set_object(self, o: T) -> GivenSubjectAnyPredicateGivenObject<T> {
		match self {
			Self::AnyGraph => GivenSubjectAnyPredicateGivenObject::AnyGraph,
			Self::SameAsPredicate => GivenSubjectAnyPredicateGivenObject::SameAsPredicate,
			Self::SameAsObject => GivenSubjectAnyPredicateGivenObject::GivenGraph(Some(o)),
			Self::GivenGraph(g) => GivenSubjectAnyPredicateGivenObject::GivenGraph(g),
		}
	}

	pub fn set_graph(&mut self, g: Option<T>) -> PatternGraph<T> {
		std::mem::replace(self, Self::GivenGraph(g)).into_graph()
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

	pub fn from_pattern<X: PartialEq>(s: X, o: X, g: Option<Pattern<T, X>>) -> Self {
		match g {
			None => Self::GivenGraph(None),
			Some(Pattern::Ground(g)) => Self::GivenGraph(Some(g)),
			Some(Pattern::Var(g)) => {
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

	pub fn matches(&self, quad: Quad<T>) -> bool
	where
		T: PartialEq,
	{
		match self {
			Self::AnyGraph => true,
			Self::SameAsSubject => quad.graph() == Some(quad.subject()),
			Self::SameAsObject => quad.graph() == Some(quad.object()),
			Self::GivenGraph(g) => quad.graph() == g.as_ref(),
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

	pub fn as_ref(&self) -> AnySubjectGivenPredicateAnyObject<&T> {
		match self {
			Self::AnyGraph => AnySubjectGivenPredicateAnyObject::AnyGraph,
			Self::SameAsSubject => AnySubjectGivenPredicateAnyObject::SameAsSubject,
			Self::SameAsObject => AnySubjectGivenPredicateAnyObject::SameAsObject,
			Self::GivenGraph(g) => AnySubjectGivenPredicateAnyObject::GivenGraph(g.as_ref()),
		}
	}

	pub fn map<U>(self, f: impl FnMut(T) -> U) -> AnySubjectGivenPredicateAnyObject<U> {
		match self {
			Self::AnyGraph => AnySubjectGivenPredicateAnyObject::AnyGraph,
			Self::SameAsSubject => AnySubjectGivenPredicateAnyObject::SameAsSubject,
			Self::SameAsObject => AnySubjectGivenPredicateAnyObject::SameAsObject,
			Self::GivenGraph(g) => AnySubjectGivenPredicateAnyObject::GivenGraph(g.map(f)),
		}
	}

	pub fn map2<U, V>(
		self,
		mut f: impl FnMut(T) -> (U, V),
	) -> (
		AnySubjectGivenPredicateAnyObject<U>,
		AnySubjectGivenPredicateAnyObject<V>,
	) {
		match self {
			Self::AnyGraph => (
				AnySubjectGivenPredicateAnyObject::AnyGraph,
				AnySubjectGivenPredicateAnyObject::AnyGraph,
			),
			Self::SameAsSubject => (
				AnySubjectGivenPredicateAnyObject::SameAsSubject,
				AnySubjectGivenPredicateAnyObject::SameAsSubject,
			),
			Self::SameAsObject => (
				AnySubjectGivenPredicateAnyObject::SameAsObject,
				AnySubjectGivenPredicateAnyObject::SameAsObject,
			),
			Self::GivenGraph(g) => {
				let (g_u, g_v) = match g {
					Some(g) => {
						let (g_u, g_v) = f(g);
						(Some(g_u), Some(g_v))
					}
					None => (None, None),
				};
				(
					AnySubjectGivenPredicateAnyObject::GivenGraph(g_u),
					AnySubjectGivenPredicateAnyObject::GivenGraph(g_v),
				)
			}
		}
	}

	pub fn set_subject(self, s: T) -> GivenSubjectGivenPredicateAnyObject<T> {
		match self {
			Self::AnyGraph => GivenSubjectGivenPredicateAnyObject::AnyGraph,
			Self::SameAsSubject => GivenSubjectGivenPredicateAnyObject::GivenGraph(Some(s)),
			Self::SameAsObject => GivenSubjectGivenPredicateAnyObject::SameAsObject,
			Self::GivenGraph(g) => GivenSubjectGivenPredicateAnyObject::GivenGraph(g),
		}
	}

	pub fn set_object(self, o: T) -> AnySubjectGivenPredicateGivenObject<T> {
		match self {
			Self::AnyGraph => AnySubjectGivenPredicateGivenObject::AnyGraph,
			Self::SameAsSubject => AnySubjectGivenPredicateGivenObject::SameAsSubject,
			Self::SameAsObject => AnySubjectGivenPredicateGivenObject::GivenGraph(Some(o)),
			Self::GivenGraph(g) => AnySubjectGivenPredicateGivenObject::GivenGraph(g),
		}
	}

	pub fn set_graph(&mut self, g: Option<T>) -> PatternGraph<T> {
		std::mem::replace(self, Self::GivenGraph(g)).into_graph()
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

	pub fn from_pattern<X: PartialEq>(s: X, p: X, g: Option<Pattern<T, X>>) -> Self {
		match g {
			None => Self::GivenGraph(None),
			Some(Pattern::Ground(g)) => Self::GivenGraph(Some(g)),
			Some(Pattern::Var(g)) => {
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

	pub fn matches(&self, quad: Quad<T>) -> bool
	where
		T: PartialEq,
	{
		match self {
			Self::AnyGraph => true,
			Self::SameAsSubject => quad.graph() == Some(quad.subject()),
			Self::SameAsPredicate => quad.graph() == Some(quad.predicate()),
			Self::GivenGraph(g) => quad.graph() == g.as_ref(),
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

	pub fn as_ref(&self) -> AnySubjectAnyPredicateGivenObject<&T> {
		match self {
			Self::AnyGraph => AnySubjectAnyPredicateGivenObject::AnyGraph,
			Self::SameAsSubject => AnySubjectAnyPredicateGivenObject::SameAsSubject,
			Self::SameAsPredicate => AnySubjectAnyPredicateGivenObject::SameAsPredicate,
			Self::GivenGraph(g) => AnySubjectAnyPredicateGivenObject::GivenGraph(g.as_ref()),
		}
	}

	pub fn map<U>(self, f: impl FnMut(T) -> U) -> AnySubjectAnyPredicateGivenObject<U> {
		match self {
			Self::AnyGraph => AnySubjectAnyPredicateGivenObject::AnyGraph,
			Self::SameAsSubject => AnySubjectAnyPredicateGivenObject::SameAsSubject,
			Self::SameAsPredicate => AnySubjectAnyPredicateGivenObject::SameAsPredicate,
			Self::GivenGraph(g) => AnySubjectAnyPredicateGivenObject::GivenGraph(g.map(f)),
		}
	}

	pub fn map2<U, V>(
		self,
		mut f: impl FnMut(T) -> (U, V),
	) -> (
		AnySubjectAnyPredicateGivenObject<U>,
		AnySubjectAnyPredicateGivenObject<V>,
	) {
		match self {
			Self::AnyGraph => (
				AnySubjectAnyPredicateGivenObject::AnyGraph,
				AnySubjectAnyPredicateGivenObject::AnyGraph,
			),
			Self::SameAsSubject => (
				AnySubjectAnyPredicateGivenObject::SameAsSubject,
				AnySubjectAnyPredicateGivenObject::SameAsSubject,
			),
			Self::SameAsPredicate => (
				AnySubjectAnyPredicateGivenObject::SameAsPredicate,
				AnySubjectAnyPredicateGivenObject::SameAsPredicate,
			),
			Self::GivenGraph(g) => {
				let (g_u, g_v) = match g {
					Some(g) => {
						let (g_u, g_v) = f(g);
						(Some(g_u), Some(g_v))
					}
					None => (None, None),
				};
				(
					AnySubjectAnyPredicateGivenObject::GivenGraph(g_u),
					AnySubjectAnyPredicateGivenObject::GivenGraph(g_v),
				)
			}
		}
	}

	pub fn set_subject(self, s: T) -> GivenSubjectAnyPredicateGivenObject<T> {
		match self {
			Self::AnyGraph => GivenSubjectAnyPredicateGivenObject::AnyGraph,
			Self::SameAsSubject => GivenSubjectAnyPredicateGivenObject::GivenGraph(Some(s)),
			Self::SameAsPredicate => GivenSubjectAnyPredicateGivenObject::SameAsPredicate,
			Self::GivenGraph(g) => GivenSubjectAnyPredicateGivenObject::GivenGraph(g),
		}
	}

	pub fn set_predicate(self, p: T) -> AnySubjectGivenPredicateGivenObject<T> {
		match self {
			Self::AnyGraph => AnySubjectGivenPredicateGivenObject::AnyGraph,
			Self::SameAsSubject => AnySubjectGivenPredicateGivenObject::SameAsSubject,
			Self::SameAsPredicate => AnySubjectGivenPredicateGivenObject::GivenGraph(Some(p)),
			Self::GivenGraph(g) => AnySubjectGivenPredicateGivenObject::GivenGraph(g),
		}
	}

	pub fn set_graph(&mut self, g: Option<T>) -> PatternGraph<T> {
		std::mem::replace(self, Self::GivenGraph(g)).into_graph()
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

	pub fn from_pattern<X: PartialEq>(s: X, g: Option<Pattern<T, X>>) -> Self {
		match g {
			None => Self::GivenGraph(None),
			Some(Pattern::Ground(g)) => Self::GivenGraph(Some(g)),
			Some(Pattern::Var(g)) => {
				if g == s {
					Self::SameAsSubject
				} else {
					Self::AnyGraph
				}
			}
		}
	}

	pub fn matches(&self, quad: Quad<T>) -> bool
	where
		T: PartialEq,
	{
		match self {
			Self::AnyGraph => true,
			Self::SameAsSubject => quad.graph() == Some(quad.subject()),
			Self::GivenGraph(g) => quad.graph() == g.as_ref(),
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

	pub fn as_ref(&self) -> AnySubjectGivenPredicateGivenObject<&T> {
		match self {
			Self::AnyGraph => AnySubjectGivenPredicateGivenObject::AnyGraph,
			Self::SameAsSubject => AnySubjectGivenPredicateGivenObject::SameAsSubject,
			Self::GivenGraph(g) => AnySubjectGivenPredicateGivenObject::GivenGraph(g.as_ref()),
		}
	}

	pub fn map<U>(self, f: impl FnMut(T) -> U) -> AnySubjectGivenPredicateGivenObject<U> {
		match self {
			Self::AnyGraph => AnySubjectGivenPredicateGivenObject::AnyGraph,
			Self::SameAsSubject => AnySubjectGivenPredicateGivenObject::SameAsSubject,
			Self::GivenGraph(g) => AnySubjectGivenPredicateGivenObject::GivenGraph(g.map(f)),
		}
	}

	pub fn map2<U, V>(
		self,
		mut f: impl FnMut(T) -> (U, V),
	) -> (
		AnySubjectGivenPredicateGivenObject<U>,
		AnySubjectGivenPredicateGivenObject<V>,
	) {
		match self {
			Self::AnyGraph => (
				AnySubjectGivenPredicateGivenObject::AnyGraph,
				AnySubjectGivenPredicateGivenObject::AnyGraph,
			),
			Self::SameAsSubject => (
				AnySubjectGivenPredicateGivenObject::SameAsSubject,
				AnySubjectGivenPredicateGivenObject::SameAsSubject,
			),
			Self::GivenGraph(g) => {
				let (g_u, g_v) = match g {
					Some(g) => {
						let (g_u, g_v) = f(g);
						(Some(g_u), Some(g_v))
					}
					None => (None, None),
				};
				(
					AnySubjectGivenPredicateGivenObject::GivenGraph(g_u),
					AnySubjectGivenPredicateGivenObject::GivenGraph(g_v),
				)
			}
		}
	}

	pub fn set_subject(self, s: T) -> GivenSubjectGivenPredicateGivenObject<T> {
		match self {
			Self::AnyGraph => GivenSubjectGivenPredicateGivenObject::AnyGraph,
			Self::SameAsSubject => GivenSubjectGivenPredicateGivenObject::GivenGraph(Some(s)),
			Self::GivenGraph(g) => GivenSubjectGivenPredicateGivenObject::GivenGraph(g),
		}
	}

	pub fn set_graph(&mut self, g: Option<T>) -> PatternGraph<T> {
		std::mem::replace(self, Self::GivenGraph(g)).into_graph()
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

	pub fn from_pattern<X: PartialEq>(p: X, g: Option<Pattern<T, X>>) -> Self {
		match g {
			None => Self::GivenGraph(None),
			Some(Pattern::Ground(g)) => Self::GivenGraph(Some(g)),
			Some(Pattern::Var(g)) => {
				if g == p {
					Self::SameAsPredicate
				} else {
					Self::AnyGraph
				}
			}
		}
	}

	pub fn matches(&self, quad: Quad<T>) -> bool
	where
		T: PartialEq,
	{
		match self {
			Self::AnyGraph => true,
			Self::SameAsPredicate => quad.graph() == Some(quad.predicate()),
			Self::GivenGraph(g) => quad.graph() == g.as_ref(),
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

	pub fn as_ref(&self) -> GivenSubjectAnyPredicateGivenObject<&T> {
		match self {
			Self::AnyGraph => GivenSubjectAnyPredicateGivenObject::AnyGraph,
			Self::SameAsPredicate => GivenSubjectAnyPredicateGivenObject::SameAsPredicate,
			Self::GivenGraph(g) => GivenSubjectAnyPredicateGivenObject::GivenGraph(g.as_ref()),
		}
	}

	pub fn map<U>(self, f: impl FnMut(T) -> U) -> GivenSubjectAnyPredicateGivenObject<U> {
		match self {
			Self::AnyGraph => GivenSubjectAnyPredicateGivenObject::AnyGraph,
			Self::SameAsPredicate => GivenSubjectAnyPredicateGivenObject::SameAsPredicate,
			Self::GivenGraph(g) => GivenSubjectAnyPredicateGivenObject::GivenGraph(g.map(f)),
		}
	}

	pub fn map2<U, V>(
		self,
		mut f: impl FnMut(T) -> (U, V),
	) -> (
		GivenSubjectAnyPredicateGivenObject<U>,
		GivenSubjectAnyPredicateGivenObject<V>,
	) {
		match self {
			Self::AnyGraph => (
				GivenSubjectAnyPredicateGivenObject::AnyGraph,
				GivenSubjectAnyPredicateGivenObject::AnyGraph,
			),
			Self::SameAsPredicate => (
				GivenSubjectAnyPredicateGivenObject::SameAsPredicate,
				GivenSubjectAnyPredicateGivenObject::SameAsPredicate,
			),
			Self::GivenGraph(g) => {
				let (g_u, g_v) = match g {
					Some(g) => {
						let (g_u, g_v) = f(g);
						(Some(g_u), Some(g_v))
					}
					None => (None, None),
				};
				(
					GivenSubjectAnyPredicateGivenObject::GivenGraph(g_u),
					GivenSubjectAnyPredicateGivenObject::GivenGraph(g_v),
				)
			}
		}
	}

	pub fn set_predicate(self, p: T) -> GivenSubjectGivenPredicateGivenObject<T> {
		match self {
			Self::AnyGraph => GivenSubjectGivenPredicateGivenObject::AnyGraph,
			Self::SameAsPredicate => GivenSubjectGivenPredicateGivenObject::GivenGraph(Some(p)),
			Self::GivenGraph(g) => GivenSubjectGivenPredicateGivenObject::GivenGraph(g),
		}
	}

	pub fn set_graph(&mut self, g: Option<T>) -> PatternGraph<T> {
		std::mem::replace(self, Self::GivenGraph(g)).into_graph()
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

	pub fn from_pattern<X: PartialEq>(o: X, g: Option<Pattern<T, X>>) -> Self {
		match g {
			None => Self::GivenGraph(None),
			Some(Pattern::Ground(g)) => Self::GivenGraph(Some(g)),
			Some(Pattern::Var(g)) => {
				if g == o {
					Self::SameAsObject
				} else {
					Self::AnyGraph
				}
			}
		}
	}

	pub fn matches(&self, quad: Quad<T>) -> bool
	where
		T: PartialEq,
	{
		match self {
			Self::AnyGraph => true,
			Self::SameAsObject => quad.graph() == Some(quad.object()),
			Self::GivenGraph(g) => quad.graph() == g.as_ref(),
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

	pub fn as_ref(&self) -> GivenSubjectGivenPredicateAnyObject<&T> {
		match self {
			Self::AnyGraph => GivenSubjectGivenPredicateAnyObject::AnyGraph,
			Self::SameAsObject => GivenSubjectGivenPredicateAnyObject::SameAsObject,
			Self::GivenGraph(g) => GivenSubjectGivenPredicateAnyObject::GivenGraph(g.as_ref()),
		}
	}

	pub fn map<U>(self, f: impl FnMut(T) -> U) -> GivenSubjectGivenPredicateAnyObject<U> {
		match self {
			Self::AnyGraph => GivenSubjectGivenPredicateAnyObject::AnyGraph,
			Self::SameAsObject => GivenSubjectGivenPredicateAnyObject::SameAsObject,
			Self::GivenGraph(g) => GivenSubjectGivenPredicateAnyObject::GivenGraph(g.map(f)),
		}
	}

	pub fn map2<U, V>(
		self,
		mut f: impl FnMut(T) -> (U, V),
	) -> (
		GivenSubjectGivenPredicateAnyObject<U>,
		GivenSubjectGivenPredicateAnyObject<V>,
	) {
		match self {
			Self::AnyGraph => (
				GivenSubjectGivenPredicateAnyObject::AnyGraph,
				GivenSubjectGivenPredicateAnyObject::AnyGraph,
			),
			Self::SameAsObject => (
				GivenSubjectGivenPredicateAnyObject::SameAsObject,
				GivenSubjectGivenPredicateAnyObject::SameAsObject,
			),
			Self::GivenGraph(g) => {
				let (g_u, g_v) = match g {
					Some(g) => {
						let (g_u, g_v) = f(g);
						(Some(g_u), Some(g_v))
					}
					None => (None, None),
				};
				(
					GivenSubjectGivenPredicateAnyObject::GivenGraph(g_u),
					GivenSubjectGivenPredicateAnyObject::GivenGraph(g_v),
				)
			}
		}
	}

	pub fn set_object(self, o: T) -> GivenSubjectGivenPredicateGivenObject<T> {
		match self {
			Self::AnyGraph => GivenSubjectGivenPredicateGivenObject::AnyGraph,
			Self::SameAsObject => GivenSubjectGivenPredicateGivenObject::GivenGraph(Some(o)),
			Self::GivenGraph(g) => GivenSubjectGivenPredicateGivenObject::GivenGraph(g),
		}
	}

	pub fn set_graph(&mut self, g: Option<T>) -> PatternGraph<T> {
		std::mem::replace(self, Self::GivenGraph(g)).into_graph()
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

	pub fn from_pattern<X>(g: Option<Pattern<T, X>>) -> Self {
		match g {
			None => Self::GivenGraph(None),
			Some(Pattern::Ground(g)) => Self::GivenGraph(Some(g)),
			Some(Pattern::Var(_)) => Self::AnyGraph,
		}
	}

	pub fn matches(&self, quad: Quad<T>) -> bool
	where
		T: PartialEq,
	{
		match self {
			Self::AnyGraph => true,
			Self::GivenGraph(g) => quad.graph() == g.as_ref(),
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

	pub fn as_ref(&self) -> GivenSubjectGivenPredicateGivenObject<&T> {
		match self {
			Self::AnyGraph => GivenSubjectGivenPredicateGivenObject::AnyGraph,
			Self::GivenGraph(g) => GivenSubjectGivenPredicateGivenObject::GivenGraph(g.as_ref()),
		}
	}

	pub fn map<U>(self, f: impl FnMut(T) -> U) -> GivenSubjectGivenPredicateGivenObject<U> {
		match self {
			Self::AnyGraph => GivenSubjectGivenPredicateGivenObject::AnyGraph,
			Self::GivenGraph(g) => GivenSubjectGivenPredicateGivenObject::GivenGraph(g.map(f)),
		}
	}

	pub fn map2<U, V>(
		self,
		mut f: impl FnMut(T) -> (U, V),
	) -> (
		GivenSubjectGivenPredicateGivenObject<U>,
		GivenSubjectGivenPredicateGivenObject<V>,
	) {
		match self {
			Self::AnyGraph => (
				GivenSubjectGivenPredicateGivenObject::AnyGraph,
				GivenSubjectGivenPredicateGivenObject::AnyGraph,
			),
			Self::GivenGraph(g) => {
				let (g_u, g_v) = match g {
					Some(g) => {
						let (g_u, g_v) = f(g);
						(Some(g_u), Some(g_v))
					}
					None => (None, None),
				};
				(
					GivenSubjectGivenPredicateGivenObject::GivenGraph(g_u),
					GivenSubjectGivenPredicateGivenObject::GivenGraph(g_v),
				)
			}
		}
	}

	pub fn set_graph(&mut self, g: Option<T>) -> PatternGraph<T> {
		std::mem::replace(self, Self::GivenGraph(g)).into_graph()
	}
}

// --- as_deref impls ---

impl<T: std::ops::Deref> CanonicalQuadPattern<T> {
	pub fn as_deref(&self) -> CanonicalQuadPattern<&T::Target> {
		match self {
			Self::AnySubject(p) => CanonicalQuadPattern::AnySubject(p.as_deref()),
			Self::GivenSubject(s, p) => CanonicalQuadPattern::GivenSubject(&**s, p.as_deref()),
		}
	}
}

impl<T: std::ops::Deref> AnySubject<T> {
	pub fn as_deref(&self) -> AnySubject<&T::Target> {
		match self {
			Self::AnyPredicate(p) => AnySubject::AnyPredicate(p.as_deref()),
			Self::SameAsSubject(p) => AnySubject::SameAsSubject(p.as_deref()),
			Self::GivenPredicate(t, p) => AnySubject::GivenPredicate(&**t, p.as_deref()),
		}
	}
}

impl<T: std::ops::Deref> AnySubjectAnyPredicate<T> {
	pub fn as_deref(&self) -> AnySubjectAnyPredicate<&T::Target> {
		match self {
			Self::AnyObject(p) => AnySubjectAnyPredicate::AnyObject(p.as_deref()),
			Self::SameAsSubject(p) => AnySubjectAnyPredicate::SameAsSubject(p.as_deref()),
			Self::SameAsPredicate(p) => AnySubjectAnyPredicate::SameAsPredicate(p.as_deref()),
			Self::GivenObject(t, p) => AnySubjectAnyPredicate::GivenObject(&**t, p.as_deref()),
		}
	}
}

impl<T: std::ops::Deref> AnySubjectGivenPredicate<T> {
	pub fn as_deref(&self) -> AnySubjectGivenPredicate<&T::Target> {
		match self {
			Self::AnyObject(p) => AnySubjectGivenPredicate::AnyObject(p.as_deref()),
			Self::SameAsSubject(p) => AnySubjectGivenPredicate::SameAsSubject(p.as_deref()),
			Self::GivenObject(t, p) => AnySubjectGivenPredicate::GivenObject(&**t, p.as_deref()),
		}
	}
}

impl<T: std::ops::Deref> GivenSubject<T> {
	pub fn as_deref(&self) -> GivenSubject<&T::Target> {
		match self {
			Self::AnyPredicate(p) => GivenSubject::AnyPredicate(p.as_deref()),
			Self::GivenPredicate(t, p) => GivenSubject::GivenPredicate(&**t, p.as_deref()),
		}
	}
}

impl<T: std::ops::Deref> GivenSubjectAnyPredicate<T> {
	pub fn as_deref(&self) -> GivenSubjectAnyPredicate<&T::Target> {
		match self {
			Self::AnyObject(p) => GivenSubjectAnyPredicate::AnyObject(p.as_deref()),
			Self::SameAsPredicate(p) => GivenSubjectAnyPredicate::SameAsPredicate(p.as_deref()),
			Self::GivenObject(t, p) => GivenSubjectAnyPredicate::GivenObject(&**t, p.as_deref()),
		}
	}
}

impl<T: std::ops::Deref> GivenSubjectGivenPredicate<T> {
	pub fn as_deref(&self) -> GivenSubjectGivenPredicate<&T::Target> {
		match self {
			Self::AnyObject(p) => GivenSubjectGivenPredicate::AnyObject(p.as_deref()),
			Self::GivenObject(t, p) => GivenSubjectGivenPredicate::GivenObject(&**t, p.as_deref()),
		}
	}
}

fn option_as_deref<T: std::ops::Deref>(o: &Option<T>) -> Option<&T::Target> {
	o.as_ref().map(|t| &**t)
}

impl<T: std::ops::Deref> AnySubjectAnyPredicateAnyObject<T> {
	pub fn as_deref(&self) -> AnySubjectAnyPredicateAnyObject<&T::Target> {
		match self {
			Self::AnyGraph => AnySubjectAnyPredicateAnyObject::AnyGraph,
			Self::SameAsSubject => AnySubjectAnyPredicateAnyObject::SameAsSubject,
			Self::SameAsPredicate => AnySubjectAnyPredicateAnyObject::SameAsPredicate,
			Self::SameAsObject => AnySubjectAnyPredicateAnyObject::SameAsObject,
			Self::GivenGraph(g) => AnySubjectAnyPredicateAnyObject::GivenGraph(option_as_deref(g)),
		}
	}
}

impl<T: std::ops::Deref> GivenSubjectAnyPredicateAnyObject<T> {
	pub fn as_deref(&self) -> GivenSubjectAnyPredicateAnyObject<&T::Target> {
		match self {
			Self::AnyGraph => GivenSubjectAnyPredicateAnyObject::AnyGraph,
			Self::SameAsPredicate => GivenSubjectAnyPredicateAnyObject::SameAsPredicate,
			Self::SameAsObject => GivenSubjectAnyPredicateAnyObject::SameAsObject,
			Self::GivenGraph(g) => {
				GivenSubjectAnyPredicateAnyObject::GivenGraph(option_as_deref(g))
			}
		}
	}
}

impl<T: std::ops::Deref> AnySubjectGivenPredicateAnyObject<T> {
	pub fn as_deref(&self) -> AnySubjectGivenPredicateAnyObject<&T::Target> {
		match self {
			Self::AnyGraph => AnySubjectGivenPredicateAnyObject::AnyGraph,
			Self::SameAsSubject => AnySubjectGivenPredicateAnyObject::SameAsSubject,
			Self::SameAsObject => AnySubjectGivenPredicateAnyObject::SameAsObject,
			Self::GivenGraph(g) => {
				AnySubjectGivenPredicateAnyObject::GivenGraph(option_as_deref(g))
			}
		}
	}
}

impl<T: std::ops::Deref> AnySubjectAnyPredicateGivenObject<T> {
	pub fn as_deref(&self) -> AnySubjectAnyPredicateGivenObject<&T::Target> {
		match self {
			Self::AnyGraph => AnySubjectAnyPredicateGivenObject::AnyGraph,
			Self::SameAsSubject => AnySubjectAnyPredicateGivenObject::SameAsSubject,
			Self::SameAsPredicate => AnySubjectAnyPredicateGivenObject::SameAsPredicate,
			Self::GivenGraph(g) => {
				AnySubjectAnyPredicateGivenObject::GivenGraph(option_as_deref(g))
			}
		}
	}
}

impl<T: std::ops::Deref> AnySubjectGivenPredicateGivenObject<T> {
	pub fn as_deref(&self) -> AnySubjectGivenPredicateGivenObject<&T::Target> {
		match self {
			Self::AnyGraph => AnySubjectGivenPredicateGivenObject::AnyGraph,
			Self::SameAsSubject => AnySubjectGivenPredicateGivenObject::SameAsSubject,
			Self::GivenGraph(g) => {
				AnySubjectGivenPredicateGivenObject::GivenGraph(option_as_deref(g))
			}
		}
	}
}

impl<T: std::ops::Deref> GivenSubjectAnyPredicateGivenObject<T> {
	pub fn as_deref(&self) -> GivenSubjectAnyPredicateGivenObject<&T::Target> {
		match self {
			Self::AnyGraph => GivenSubjectAnyPredicateGivenObject::AnyGraph,
			Self::SameAsPredicate => GivenSubjectAnyPredicateGivenObject::SameAsPredicate,
			Self::GivenGraph(g) => {
				GivenSubjectAnyPredicateGivenObject::GivenGraph(option_as_deref(g))
			}
		}
	}
}

impl<T: std::ops::Deref> GivenSubjectGivenPredicateAnyObject<T> {
	pub fn as_deref(&self) -> GivenSubjectGivenPredicateAnyObject<&T::Target> {
		match self {
			Self::AnyGraph => GivenSubjectGivenPredicateAnyObject::AnyGraph,
			Self::SameAsObject => GivenSubjectGivenPredicateAnyObject::SameAsObject,
			Self::GivenGraph(g) => {
				GivenSubjectGivenPredicateAnyObject::GivenGraph(option_as_deref(g))
			}
		}
	}
}

impl<T: std::ops::Deref> GivenSubjectGivenPredicateGivenObject<T> {
	pub fn as_deref(&self) -> GivenSubjectGivenPredicateGivenObject<&T::Target> {
		match self {
			Self::AnyGraph => GivenSubjectGivenPredicateGivenObject::AnyGraph,
			Self::GivenGraph(g) => {
				GivenSubjectGivenPredicateGivenObject::GivenGraph(option_as_deref(g))
			}
		}
	}
}

#[cfg(test)]
mod mutation_tests {
	use super::*;
	use crate::Quad;

	fn pattern(
		s: Pattern<i32, u8>,
		p: Pattern<i32, u8>,
		o: Pattern<i32, u8>,
		g: Option<Pattern<i32, u8>>,
	) -> CanonicalQuadPattern<i32> {
		CanonicalQuadPattern::from_pattern(Quad(s, p, o, g))
	}

	#[test]
	fn set_predicate_propagates_to_subject() {
		// ?0 ?0 ?1 ?2 (subject == predicate)
		let mut pat = pattern(
			Pattern::Var(0),
			Pattern::Var(0),
			Pattern::Var(1),
			Some(Pattern::Var(2)),
		);
		let old = pat.set_predicate(42);
		assert_eq!(old, PatternPredicate::SameAsSubject);
		assert_eq!(pat.subject(), PatternSubject::Given(&42));
		assert_eq!(pat.predicate(), PatternPredicate::Given(&42));
		assert_eq!(pat.object(), PatternObject::Any);
		assert_eq!(pat.graph(), PatternGraph::Any);
	}

	#[test]
	fn set_subject_propagates_to_predicate() {
		// ?0 ?0 ?1 ?2 (subject == predicate)
		let mut pat = pattern(
			Pattern::Var(0),
			Pattern::Var(0),
			Pattern::Var(1),
			Some(Pattern::Var(2)),
		);
		let old = pat.set_subject(7);
		assert_eq!(old, PatternSubject::Any);
		assert_eq!(pat.subject(), PatternSubject::Given(&7));
		assert_eq!(pat.predicate(), PatternPredicate::Given(&7));
	}

	#[test]
	fn set_subject_propagates_to_object() {
		// ?0 ?1 ?0 ?2 (object == subject, predicate independent)
		let mut pat = pattern(
			Pattern::Var(0),
			Pattern::Var(1),
			Pattern::Var(0),
			Some(Pattern::Var(2)),
		);
		let old = pat.set_subject(9);
		assert_eq!(old, PatternSubject::Any);
		assert_eq!(pat.subject(), PatternSubject::Given(&9));
		assert_eq!(pat.object(), PatternObject::Given(&9));
		assert_eq!(pat.predicate(), PatternPredicate::Any);
	}

	#[test]
	fn set_predicate_does_not_disturb_unrelated_same_as_subject_object() {
		// ?0 ?1 ?0 ?2 (object == subject, predicate independent): setting the
		// predicate must NOT collapse the object's `SameAsSubject` relation
		// down to `Any` (regression test for the fixed bug class).
		let mut pat = pattern(
			Pattern::Var(0),
			Pattern::Var(1),
			Pattern::Var(0),
			Some(Pattern::Var(2)),
		);
		let old = pat.set_predicate(5);
		assert_eq!(old, PatternPredicate::Any);
		assert_eq!(pat.predicate(), PatternPredicate::Given(&5));
		assert_eq!(pat.object(), PatternObject::SameAsSubject);
	}

	#[test]
	fn set_predicate_propagates_to_object() {
		// ?0 ?1 ?1 ?2 (object == predicate)
		let mut pat = pattern(
			Pattern::Var(0),
			Pattern::Var(1),
			Pattern::Var(1),
			Some(Pattern::Var(2)),
		);
		let old = pat.set_predicate(3);
		assert_eq!(old, PatternPredicate::Any);
		assert_eq!(pat.predicate(), PatternPredicate::Given(&3));
		assert_eq!(pat.object(), PatternObject::Given(&3));
	}

	#[test]
	fn set_object_propagates_to_predicate() {
		// ?0 ?1 ?1 ?2 (object == predicate)
		let mut pat = pattern(
			Pattern::Var(0),
			Pattern::Var(1),
			Pattern::Var(1),
			Some(Pattern::Var(2)),
		);
		let old = pat.set_object(11);
		assert_eq!(old, PatternObject::Any);
		assert_eq!(pat.predicate(), PatternPredicate::Given(&11));
		assert_eq!(pat.object(), PatternObject::Given(&11));
	}

	#[test]
	fn set_subject_propagates_to_graph() {
		// ?0 ?1 ?2 ?0 (graph == subject)
		let mut pat = pattern(
			Pattern::Var(0),
			Pattern::Var(1),
			Pattern::Var(2),
			Some(Pattern::Var(0)),
		);
		let old = pat.set_subject(13);
		assert_eq!(old, PatternSubject::Any);
		assert_eq!(pat.graph(), PatternGraph::Given(Some(&13)));
	}

	#[test]
	fn set_predicate_propagates_to_graph() {
		// ?0 ?1 ?2 ?1 (graph == predicate)
		let mut pat = pattern(
			Pattern::Var(0),
			Pattern::Var(1),
			Pattern::Var(2),
			Some(Pattern::Var(1)),
		);
		let old = pat.set_predicate(17);
		assert_eq!(old, PatternPredicate::Any);
		assert_eq!(pat.graph(), PatternGraph::Given(Some(&17)));
	}

	#[test]
	fn set_object_propagates_to_graph() {
		// ?0 ?1 ?2 ?2 (graph == object)
		let mut pat = pattern(
			Pattern::Var(0),
			Pattern::Var(1),
			Pattern::Var(2),
			Some(Pattern::Var(2)),
		);
		let old = pat.set_object(19);
		assert_eq!(old, PatternObject::Any);
		assert_eq!(pat.graph(), PatternGraph::Given(Some(&19)));
	}

	#[test]
	fn set_graph_does_not_affect_earlier_positions() {
		// ?0 ?1 ?2 ?2 (graph == object): setting the graph must not affect
		// subject/predicate/object at all.
		let mut pat = pattern(
			Pattern::Var(0),
			Pattern::Var(1),
			Pattern::Var(2),
			Some(Pattern::Var(2)),
		);
		let old = pat.set_graph(Some(23));
		assert_eq!(old, PatternGraph::SameAsObject);
		assert_eq!(pat.subject(), PatternSubject::Any);
		assert_eq!(pat.predicate(), PatternPredicate::Any);
		assert_eq!(pat.object(), PatternObject::Any);
		assert_eq!(pat.graph(), PatternGraph::Given(Some(&23)));
	}

	#[test]
	fn with_subject_with_predicate_with_object_with_graph() {
		let pat = CanonicalQuadPattern::<i32>::ANY
			.with_subject(1)
			.with_predicate(2)
			.with_object(3)
			.with_graph(Some(4));
		assert_eq!(
			pat.into_parts(),
			(
				PatternSubject::Given(1),
				PatternPredicate::Given(2),
				PatternObject::Given(3),
				PatternGraph::Given(Some(4)),
			)
		);
	}
}
