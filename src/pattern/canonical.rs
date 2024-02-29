use super::{ResourceOrVar, TriplePattern};
use crate::Triple;

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

// impl<T: Clone, X> From<CanonicalTriplePattern<T>> for TriplePattern<T, X> {
// 	fn from(value: CanonicalTriplePattern<T>) -> Self {
// 		let s = match value.subject().cloned() {
// 			PatternSubject::Any => ResourceOrVar::Var(0),
// 			PatternSubject::Given(id) => ResourceOrVar::Resource(id),
// 		};

// 		let p = match value.predicate().cloned() {
// 			PatternPredicate::Any => ResourceOrVar::Var(1),
// 			PatternPredicate::SameAsSubject => ResourceOrVar::Var(0),
// 			PatternPredicate::Given(id) => ResourceOrVar::Resource(id),
// 		};

// 		let o = match value.object().cloned() {
// 			PatternObject::Any => ResourceOrVar::Var(2),
// 			PatternObject::SameAsSubject => ResourceOrVar::Var(0),
// 			PatternObject::SameAsPredicate => ResourceOrVar::Var(1),
// 			PatternObject::Given(id) => ResourceOrVar::Resource(id),
// 		};

// 		Triple(s, p, o)
// 	}
// }

impl<T> CanonicalTriplePattern<T> {
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
			ResourceOrVar::Resource(s) => {
				Self::GivenSubject(s, GivenSubject::from_pattern(pattern.1, pattern.2))
			}
			ResourceOrVar::Var(s) => {
				Self::AnySubject(AnySubject::from_pattern(s, pattern.1, pattern.2))
			}
		}
	}

	pub fn subject(&self) -> PatternSubject<&T> {
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

	pub fn object(&self) -> PatternObject<&T> {
		match self {
			Self::AnySubject(t) => t.object(),
			Self::GivenSubject(_, t) => t.object(),
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
	pub fn from_option_triple(p: Option<T>, o: Option<T>) -> Self {
		match p {
			Some(p) => Self::GivenPredicate(p, AnySubjectGivenPredicate::from_option(o)),
			None => Self::AnyPredicate(AnySubjectAnyPredicate::from_option(o)),
		}
	}

	pub fn from_pattern<X: PartialEq>(
		s: X,
		p: ResourceOrVar<T, X>,
		o: ResourceOrVar<T, X>,
	) -> Self {
		match p {
			ResourceOrVar::Resource(p) => {
				Self::GivenPredicate(p, AnySubjectGivenPredicate::from_pattern(s, o))
			}
			ResourceOrVar::Var(p) => {
				if p == s {
					Self::SameAsSubject(AnySubjectGivenPredicate::from_pattern(s, o))
				} else {
					Self::AnyPredicate(AnySubjectAnyPredicate::from_pattern(s, p, o))
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

	pub fn object(&self) -> PatternObject<&T> {
		match self {
			Self::AnyPredicate(t) => t.object(),
			Self::SameAsSubject(t) => t.object(),
			Self::GivenPredicate(_, t) => t.object(),
		}
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
	pub fn from_option(o: Option<T>) -> Self {
		match o {
			Some(o) => Self::GivenObject(o),
			None => Self::AnyObject,
		}
	}

	pub fn from_pattern<X: PartialEq>(s: X, p: X, o: ResourceOrVar<T, X>) -> Self {
		match o {
			ResourceOrVar::Resource(o) => Self::GivenObject(o),
			ResourceOrVar::Var(o) => {
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

	pub fn object(&self) -> PatternObject<&T> {
		match self {
			Self::AnyObject => PatternObject::Any,
			Self::SameAsSubject => PatternObject::SameAsSubject,
			Self::SameAsPredicate => PatternObject::SameAsPredicate,
			Self::GivenObject(id) => PatternObject::Given(id),
		}
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum AnySubjectGivenPredicate<T> {
	AnyObject,
	SameAsSubject,
	GivenObject(T),
}

impl<T> AnySubjectGivenPredicate<T> {
	pub fn from_option(o: Option<T>) -> Self {
		match o {
			Some(o) => Self::GivenObject(o),
			None => Self::AnyObject,
		}
	}

	pub fn from_pattern<X: PartialEq>(s: X, o: ResourceOrVar<T, X>) -> Self {
		match o {
			ResourceOrVar::Resource(o) => Self::GivenObject(o),
			ResourceOrVar::Var(o) => {
				if o == s {
					Self::SameAsSubject
				} else {
					Self::AnyObject
				}
			}
		}
	}

	pub fn object(&self) -> PatternObject<&T> {
		match self {
			Self::AnyObject => PatternObject::Any,
			Self::SameAsSubject => PatternObject::SameAsSubject,
			Self::GivenObject(id) => PatternObject::Given(id),
		}
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum GivenSubject<T> {
	AnyPredicate(GivenSubjectAnyPredicate<T>),
	GivenPredicate(T, GivenSubjectGivenPredicate<T>),
}

impl<T> GivenSubject<T> {
	pub fn from_option_triple(p: Option<T>, o: Option<T>) -> Self {
		match p {
			Some(p) => Self::GivenPredicate(p, GivenSubjectGivenPredicate::from_option(o)),
			None => Self::AnyPredicate(GivenSubjectAnyPredicate::from_option(o)),
		}
	}

	pub fn from_pattern<X: PartialEq>(p: ResourceOrVar<T, X>, o: ResourceOrVar<T, X>) -> Self {
		match p {
			ResourceOrVar::Resource(p) => {
				Self::GivenPredicate(p, GivenSubjectGivenPredicate::from_pattern(o))
			}
			ResourceOrVar::Var(p) => {
				Self::AnyPredicate(GivenSubjectAnyPredicate::from_pattern(p, o))
			}
		}
	}

	pub fn predicate(&self) -> PatternPredicate<&T> {
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
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum GivenSubjectAnyPredicate<T> {
	AnyObject,
	SameAsPredicate,
	GivenObject(T),
}

impl<T> GivenSubjectAnyPredicate<T> {
	pub fn from_option(o: Option<T>) -> Self {
		match o {
			Some(o) => Self::GivenObject(o),
			None => Self::AnyObject,
		}
	}

	pub fn from_pattern<X: PartialEq>(p: X, o: ResourceOrVar<T, X>) -> Self {
		match o {
			ResourceOrVar::Resource(o) => Self::GivenObject(o),
			ResourceOrVar::Var(o) => {
				if p == o {
					Self::SameAsPredicate
				} else {
					Self::AnyObject
				}
			}
		}
	}

	pub fn object(&self) -> PatternObject<&T> {
		match self {
			Self::AnyObject => PatternObject::Any,
			Self::SameAsPredicate => PatternObject::SameAsPredicate,
			Self::GivenObject(id) => PatternObject::Given(id),
		}
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum GivenSubjectGivenPredicate<T> {
	AnyObject,
	GivenObject(T),
}

impl<T> GivenSubjectGivenPredicate<T> {
	pub fn from_option(o: Option<T>) -> Self {
		match o {
			Some(o) => Self::GivenObject(o),
			None => Self::AnyObject,
		}
	}

	pub fn from_pattern<X>(o: ResourceOrVar<T, X>) -> Self {
		match o {
			ResourceOrVar::Resource(o) => Self::GivenObject(o),
			ResourceOrVar::Var(_) => Self::AnyObject,
		}
	}

	pub fn object(&self) -> PatternObject<&T> {
		match self {
			Self::AnyObject => PatternObject::Any,
			Self::GivenObject(id) => PatternObject::Given(id),
		}
	}
}
