use std::borrow::Cow;

use replace_with::replace_with_or_abort_and_return;

use crate::{
	Triple,
	pattern::{CanonicalQuadPattern, Pattern, TriplePattern, quad},
};

/// Canonical triple pattern.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum CanonicalTriplePattern<T> {
	AnySubject(AnySubject<T>),
	GivenSubject(T, GivenSubject<T>),
}

impl<T> From<Triple<T>> for CanonicalTriplePattern<T> {
	fn from(value: Triple<T>) -> Self {
		Self::from_triple(value)
	}
}

impl<T> From<CanonicalTriplePattern<T>> for Triple<Option<T>> {
	fn from(value: CanonicalTriplePattern<T>) -> Self {
		value.into_triple()
	}
}

impl<T> From<Triple<Option<T>, Option<T>, Option<T>>> for CanonicalTriplePattern<T> {
	fn from(value: Triple<Option<T>, Option<T>, Option<T>>) -> Self {
		Self::from_option_triple(value)
	}
}

impl<T, X: PartialEq> From<TriplePattern<T, X>> for CanonicalTriplePattern<T> {
	fn from(value: TriplePattern<T, X>) -> Self {
		Self::from_pattern(value)
	}
}

impl<'a, T: ToOwned> From<Triple<&'a T>> for CanonicalTriplePattern<Cow<'a, T>> {
	fn from(value: Triple<&'a T>) -> Self {
		Self::from_triple(value.map(Cow::Borrowed))
	}
}

impl<'a, T: ToOwned> From<Triple<Option<&'a T>>> for CanonicalTriplePattern<Cow<'a, T>> {
	fn from(value: Triple<Option<&'a T>>) -> Self {
		Self::from_option_triple(value.map(|r| r.map(Cow::Borrowed)))
	}
}

impl<'a, T: ToOwned> From<CanonicalTriplePattern<&'a T>> for CanonicalTriplePattern<Cow<'a, T>> {
	fn from(value: CanonicalTriplePattern<&'a T>) -> Self {
		value.map(Cow::Borrowed)
	}
}

impl<T> CanonicalTriplePattern<T> {
	pub const ANY: Self =
		Self::AnySubject(AnySubject::AnyPredicate(AnySubjectAnyPredicate::AnyObject));

	pub fn matches(&self, triple: Triple<T>) -> bool
	where
		T: PartialEq,
	{
		match self {
			Self::AnySubject(p) => p.matches(triple),
			Self::GivenSubject(s, p) => triple.subject() == s && p.matches(triple),
		}
	}

	pub fn from_triple(triple: Triple<T>) -> Self {
		Self::GivenSubject(
			triple.0,
			GivenSubject::GivenPredicate(
				triple.1,
				GivenSubjectGivenPredicate::GivenObject(triple.2),
			),
		)
	}

	pub fn from_option_triple(triple: Triple<Option<T>, Option<T>, Option<T>>) -> Self {
		match triple.0 {
			Some(s) => Self::GivenSubject(s, GivenSubject::from_option_triple(triple.1, triple.2)),
			None => Self::AnySubject(AnySubject::from_option_triple(triple.1, triple.2)),
		}
	}

	pub fn from_pattern<X: PartialEq>(pattern: TriplePattern<T, X>) -> Self {
		match pattern.0 {
			Pattern::Ground(s) => {
				Self::GivenSubject(s, GivenSubject::from_pattern(pattern.1, pattern.2))
			}
			Pattern::Var(s) => Self::AnySubject(AnySubject::from_pattern(s, pattern.1, pattern.2)),
		}
	}

	pub fn with_any_graph(self) -> CanonicalQuadPattern<T> {
		match self {
			Self::AnySubject(p) => CanonicalQuadPattern::AnySubject(p.with_any_graph()),
			Self::GivenSubject(id, p) => CanonicalQuadPattern::GivenSubject(id, p.with_any_graph()),
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

	/// Converts the pattern into a triple of optional components.
	///
	/// Each `Any` or `SameAs*` component becomes `None`.
	pub fn into_triple(self) -> Triple<Option<T>> {
		match self {
			Self::AnySubject(p) => {
				let (pred, obj) = p.into_predicate_object();
				Triple(None, pred, obj)
			}
			Self::GivenSubject(s, p) => {
				let (pred, obj) = p.into_predicate_object();
				Triple(Some(s), pred, obj)
			}
		}
	}

	pub fn into_parts(self) -> (PatternSubject<T>, PatternPredicate<T>, PatternObject<T>) {
		match self {
			Self::AnySubject(pq) => {
				let (p, o) = pq.into_parts();
				(PatternSubject::Any, p, o)
			}
			Self::GivenSubject(s, pq) => {
				let (p, o) = pq.into_parts();
				(PatternSubject::Given(s), p, o)
			}
		}
	}

	pub fn as_ref(&self) -> CanonicalTriplePattern<&T> {
		match self {
			Self::AnySubject(p) => CanonicalTriplePattern::AnySubject(p.as_ref()),
			Self::GivenSubject(s, p) => CanonicalTriplePattern::GivenSubject(s, p.as_ref()),
		}
	}

	pub fn map<U>(self, mut f: impl FnMut(T) -> U) -> CanonicalTriplePattern<U> {
		match self {
			Self::AnySubject(pq) => CanonicalTriplePattern::AnySubject(pq.map(f)),
			Self::GivenSubject(s, pq) => CanonicalTriplePattern::GivenSubject(f(s), pq.map(f)),
		}
	}

	pub fn map2<U, V>(
		self,
		mut f: impl FnMut(T) -> (U, V),
	) -> (CanonicalTriplePattern<U>, CanonicalTriplePattern<V>) {
		match self {
			Self::AnySubject(pq) => {
				let (pq_u, pq_v) = pq.map2(f);
				(
					CanonicalTriplePattern::AnySubject(pq_u),
					CanonicalTriplePattern::AnySubject(pq_v),
				)
			}
			Self::GivenSubject(s, pq) => {
				let (u, v) = f(s);
				let (pq_u, pq_v) = pq.map2(f);
				(
					CanonicalTriplePattern::GivenSubject(u, pq_u),
					CanonicalTriplePattern::GivenSubject(v, pq_v),
				)
			}
		}
	}

	pub fn set_subject(&mut self, s: T) -> PatternSubject<T>
	where
		T: Clone,
	{
		replace_with_or_abort_and_return(self, |this| match this {
			Self::AnySubject(AnySubject::AnyPredicate(AnySubjectAnyPredicate::AnyObject)) => (
				PatternSubject::Any,
				Self::GivenSubject(
					s,
					GivenSubject::AnyPredicate(GivenSubjectAnyPredicate::AnyObject),
				),
			),
			Self::AnySubject(AnySubject::AnyPredicate(AnySubjectAnyPredicate::SameAsSubject)) => (
				PatternSubject::Any,
				Self::GivenSubject(
					s.clone(),
					GivenSubject::AnyPredicate(GivenSubjectAnyPredicate::GivenObject(s)),
				),
			),
			Self::AnySubject(AnySubject::AnyPredicate(AnySubjectAnyPredicate::SameAsPredicate)) => {
				(
					PatternSubject::Any,
					Self::GivenSubject(
						s,
						GivenSubject::AnyPredicate(GivenSubjectAnyPredicate::SameAsPredicate),
					),
				)
			}
			Self::AnySubject(AnySubject::AnyPredicate(AnySubjectAnyPredicate::GivenObject(o))) => (
				PatternSubject::Any,
				Self::GivenSubject(
					s,
					GivenSubject::AnyPredicate(GivenSubjectAnyPredicate::GivenObject(o)),
				),
			),
			Self::AnySubject(AnySubject::SameAsSubject(AnySubjectGivenPredicate::AnyObject)) => (
				PatternSubject::Any,
				Self::GivenSubject(
					s.clone(),
					GivenSubject::GivenPredicate(s, GivenSubjectGivenPredicate::AnyObject),
				),
			),
			Self::AnySubject(AnySubject::SameAsSubject(
				AnySubjectGivenPredicate::SameAsSubject,
			)) => (
				PatternSubject::Any,
				Self::GivenSubject(
					s.clone(),
					GivenSubject::GivenPredicate(
						s.clone(),
						GivenSubjectGivenPredicate::GivenObject(s),
					),
				),
			),
			Self::AnySubject(AnySubject::SameAsSubject(AnySubjectGivenPredicate::GivenObject(
				o,
			))) => (
				PatternSubject::Any,
				Self::GivenSubject(
					s.clone(),
					GivenSubject::GivenPredicate(s, GivenSubjectGivenPredicate::GivenObject(o)),
				),
			),
			Self::AnySubject(AnySubject::GivenPredicate(
				p,
				AnySubjectGivenPredicate::AnyObject,
			)) => (
				PatternSubject::Any,
				Self::GivenSubject(
					s,
					GivenSubject::GivenPredicate(p, GivenSubjectGivenPredicate::AnyObject),
				),
			),
			Self::AnySubject(AnySubject::GivenPredicate(
				p,
				AnySubjectGivenPredicate::SameAsSubject,
			)) => (
				PatternSubject::Any,
				Self::GivenSubject(
					s.clone(),
					GivenSubject::GivenPredicate(p, GivenSubjectGivenPredicate::GivenObject(s)),
				),
			),
			Self::AnySubject(AnySubject::GivenPredicate(
				p,
				AnySubjectGivenPredicate::GivenObject(o),
			)) => (
				PatternSubject::Any,
				Self::GivenSubject(
					s,
					GivenSubject::GivenPredicate(p, GivenSubjectGivenPredicate::GivenObject(o)),
				),
			),
			Self::GivenSubject(
				current_s,
				GivenSubject::AnyPredicate(GivenSubjectAnyPredicate::AnyObject),
			) => (
				PatternSubject::Given(current_s),
				Self::GivenSubject(
					s,
					GivenSubject::AnyPredicate(GivenSubjectAnyPredicate::AnyObject),
				),
			),
			Self::GivenSubject(
				current_s,
				GivenSubject::AnyPredicate(GivenSubjectAnyPredicate::SameAsPredicate),
			) => (
				PatternSubject::Given(current_s),
				Self::GivenSubject(
					s,
					GivenSubject::AnyPredicate(GivenSubjectAnyPredicate::SameAsPredicate),
				),
			),
			Self::GivenSubject(
				current_s,
				GivenSubject::AnyPredicate(GivenSubjectAnyPredicate::GivenObject(o)),
			) => (
				PatternSubject::Given(current_s),
				Self::GivenSubject(
					s,
					GivenSubject::AnyPredicate(GivenSubjectAnyPredicate::GivenObject(o)),
				),
			),
			Self::GivenSubject(
				current_s,
				GivenSubject::GivenPredicate(p, GivenSubjectGivenPredicate::AnyObject),
			) => (
				PatternSubject::Given(current_s),
				Self::GivenSubject(
					s,
					GivenSubject::GivenPredicate(p, GivenSubjectGivenPredicate::AnyObject),
				),
			),
			Self::GivenSubject(
				current_s,
				GivenSubject::GivenPredicate(p, GivenSubjectGivenPredicate::GivenObject(o)),
			) => (
				PatternSubject::Given(current_s),
				Self::GivenSubject(
					s,
					GivenSubject::GivenPredicate(p, GivenSubjectGivenPredicate::GivenObject(o)),
				),
			),
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
			Self::AnySubject(AnySubject::SameAsSubject(AnySubjectGivenPredicate::AnyObject)) => (
				PatternPredicate::SameAsSubject,
				Self::GivenSubject(
					p.clone(),
					GivenSubject::GivenPredicate(p, GivenSubjectGivenPredicate::AnyObject),
				),
			),
			Self::AnySubject(AnySubject::SameAsSubject(
				AnySubjectGivenPredicate::SameAsSubject,
			)) => (
				PatternPredicate::SameAsSubject,
				Self::GivenSubject(
					p.clone(),
					GivenSubject::GivenPredicate(
						p.clone(),
						GivenSubjectGivenPredicate::GivenObject(p),
					),
				),
			),
			Self::AnySubject(AnySubject::SameAsSubject(AnySubjectGivenPredicate::GivenObject(
				o,
			))) => (
				PatternPredicate::SameAsSubject,
				Self::GivenSubject(
					p.clone(),
					GivenSubject::GivenPredicate(p, GivenSubjectGivenPredicate::GivenObject(o)),
				),
			),
			Self::AnySubject(mut current_p) => {
				let old_p = current_p.set_predicate(p);
				(old_p, Self::AnySubject(current_p))
			}
			Self::GivenSubject(s, mut current_p) => {
				let old_p = current_p.set_predicate(p);
				(old_p, Self::GivenSubject(s, current_p))
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
			Self::AnySubject(AnySubject::AnyPredicate(AnySubjectAnyPredicate::SameAsSubject)) => (
				PatternObject::SameAsSubject,
				Self::GivenSubject(
					o.clone(),
					GivenSubject::AnyPredicate(GivenSubjectAnyPredicate::GivenObject(o)),
				),
			),
			Self::AnySubject(AnySubject::SameAsSubject(
				AnySubjectGivenPredicate::SameAsSubject,
			)) => (
				PatternObject::SameAsSubject,
				Self::GivenSubject(
					o.clone(),
					GivenSubject::GivenPredicate(
						o.clone(),
						GivenSubjectGivenPredicate::GivenObject(o),
					),
				),
			),
			Self::AnySubject(mut current_p) => {
				let old_p = current_p.set_object(o);
				(old_p, Self::AnySubject(current_p))
			}
			Self::GivenSubject(s, mut current_p) => {
				let old_p = current_p.set_object(o);
				(old_p, Self::GivenSubject(s, current_p))
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
}

impl<T: std::ops::Deref> CanonicalTriplePattern<T> {
	pub fn as_deref(&self) -> CanonicalTriplePattern<&T::Target> {
		match self {
			Self::AnySubject(p) => CanonicalTriplePattern::AnySubject(p.as_deref()),
			Self::GivenSubject(s, p) => CanonicalTriplePattern::GivenSubject(&**s, p.as_deref()),
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
	pub fn as_ref(&self) -> AnySubject<&T> {
		match self {
			Self::AnyPredicate(p) => AnySubject::AnyPredicate(p.as_ref()),
			Self::SameAsSubject(p) => AnySubject::SameAsSubject(p.as_ref()),
			Self::GivenPredicate(t, p) => AnySubject::GivenPredicate(t, p.as_ref()),
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

impl<T> AnySubject<T> {
	pub fn from_option_triple(p: Option<T>, o: Option<T>) -> Self {
		match p {
			Some(p) => Self::GivenPredicate(p, AnySubjectGivenPredicate::from_option(o)),
			None => Self::AnyPredicate(AnySubjectAnyPredicate::from_option(o)),
		}
	}

	pub fn from_pattern<X: PartialEq>(s: X, p: Pattern<T, X>, o: Pattern<T, X>) -> Self {
		match p {
			Pattern::Ground(p) => {
				Self::GivenPredicate(p, AnySubjectGivenPredicate::from_pattern(s, o))
			}
			Pattern::Var(p) => {
				if p == s {
					Self::SameAsSubject(AnySubjectGivenPredicate::from_pattern(s, o))
				} else {
					Self::AnyPredicate(AnySubjectAnyPredicate::from_pattern(s, p, o))
				}
			}
		}
	}

	pub fn with_any_graph(self) -> quad::AnySubject<T> {
		match self {
			Self::AnyPredicate(o) => quad::AnySubject::AnyPredicate(o.with_any_graph()),
			Self::SameAsSubject(o) => quad::AnySubject::SameAsSubject(o.with_any_graph()),
			Self::GivenPredicate(id, o) => quad::AnySubject::GivenPredicate(id, o.with_any_graph()),
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

	pub fn into_predicate_object(self) -> (Option<T>, Option<T>) {
		match self {
			Self::AnyPredicate(o) => (None, o.into_object().into_id()),
			Self::SameAsSubject(o) => (None, o.into_object().into_id()),
			Self::GivenPredicate(p, o) => (Some(p), o.into_object().into_id()),
		}
	}

	pub fn into_parts(self) -> (PatternPredicate<T>, PatternObject<T>) {
		match self {
			Self::AnyPredicate(t) => (PatternPredicate::Any, t.into_object()),
			Self::SameAsSubject(t) => (PatternPredicate::SameAsSubject, t.into_object()),
			Self::GivenPredicate(id, t) => (PatternPredicate::Given(id), t.into_object()),
		}
	}

	pub fn matches(&self, triple: Triple<T>) -> bool
	where
		T: PartialEq,
	{
		match self {
			Self::AnyPredicate(pattern) => pattern.matches(triple),
			Self::SameAsSubject(pattern) => {
				triple.predicate() == triple.subject() && pattern.matches(triple)
			}
			Self::GivenPredicate(p, pattern) => triple.predicate() == p && pattern.matches(triple),
		}
	}

	pub fn map<U>(self, mut f: impl FnMut(T) -> U) -> AnySubject<U> {
		match self {
			Self::AnyPredicate(pq) => AnySubject::AnyPredicate(pq.map(f)),
			Self::SameAsSubject(pq) => AnySubject::SameAsSubject(pq.map(f)),
			Self::GivenPredicate(p, pq) => AnySubject::GivenPredicate(f(p), pq.map(f)),
		}
	}

	pub fn map2<U, V>(self, mut f: impl FnMut(T) -> (U, V)) -> (AnySubject<U>, AnySubject<V>) {
		match self {
			Self::AnyPredicate(pq) => {
				let (pq_u, pq_v) = pq.map2(f);
				(
					AnySubject::AnyPredicate(pq_u),
					AnySubject::AnyPredicate(pq_v),
				)
			}
			Self::SameAsSubject(pq) => {
				let (pq_u, pq_v) = pq.map2(f);
				(
					AnySubject::SameAsSubject(pq_u),
					AnySubject::SameAsSubject(pq_v),
				)
			}
			Self::GivenPredicate(p, pq) => {
				let (u, v) = f(p);
				let (pq_u, pq_v) = pq.map2(f);
				(
					AnySubject::GivenPredicate(u, pq_u),
					AnySubject::GivenPredicate(v, pq_v),
				)
			}
		}
	}

	pub fn set_predicate(&mut self, p: T) -> PatternPredicate<T>
	where
		T: Clone,
	{
		replace_with_or_abort_and_return(self, |this| match this {
			Self::AnyPredicate(AnySubjectAnyPredicate::AnyObject) => (
				PatternPredicate::Any,
				Self::GivenPredicate(p, AnySubjectGivenPredicate::AnyObject),
			),
			Self::AnyPredicate(AnySubjectAnyPredicate::SameAsSubject) => (
				PatternPredicate::Any,
				Self::GivenPredicate(p, AnySubjectGivenPredicate::SameAsSubject),
			),
			Self::AnyPredicate(AnySubjectAnyPredicate::SameAsPredicate) => (
				PatternPredicate::Any,
				Self::GivenPredicate(p.clone(), AnySubjectGivenPredicate::GivenObject(p)),
			),
			Self::AnyPredicate(AnySubjectAnyPredicate::GivenObject(o)) => (
				PatternPredicate::Any,
				Self::GivenPredicate(p, AnySubjectGivenPredicate::GivenObject(o)),
			),
			Self::SameAsSubject(AnySubjectGivenPredicate::AnyObject) => (
				PatternPredicate::SameAsSubject,
				Self::GivenPredicate(p, AnySubjectGivenPredicate::AnyObject),
			),
			Self::SameAsSubject(AnySubjectGivenPredicate::SameAsSubject) => (
				PatternPredicate::SameAsSubject,
				Self::GivenPredicate(p.clone(), AnySubjectGivenPredicate::GivenObject(p)),
			),
			Self::SameAsSubject(AnySubjectGivenPredicate::GivenObject(o)) => (
				PatternPredicate::SameAsSubject,
				Self::GivenPredicate(p, AnySubjectGivenPredicate::GivenObject(o)),
			),
			Self::GivenPredicate(current_p, o) => (
				PatternPredicate::Given(current_p),
				Self::GivenPredicate(p, o),
			),
		})
	}

	pub fn set_object(&mut self, o: T) -> PatternObject<T>
	where
		T: Clone,
	{
		replace_with_or_abort_and_return(self, |this| match this {
			Self::AnyPredicate(AnySubjectAnyPredicate::SameAsPredicate) => (
				PatternObject::Any,
				Self::GivenPredicate(o.clone(), AnySubjectGivenPredicate::GivenObject(o)),
			),
			Self::AnyPredicate(mut current_o) => {
				let old_o = current_o.set_object(o);
				(old_o, Self::AnyPredicate(current_o))
			}
			Self::SameAsSubject(AnySubjectGivenPredicate::SameAsSubject) => (
				PatternObject::SameAsSubject,
				Self::GivenPredicate(o.clone(), AnySubjectGivenPredicate::GivenObject(o)),
			),
			Self::SameAsSubject(mut current_o) => {
				let old_o = current_o.set_object(o);
				(old_o, Self::SameAsSubject(current_o))
			}
			Self::GivenPredicate(p, mut current_o) => {
				let old_o = current_o.set_object(o);
				(old_o, Self::GivenPredicate(p, current_o))
			}
		})
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum AnySubjectAnyPredicate<T> {
	AnyObject,
	SameAsSubject,
	SameAsPredicate,
	GivenObject(T),
}

impl<T> AnySubjectAnyPredicate<T> {
	pub fn as_ref(&self) -> AnySubjectAnyPredicate<&T> {
		match self {
			Self::AnyObject => AnySubjectAnyPredicate::AnyObject,
			Self::SameAsSubject => AnySubjectAnyPredicate::SameAsSubject,
			Self::SameAsPredicate => AnySubjectAnyPredicate::SameAsPredicate,
			Self::GivenObject(t) => AnySubjectAnyPredicate::GivenObject(t),
		}
	}

	pub fn matches(&self, triple: Triple<T>) -> bool
	where
		T: PartialEq,
	{
		match self {
			Self::AnyObject => true,
			Self::SameAsSubject => triple.object() == triple.subject(),
			Self::SameAsPredicate => triple.object() == triple.predicate(),
			Self::GivenObject(o) => triple.object() == o,
		}
	}

	pub fn map<U>(self, mut f: impl FnMut(T) -> U) -> AnySubjectAnyPredicate<U> {
		match self {
			Self::AnyObject => AnySubjectAnyPredicate::AnyObject,
			Self::SameAsSubject => AnySubjectAnyPredicate::SameAsSubject,
			Self::SameAsPredicate => AnySubjectAnyPredicate::SameAsPredicate,
			Self::GivenObject(o) => AnySubjectAnyPredicate::GivenObject(f(o)),
		}
	}

	pub fn map2<U, V>(
		self,
		mut f: impl FnMut(T) -> (U, V),
	) -> (AnySubjectAnyPredicate<U>, AnySubjectAnyPredicate<V>) {
		match self {
			Self::AnyObject => (
				AnySubjectAnyPredicate::AnyObject,
				AnySubjectAnyPredicate::AnyObject,
			),
			Self::SameAsSubject => (
				AnySubjectAnyPredicate::SameAsSubject,
				AnySubjectAnyPredicate::SameAsSubject,
			),
			Self::SameAsPredicate => (
				AnySubjectAnyPredicate::SameAsPredicate,
				AnySubjectAnyPredicate::SameAsPredicate,
			),
			Self::GivenObject(o) => {
				let (u, v) = f(o);
				(
					AnySubjectAnyPredicate::GivenObject(u),
					AnySubjectAnyPredicate::GivenObject(v),
				)
			}
		}
	}
}

impl<T: std::ops::Deref> AnySubjectAnyPredicate<T> {
	pub fn as_deref(&self) -> AnySubjectAnyPredicate<&T::Target> {
		match self {
			Self::AnyObject => AnySubjectAnyPredicate::AnyObject,
			Self::SameAsSubject => AnySubjectAnyPredicate::SameAsSubject,
			Self::SameAsPredicate => AnySubjectAnyPredicate::SameAsPredicate,
			Self::GivenObject(t) => AnySubjectAnyPredicate::GivenObject(&**t),
		}
	}
}

impl<T> AnySubjectAnyPredicate<T> {
	pub fn from_option(o: Option<T>) -> Self {
		match o {
			Some(o) => Self::GivenObject(o),
			None => Self::AnyObject,
		}
	}

	pub fn from_pattern<X: PartialEq>(s: X, p: X, o: Pattern<T, X>) -> Self {
		match o {
			Pattern::Ground(o) => Self::GivenObject(o),
			Pattern::Var(o) => {
				if o == s {
					Self::SameAsSubject
				} else if o == p {
					Self::SameAsPredicate
				} else {
					Self::AnyObject
				}
			}
		}
	}

	pub fn with_any_graph(self) -> quad::AnySubjectAnyPredicate<T> {
		match self {
			Self::AnyObject => quad::AnySubjectAnyPredicate::AnyObject(
				quad::AnySubjectAnyPredicateAnyObject::AnyGraph,
			),
			Self::SameAsSubject => quad::AnySubjectAnyPredicate::SameAsSubject(
				quad::AnySubjectAnyPredicateGivenObject::AnyGraph,
			),
			Self::SameAsPredicate => quad::AnySubjectAnyPredicate::SameAsPredicate(
				quad::AnySubjectAnyPredicateGivenObject::AnyGraph,
			),
			Self::GivenObject(id) => quad::AnySubjectAnyPredicate::GivenObject(
				id,
				quad::AnySubjectAnyPredicateGivenObject::AnyGraph,
			),
		}
	}

	pub fn object(&self) -> PatternObject<&T> {
		match self {
			Self::AnyObject => PatternObject::Any,
			Self::SameAsSubject => PatternObject::SameAsSubject,
			Self::SameAsPredicate => PatternObject::SameAsPredicate,
			Self::GivenObject(id) => PatternObject::Given(id),
		}
	}

	pub fn into_object(self) -> PatternObject<T> {
		match self {
			Self::AnyObject => PatternObject::Any,
			Self::SameAsSubject => PatternObject::SameAsSubject,
			Self::SameAsPredicate => PatternObject::SameAsPredicate,
			Self::GivenObject(id) => PatternObject::Given(id),
		}
	}

	pub fn set_object(&mut self, t: T) -> PatternObject<T> {
		std::mem::replace(self, Self::GivenObject(t)).into_object()
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum AnySubjectGivenPredicate<T> {
	AnyObject,
	SameAsSubject,
	GivenObject(T),
}

impl<T> AnySubjectGivenPredicate<T> {
	pub fn as_ref(&self) -> AnySubjectGivenPredicate<&T> {
		match self {
			Self::AnyObject => AnySubjectGivenPredicate::AnyObject,
			Self::SameAsSubject => AnySubjectGivenPredicate::SameAsSubject,
			Self::GivenObject(t) => AnySubjectGivenPredicate::GivenObject(t),
		}
	}

	pub fn matches(&self, triple: Triple<T>) -> bool
	where
		T: PartialEq,
	{
		match self {
			Self::AnyObject => true,
			Self::SameAsSubject => triple.object() == triple.subject(),
			Self::GivenObject(o) => triple.object() == o,
		}
	}

	pub fn map<U>(self, mut f: impl FnMut(T) -> U) -> AnySubjectGivenPredicate<U> {
		match self {
			Self::AnyObject => AnySubjectGivenPredicate::AnyObject,
			Self::SameAsSubject => AnySubjectGivenPredicate::SameAsSubject,
			Self::GivenObject(o) => AnySubjectGivenPredicate::GivenObject(f(o)),
		}
	}

	pub fn map2<U, V>(
		self,
		mut f: impl FnMut(T) -> (U, V),
	) -> (AnySubjectGivenPredicate<U>, AnySubjectGivenPredicate<V>) {
		match self {
			Self::AnyObject => (
				AnySubjectGivenPredicate::AnyObject,
				AnySubjectGivenPredicate::AnyObject,
			),
			Self::SameAsSubject => (
				AnySubjectGivenPredicate::SameAsSubject,
				AnySubjectGivenPredicate::SameAsSubject,
			),
			Self::GivenObject(o) => {
				let (u, v) = f(o);
				(
					AnySubjectGivenPredicate::GivenObject(u),
					AnySubjectGivenPredicate::GivenObject(v),
				)
			}
		}
	}
}

impl<T: std::ops::Deref> AnySubjectGivenPredicate<T> {
	pub fn as_deref(&self) -> AnySubjectGivenPredicate<&T::Target> {
		match self {
			Self::AnyObject => AnySubjectGivenPredicate::AnyObject,
			Self::SameAsSubject => AnySubjectGivenPredicate::SameAsSubject,
			Self::GivenObject(t) => AnySubjectGivenPredicate::GivenObject(&**t),
		}
	}
}

impl<T> AnySubjectGivenPredicate<T> {
	pub fn from_option(o: Option<T>) -> Self {
		match o {
			Some(o) => Self::GivenObject(o),
			None => Self::AnyObject,
		}
	}

	pub fn from_pattern<X: PartialEq>(s: X, o: Pattern<T, X>) -> Self {
		match o {
			Pattern::Ground(o) => Self::GivenObject(o),
			Pattern::Var(o) => {
				if o == s {
					Self::SameAsSubject
				} else {
					Self::AnyObject
				}
			}
		}
	}

	pub fn with_any_graph(self) -> quad::AnySubjectGivenPredicate<T> {
		match self {
			Self::AnyObject => quad::AnySubjectGivenPredicate::AnyObject(
				quad::AnySubjectGivenPredicateAnyObject::AnyGraph,
			),
			Self::SameAsSubject => quad::AnySubjectGivenPredicate::SameAsSubject(
				quad::AnySubjectGivenPredicateGivenObject::AnyGraph,
			),
			Self::GivenObject(id) => quad::AnySubjectGivenPredicate::GivenObject(
				id,
				quad::AnySubjectGivenPredicateGivenObject::AnyGraph,
			),
		}
	}

	pub fn object(&self) -> PatternObject<&T> {
		match self {
			Self::AnyObject => PatternObject::Any,
			Self::SameAsSubject => PatternObject::SameAsSubject,
			Self::GivenObject(id) => PatternObject::Given(id),
		}
	}

	pub fn into_object(self) -> PatternObject<T> {
		match self {
			Self::AnyObject => PatternObject::Any,
			Self::SameAsSubject => PatternObject::SameAsSubject,
			Self::GivenObject(id) => PatternObject::Given(id),
		}
	}

	pub fn set_object(&mut self, t: T) -> PatternObject<T> {
		std::mem::replace(self, Self::GivenObject(t)).into_object()
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum GivenSubject<T> {
	AnyPredicate(GivenSubjectAnyPredicate<T>),
	GivenPredicate(T, GivenSubjectGivenPredicate<T>),
}

impl<T> GivenSubject<T> {
	pub fn as_ref(&self) -> GivenSubject<&T> {
		match self {
			Self::AnyPredicate(p) => GivenSubject::AnyPredicate(p.as_ref()),
			Self::GivenPredicate(t, p) => GivenSubject::GivenPredicate(t, p.as_ref()),
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

impl<T> GivenSubject<T> {
	pub fn from_option_triple(p: Option<T>, o: Option<T>) -> Self {
		match p {
			Some(p) => Self::GivenPredicate(p, GivenSubjectGivenPredicate::from_option(o)),
			None => Self::AnyPredicate(GivenSubjectAnyPredicate::from_option(o)),
		}
	}

	pub fn from_pattern<X: PartialEq>(p: Pattern<T, X>, o: Pattern<T, X>) -> Self {
		match p {
			Pattern::Ground(p) => {
				Self::GivenPredicate(p, GivenSubjectGivenPredicate::from_pattern(o))
			}
			Pattern::Var(p) => Self::AnyPredicate(GivenSubjectAnyPredicate::from_pattern(p, o)),
		}
	}

	pub fn with_any_graph(self) -> quad::GivenSubject<T> {
		match self {
			Self::AnyPredicate(o) => quad::GivenSubject::AnyPredicate(o.with_any_graph()),
			Self::GivenPredicate(id, o) => {
				quad::GivenSubject::GivenPredicate(id, o.with_any_graph())
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

	pub fn into_predicate_object(self) -> (Option<T>, Option<T>) {
		match self {
			Self::AnyPredicate(o) => (None, o.into_object().into_id()),
			Self::GivenPredicate(p, o) => (Some(p), o.into_object().into_id()),
		}
	}

	pub fn into_parts(self) -> (PatternPredicate<T>, PatternObject<T>) {
		match self {
			Self::AnyPredicate(t) => (PatternPredicate::Any, t.into_object()),
			Self::GivenPredicate(id, t) => (PatternPredicate::Given(id), t.into_object()),
		}
	}

	pub fn matches(&self, triple: Triple<T>) -> bool
	where
		T: PartialEq,
	{
		match self {
			Self::AnyPredicate(pattern) => pattern.matches(triple),
			Self::GivenPredicate(p, pattern) => triple.predicate() == p && pattern.matches(triple),
		}
	}

	pub fn map<U>(self, mut f: impl FnMut(T) -> U) -> GivenSubject<U> {
		match self {
			Self::AnyPredicate(pq) => GivenSubject::AnyPredicate(pq.map(f)),
			Self::GivenPredicate(p, pq) => GivenSubject::GivenPredicate(f(p), pq.map(f)),
		}
	}

	pub fn map2<U, V>(self, mut f: impl FnMut(T) -> (U, V)) -> (GivenSubject<U>, GivenSubject<V>) {
		match self {
			Self::AnyPredicate(pq) => {
				let (pq_u, pq_v) = pq.map2(f);
				(
					GivenSubject::AnyPredicate(pq_u),
					GivenSubject::AnyPredicate(pq_v),
				)
			}
			Self::GivenPredicate(p, pq) => {
				let (u, v) = f(p);
				let (pq_u, pq_v) = pq.map2(f);
				(
					GivenSubject::GivenPredicate(u, pq_u),
					GivenSubject::GivenPredicate(v, pq_v),
				)
			}
		}
	}

	pub fn set_predicate(&mut self, p: T) -> PatternPredicate<T>
	where
		T: Clone,
	{
		replace_with_or_abort_and_return(self, |this| match this {
			Self::AnyPredicate(GivenSubjectAnyPredicate::SameAsPredicate) => (
				PatternPredicate::Any,
				Self::GivenPredicate(p.clone(), GivenSubjectGivenPredicate::GivenObject(p)),
			),
			Self::AnyPredicate(GivenSubjectAnyPredicate::AnyObject) => (
				PatternPredicate::Any,
				Self::GivenPredicate(p, GivenSubjectGivenPredicate::AnyObject),
			),
			Self::AnyPredicate(GivenSubjectAnyPredicate::GivenObject(o)) => (
				PatternPredicate::Any,
				Self::GivenPredicate(p, GivenSubjectGivenPredicate::GivenObject(o)),
			),
			Self::GivenPredicate(current_p, o) => (
				PatternPredicate::Given(current_p),
				Self::GivenPredicate(p, o),
			),
		})
	}

	pub fn set_object(&mut self, o: T) -> PatternObject<T>
	where
		T: Clone,
	{
		replace_with_or_abort_and_return(self, |this| match this {
			Self::AnyPredicate(GivenSubjectAnyPredicate::SameAsPredicate) => (
				PatternObject::Any,
				Self::GivenPredicate(o.clone(), GivenSubjectGivenPredicate::GivenObject(o)),
			),
			Self::AnyPredicate(mut current_o) => {
				let old_o = current_o.set_object(o);
				(old_o, Self::AnyPredicate(current_o))
			}
			Self::GivenPredicate(p, mut current_o) => {
				let old_o = current_o.set_object(o);
				(old_o, Self::GivenPredicate(p, current_o))
			}
		})
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum GivenSubjectAnyPredicate<T> {
	AnyObject,
	SameAsPredicate,
	GivenObject(T),
}

impl<T> GivenSubjectAnyPredicate<T> {
	pub fn as_ref(&self) -> GivenSubjectAnyPredicate<&T> {
		match self {
			Self::AnyObject => GivenSubjectAnyPredicate::AnyObject,
			Self::SameAsPredicate => GivenSubjectAnyPredicate::SameAsPredicate,
			Self::GivenObject(t) => GivenSubjectAnyPredicate::GivenObject(t),
		}
	}

	pub fn matches(&self, triple: Triple<T>) -> bool
	where
		T: PartialEq,
	{
		match self {
			Self::AnyObject => true,
			Self::SameAsPredicate => triple.object() == triple.predicate(),
			Self::GivenObject(o) => triple.object() == o,
		}
	}

	pub fn map<U>(self, mut f: impl FnMut(T) -> U) -> GivenSubjectAnyPredicate<U> {
		match self {
			Self::AnyObject => GivenSubjectAnyPredicate::AnyObject,
			Self::SameAsPredicate => GivenSubjectAnyPredicate::SameAsPredicate,
			Self::GivenObject(o) => GivenSubjectAnyPredicate::GivenObject(f(o)),
		}
	}

	pub fn map2<U, V>(
		self,
		mut f: impl FnMut(T) -> (U, V),
	) -> (GivenSubjectAnyPredicate<U>, GivenSubjectAnyPredicate<V>) {
		match self {
			Self::AnyObject => (
				GivenSubjectAnyPredicate::AnyObject,
				GivenSubjectAnyPredicate::AnyObject,
			),
			Self::SameAsPredicate => (
				GivenSubjectAnyPredicate::SameAsPredicate,
				GivenSubjectAnyPredicate::SameAsPredicate,
			),
			Self::GivenObject(o) => {
				let (u, v) = f(o);
				(
					GivenSubjectAnyPredicate::GivenObject(u),
					GivenSubjectAnyPredicate::GivenObject(v),
				)
			}
		}
	}
}

impl<T: std::ops::Deref> GivenSubjectAnyPredicate<T> {
	pub fn as_deref(&self) -> GivenSubjectAnyPredicate<&T::Target> {
		match self {
			Self::AnyObject => GivenSubjectAnyPredicate::AnyObject,
			Self::SameAsPredicate => GivenSubjectAnyPredicate::SameAsPredicate,
			Self::GivenObject(t) => GivenSubjectAnyPredicate::GivenObject(&**t),
		}
	}
}

impl<T> GivenSubjectAnyPredicate<T> {
	pub fn from_option(o: Option<T>) -> Self {
		match o {
			Some(o) => Self::GivenObject(o),
			None => Self::AnyObject,
		}
	}

	pub fn from_pattern<X: PartialEq>(p: X, o: Pattern<T, X>) -> Self {
		match o {
			Pattern::Ground(o) => Self::GivenObject(o),
			Pattern::Var(o) => {
				if p == o {
					Self::SameAsPredicate
				} else {
					Self::AnyObject
				}
			}
		}
	}

	pub fn with_any_graph(self) -> quad::GivenSubjectAnyPredicate<T> {
		match self {
			Self::AnyObject => quad::GivenSubjectAnyPredicate::AnyObject(
				quad::GivenSubjectAnyPredicateAnyObject::AnyGraph,
			),
			Self::SameAsPredicate => quad::GivenSubjectAnyPredicate::SameAsPredicate(
				quad::GivenSubjectAnyPredicateGivenObject::AnyGraph,
			),
			Self::GivenObject(id) => quad::GivenSubjectAnyPredicate::GivenObject(
				id,
				quad::GivenSubjectAnyPredicateGivenObject::AnyGraph,
			),
		}
	}

	pub fn as_given(&self) -> Option<&T> {
		match self {
			Self::GivenObject(o) => Some(o),
			_ => None,
		}
	}

	pub fn object(&self) -> PatternObject<&T> {
		match self {
			Self::AnyObject => PatternObject::Any,
			Self::SameAsPredicate => PatternObject::SameAsPredicate,
			Self::GivenObject(id) => PatternObject::Given(id),
		}
	}

	pub fn into_object(self) -> PatternObject<T> {
		match self {
			Self::AnyObject => PatternObject::Any,
			Self::SameAsPredicate => PatternObject::SameAsPredicate,
			Self::GivenObject(id) => PatternObject::Given(id),
		}
	}

	pub fn set_object(&mut self, t: T) -> PatternObject<T> {
		std::mem::replace(self, Self::GivenObject(t)).into_object()
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum GivenSubjectGivenPredicate<T> {
	AnyObject,
	GivenObject(T),
}

impl<T> GivenSubjectGivenPredicate<T> {
	pub fn as_ref(&self) -> GivenSubjectGivenPredicate<&T> {
		match self {
			Self::AnyObject => GivenSubjectGivenPredicate::AnyObject,
			Self::GivenObject(t) => GivenSubjectGivenPredicate::GivenObject(t),
		}
	}

	pub fn matches(&self, triple: Triple<T>) -> bool
	where
		T: PartialEq,
	{
		match self {
			Self::AnyObject => true,
			Self::GivenObject(o) => triple.object() == o,
		}
	}

	pub fn map<U>(self, mut f: impl FnMut(T) -> U) -> GivenSubjectGivenPredicate<U> {
		match self {
			Self::AnyObject => GivenSubjectGivenPredicate::AnyObject,
			Self::GivenObject(o) => GivenSubjectGivenPredicate::GivenObject(f(o)),
		}
	}

	pub fn map2<U, V>(
		self,
		mut f: impl FnMut(T) -> (U, V),
	) -> (GivenSubjectGivenPredicate<U>, GivenSubjectGivenPredicate<V>) {
		match self {
			Self::AnyObject => (
				GivenSubjectGivenPredicate::AnyObject,
				GivenSubjectGivenPredicate::AnyObject,
			),
			Self::GivenObject(o) => {
				let (u, v) = f(o);
				(
					GivenSubjectGivenPredicate::GivenObject(u),
					GivenSubjectGivenPredicate::GivenObject(v),
				)
			}
		}
	}
}

impl<T: std::ops::Deref> GivenSubjectGivenPredicate<T> {
	pub fn as_deref(&self) -> GivenSubjectGivenPredicate<&T::Target> {
		match self {
			Self::AnyObject => GivenSubjectGivenPredicate::AnyObject,
			Self::GivenObject(t) => GivenSubjectGivenPredicate::GivenObject(&**t),
		}
	}
}

impl<T> GivenSubjectGivenPredicate<T> {
	pub fn from_option(o: Option<T>) -> Self {
		match o {
			Some(o) => Self::GivenObject(o),
			None => Self::AnyObject,
		}
	}

	pub fn from_pattern<X>(o: Pattern<T, X>) -> Self {
		match o {
			Pattern::Ground(o) => Self::GivenObject(o),
			Pattern::Var(_) => Self::AnyObject,
		}
	}

	pub fn with_any_graph(self) -> quad::GivenSubjectGivenPredicate<T> {
		match self {
			Self::AnyObject => quad::GivenSubjectGivenPredicate::AnyObject(
				quad::GivenSubjectGivenPredicateAnyObject::AnyGraph,
			),
			Self::GivenObject(id) => quad::GivenSubjectGivenPredicate::GivenObject(
				id,
				quad::GivenSubjectGivenPredicateGivenObject::AnyGraph,
			),
		}
	}

	pub fn as_given(&self) -> Option<&T> {
		match self {
			Self::GivenObject(o) => Some(o),
			_ => None,
		}
	}

	pub fn object(&self) -> PatternObject<&T> {
		match self {
			Self::AnyObject => PatternObject::Any,
			Self::GivenObject(id) => PatternObject::Given(id),
		}
	}

	pub fn into_object(self) -> PatternObject<T> {
		match self {
			Self::AnyObject => PatternObject::Any,
			Self::GivenObject(id) => PatternObject::Given(id),
		}
	}

	pub fn set_object(&mut self, t: T) -> PatternObject<T> {
		std::mem::replace(self, Self::GivenObject(t)).into_object()
	}
}

#[cfg(test)]
mod extra_tests {
	use super::*;
	use std::borrow::Cow;

	fn pattern(
		s: Pattern<i32, u8>,
		p: Pattern<i32, u8>,
		o: Pattern<i32, u8>,
	) -> CanonicalTriplePattern<i32> {
		CanonicalTriplePattern::from_pattern(Triple(s, p, o))
	}

	#[test]
	fn any_constant_matches_everything() {
		let any = CanonicalTriplePattern::<i32>::ANY;
		assert!(any.matches(Triple(1, 2, 3)));
		assert!(any.matches(Triple(1, 1, 1)));
	}

	#[test]
	fn matches_same_as_subject_object() {
		// ?0 ?1 ?0 (object == subject)
		let pat = pattern(Pattern::Var(0), Pattern::Var(1), Pattern::Var(0));
		assert!(pat.matches(Triple(1, 2, 1)));
		assert!(!pat.matches(Triple(1, 2, 3)));
	}

	#[test]
	fn matches_given_subject() {
		let pat = pattern(Pattern::Ground(1), Pattern::Var(1), Pattern::Var(2));
		assert!(pat.matches(Triple(1, 2, 3)));
		assert!(!pat.matches(Triple(9, 2, 3)));
	}

	#[test]
	fn into_parts_round_trip() {
		let pat = CanonicalTriplePattern::from_triple(Triple(1, 2, 3));
		assert_eq!(
			pat.into_parts(),
			(
				PatternSubject::Given(1),
				PatternPredicate::Given(2),
				PatternObject::Given(3),
			)
		);
	}

	#[test]
	fn as_ref_borrows() {
		let pat = CanonicalTriplePattern::from_triple(Triple(1, 2, 3));
		let borrowed = pat.as_ref();
		assert_eq!(
			borrowed.into_parts(),
			(
				PatternSubject::Given(&1),
				PatternPredicate::Given(&2),
				PatternObject::Given(&3),
			)
		);
	}

	#[test]
	fn map_and_map2() {
		let pat = CanonicalTriplePattern::from_triple(Triple(1, 2, 3));
		let mapped = pat.map(|x| x * 10);
		assert_eq!(
			mapped.into_parts(),
			(
				PatternSubject::Given(10),
				PatternPredicate::Given(20),
				PatternObject::Given(30),
			)
		);

		let pat = CanonicalTriplePattern::from_triple(Triple(1, 2, 3));
		let (a, b) = pat.map2(|x| (x, x * 100));
		assert_eq!(
			a.into_parts(),
			(
				PatternSubject::Given(1),
				PatternPredicate::Given(2),
				PatternObject::Given(3),
			)
		);
		assert_eq!(
			b.into_parts(),
			(
				PatternSubject::Given(100),
				PatternPredicate::Given(200),
				PatternObject::Given(300),
			)
		);
	}

	#[test]
	fn from_borrowed_triple_and_pattern() {
		let a = 1;
		let b = 2;
		let c = 3;
		let pat: CanonicalTriplePattern<Cow<i32>> = Triple(&a, &b, &c).into();
		assert_eq!(
			pat.into_parts(),
			(
				PatternSubject::Given(Cow::Borrowed(&1)),
				PatternPredicate::Given(Cow::Borrowed(&2)),
				PatternObject::Given(Cow::Borrowed(&3)),
			)
		);

		let base = CanonicalTriplePattern::from_triple(Triple(&a, &b, &c));
		let owned: CanonicalTriplePattern<Cow<i32>> = base.into();
		assert_eq!(
			owned.into_parts(),
			(
				PatternSubject::Given(Cow::Borrowed(&1)),
				PatternPredicate::Given(Cow::Borrowed(&2)),
				PatternObject::Given(Cow::Borrowed(&3)),
			)
		);
	}
}
