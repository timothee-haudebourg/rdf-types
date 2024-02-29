use crate::Triple;
use educe::Educe;
use std::collections::{HashMap, HashSet};
use std::hash::Hash;

use super::canonical::{
	AnySubject, AnySubjectAnyPredicate, AnySubjectGivenPredicate, CanonicalTriplePattern,
	GivenSubject, GivenSubjectAnyPredicate, GivenSubjectGivenPredicate,
};

/// Data-structure that maps triple patterns to values.
#[derive(Debug, Educe)]
#[educe(Default)]
pub struct TriplePatternMap<V, T> {
	any: AnySubjectMap<V, T>,
	given: HashMap<T, GivenSubjectMap<V, T>>,
}

impl<V: Eq + Hash, T: Eq + Hash> TriplePatternMap<V, T> {
	pub fn insert(&mut self, pattern: CanonicalTriplePattern<T>, value: V) -> bool {
		match pattern {
			CanonicalTriplePattern::AnySubject(rest) => self.any.insert(rest, value),
			CanonicalTriplePattern::GivenSubject(id, rest) => {
				self.given.entry(id).or_default().insert(rest, value)
			}
		}
	}
}

impl<V, T: Eq + Hash> TriplePatternMap<V, T> {
	pub fn get(&self, triple: Triple<&T>) -> Values<V> {
		Values {
			any: self.any.get(triple),
			given: self.given.get(triple.subject()).map(|s| s.get(triple)),
		}
	}
}

// impl<V: Eq + Hash + ReplaceId> ReplaceId for Map<V> {
// 	fn replace_id(&mut self, a: Id, b: Id) {
// 		self.any.replace_id(a, b);
// 		self.given.replace_id(a, b)
// 	}
// }

pub struct Values<'a, V> {
	any: AnySubjectValues<'a, V>,
	given: Option<GivenSubjectValues<'a, V>>,
}

impl<'a, V> Iterator for Values<'a, V> {
	type Item = &'a V;

	fn next(&mut self) -> Option<Self::Item> {
		self.any
			.next()
			.or_else(|| self.given.as_mut().and_then(|i| i.next()))
	}
}

#[derive(Debug, Educe)]
#[educe(Default)]
pub struct GivenSubjectMap<V, T> {
	any: GivenSubjectAnyPredicateMap<V, T>,
	given: HashMap<T, GivenSubjectGivenPredicateMap<V, T>>,
}

impl<V: Eq + Hash, T: Eq + Hash> GivenSubjectMap<V, T> {
	pub fn insert(&mut self, pattern: GivenSubject<T>, value: V) -> bool {
		match pattern {
			GivenSubject::AnyPredicate(rest) => self.any.insert(rest, value),
			GivenSubject::GivenPredicate(id, rest) => {
				self.given.entry(id).or_default().insert(rest, value)
			}
		}
	}
}

impl<V, T: Eq + Hash> GivenSubjectMap<V, T> {
	pub fn get(&self, triple: Triple<&T>) -> GivenSubjectValues<V> {
		GivenSubjectValues {
			any: self.any.get(triple),
			given: self.given.get(triple.predicate()).map(|p| p.get(triple)),
		}
	}
}

impl<V: Eq + Hash, T: Eq + Hash> GivenSubjectMap<V, T> {
	pub fn union_with(&mut self, other: Self) {
		self.any.union_with(other.any);
		self.given.extend(other.given);
	}
}

// impl<V: Eq + Hash + ReplaceId> ReplaceId for GivenSubjectMap<V, Id> {
// 	fn replace_id(&mut self, a: Id, b: Id) {
// 		self.any.replace_id(a, b);
// 		self.given.replace_id(a, b)
// 	}
// }

pub struct GivenSubjectValues<'a, V> {
	any: GivenSubjectAnyPredicateValues<'a, V>,
	given: Option<GivenSubjectGivenPredicateValues<'a, V>>,
}

impl<'a, V> Iterator for GivenSubjectValues<'a, V> {
	type Item = &'a V;

	fn next(&mut self) -> Option<Self::Item> {
		self.any
			.next()
			.or_else(|| self.given.as_mut().and_then(|i| i.next()))
	}
}

#[derive(Debug, Educe)]
#[educe(Default)]
pub struct GivenSubjectAnyPredicateMap<V, T> {
	any: HashSet<V>,
	same_as_predicate: HashSet<V>,
	given: HashMap<T, HashSet<V>>,
}

impl<V: Eq + Hash, T: Eq + Hash> GivenSubjectAnyPredicateMap<V, T> {
	pub fn insert(&mut self, pattern: GivenSubjectAnyPredicate<T>, value: V) -> bool {
		match pattern {
			GivenSubjectAnyPredicate::AnyObject => self.any.insert(value),
			GivenSubjectAnyPredicate::SameAsPredicate => self.same_as_predicate.insert(value),
			GivenSubjectAnyPredicate::GivenObject(id) => {
				self.given.entry(id).or_default().insert(value)
			}
		}
	}
}

impl<V, T: Eq + Hash> GivenSubjectAnyPredicateMap<V, T> {
	pub fn get(&self, triple: Triple<&T>) -> GivenSubjectAnyPredicateValues<V> {
		GivenSubjectAnyPredicateValues {
			any: self.any.iter(),
			same_as_predicate: if triple.predicate() == triple.object() {
				Some(self.same_as_predicate.iter())
			} else {
				None
			},
			given: self.given.get(triple.object()).map(|o| o.iter()),
		}
	}
}

// impl<V: Eq + Hash + ReplaceId> ReplaceId for GivenSubjectAnyPredicateMap<V, Id> {
// 	fn replace_id(&mut self, a: Id, b: Id) {
// 		self.any.replace_id(a, b);
// 		self.same_as_predicate.replace_id(a, b);
// 		self.given.replace_id(a, b)
// 	}
// }

impl<V: Eq + Hash, T: Eq + Hash> GivenSubjectAnyPredicateMap<V, T> {
	pub fn union_with(&mut self, other: Self) {
		self.any.extend(other.any);
		self.same_as_predicate.extend(other.same_as_predicate);
		self.given.extend(other.given)
	}
}

pub struct GivenSubjectAnyPredicateValues<'a, V> {
	any: std::collections::hash_set::Iter<'a, V>,
	same_as_predicate: Option<std::collections::hash_set::Iter<'a, V>>,
	given: Option<std::collections::hash_set::Iter<'a, V>>,
}

impl<'a, V> Iterator for GivenSubjectAnyPredicateValues<'a, V> {
	type Item = &'a V;

	fn next(&mut self) -> Option<Self::Item> {
		self.any
			.next()
			.or_else(|| self.same_as_predicate.as_mut().and_then(|i| i.next()))
			.or_else(|| self.given.as_mut().and_then(|i| i.next()))
	}
}

#[derive(Debug, Educe)]
#[educe(Default)]
pub struct GivenSubjectGivenPredicateMap<V, T> {
	any: HashSet<V>,
	given: HashMap<T, HashSet<V>>,
}

impl<V: Eq + Hash, T: Eq + Hash> GivenSubjectGivenPredicateMap<V, T> {
	pub fn insert(&mut self, pattern: GivenSubjectGivenPredicate<T>, value: V) -> bool {
		match pattern {
			GivenSubjectGivenPredicate::AnyObject => self.any.insert(value),
			GivenSubjectGivenPredicate::GivenObject(id) => {
				self.given.entry(id).or_default().insert(value)
			}
		}
	}
}

impl<V, T: Eq + Hash> GivenSubjectGivenPredicateMap<V, T> {
	pub fn get(&self, triple: Triple<&T>) -> GivenSubjectGivenPredicateValues<V> {
		GivenSubjectGivenPredicateValues {
			any: self.any.iter(),
			given: self.given.get(triple.object()).map(|o| o.iter()),
		}
	}
}

// impl<V: Eq + Hash + ReplaceId> ReplaceId for GivenSubjectGivenPredicateMap<V, Id> {
// 	fn replace_id(&mut self, a: Id, b: Id) {
// 		self.any.replace_id(a, b);
// 		self.given.replace_id(a, b)
// 	}
// }

impl<V: Eq + Hash, T: Eq + Hash> GivenSubjectGivenPredicateMap<V, T> {
	pub fn union_with(&mut self, other: Self) {
		self.any.extend(other.any);
		self.given.extend(other.given)
	}
}

pub struct GivenSubjectGivenPredicateValues<'a, V> {
	any: std::collections::hash_set::Iter<'a, V>,
	given: Option<std::collections::hash_set::Iter<'a, V>>,
}

impl<'a, V> Iterator for GivenSubjectGivenPredicateValues<'a, V> {
	type Item = &'a V;

	fn next(&mut self) -> Option<Self::Item> {
		self.any
			.next()
			.or_else(|| self.given.as_mut().and_then(|i| i.next()))
	}
}

#[derive(Debug, Educe)]
#[educe(Default)]
pub struct AnySubjectMap<V, T> {
	any: AnySubjectAnyPredicateMap<V, T>,
	same_as_subject: AnySubjectGivenPredicateMap<V, T>,
	given: HashMap<T, AnySubjectGivenPredicateMap<V, T>>,
}

impl<V: Eq + Hash, T: Eq + Hash> AnySubjectMap<V, T> {
	pub fn insert(&mut self, pattern: AnySubject<T>, value: V) -> bool {
		match pattern {
			AnySubject::AnyPredicate(rest) => self.any.insert(rest, value),
			AnySubject::SameAsSubject(rest) => self.same_as_subject.insert(rest, value),
			AnySubject::GivenPredicate(id, rest) => {
				self.given.entry(id).or_default().insert(rest, value)
			}
		}
	}
}

impl<V, T: Eq + Hash> AnySubjectMap<V, T> {
	pub fn get(&self, triple: Triple<&T>) -> AnySubjectValues<V> {
		AnySubjectValues {
			any: self.any.get(triple),
			same_as_subject: if triple.subject() == triple.predicate() {
				Some(self.same_as_subject.get(triple))
			} else {
				None
			},
			given: self.given.get(triple.predicate()).map(|p| p.get(triple)),
		}
	}
}

// impl<V: Eq + Hash + ReplaceId> ReplaceId for AnySubjectMap<V, Id> {
// 	fn replace_id(&mut self, a: Id, b: Id) {
// 		self.any.replace_id(a, b);
// 		self.same_as_subject.replace_id(a, b);
// 		self.given.replace_id(a, b)
// 	}
// }

impl<V: Eq + Hash, T: Eq + Hash> AnySubjectMap<V, T> {
	pub fn union_with(&mut self, other: Self) {
		self.any.union_with(other.any);
		self.same_as_subject.union_with(other.same_as_subject);
		self.given.extend(other.given)
	}
}

pub struct AnySubjectValues<'a, V> {
	any: AnySubjectAnyPredicateValues<'a, V>,
	same_as_subject: Option<AnySubjectGivenPredicateValues<'a, V>>,
	given: Option<AnySubjectGivenPredicateValues<'a, V>>,
}

impl<'a, V> Iterator for AnySubjectValues<'a, V> {
	type Item = &'a V;

	fn next(&mut self) -> Option<Self::Item> {
		self.any
			.next()
			.or_else(|| self.same_as_subject.as_mut().and_then(|i| i.next()))
			.or_else(|| self.given.as_mut().and_then(|i| i.next()))
	}
}

#[derive(Debug, Educe)]
#[educe(Default)]
pub struct AnySubjectAnyPredicateMap<V, T> {
	any: HashSet<V>,
	same_as_subject: HashSet<V>,
	same_as_predicate: HashSet<V>,
	given: HashMap<T, HashSet<V>>,
}

impl<V: Eq + Hash, T: Eq + Hash> AnySubjectAnyPredicateMap<V, T> {
	pub fn insert(&mut self, pattern: AnySubjectAnyPredicate<T>, value: V) -> bool {
		match pattern {
			AnySubjectAnyPredicate::AnyObject => self.any.insert(value),
			AnySubjectAnyPredicate::SameAsSubject => self.same_as_subject.insert(value),
			AnySubjectAnyPredicate::SameAsPredicate => self.same_as_predicate.insert(value),
			AnySubjectAnyPredicate::GivenObject(id) => {
				self.given.entry(id).or_default().insert(value)
			}
		}
	}
}

impl<V, T: Eq + Hash> AnySubjectAnyPredicateMap<V, T> {
	pub fn get(&self, triple: Triple<&T>) -> AnySubjectAnyPredicateValues<V> {
		AnySubjectAnyPredicateValues {
			any: self.any.iter(),
			same_as_subject: if triple.subject() == triple.object() {
				Some(self.same_as_subject.iter())
			} else {
				None
			},
			same_as_predicate: if triple.predicate() == triple.object() {
				Some(self.same_as_predicate.iter())
			} else {
				None
			},
			given: self.given.get(triple.object()).map(|o| o.iter()),
		}
	}
}

// impl<V: Eq + Hash + ReplaceId> ReplaceId for AnySubjectAnyPredicateMap<V, Id> {
// 	fn replace_id(&mut self, a: Id, b: Id) {
// 		self.any.replace_id(a, b);
// 		self.same_as_subject.replace_id(a, b);
// 		self.same_as_predicate.replace_id(a, b);
// 		self.given.replace_id(a, b)
// 	}
// }

impl<V: Eq + Hash, T: Eq + Hash> AnySubjectAnyPredicateMap<V, T> {
	pub fn union_with(&mut self, other: Self) {
		self.any.extend(other.any);
		self.same_as_subject.extend(other.same_as_subject);
		self.same_as_predicate.extend(other.same_as_predicate);
		self.given.extend(other.given)
	}
}

pub struct AnySubjectAnyPredicateValues<'a, V> {
	any: std::collections::hash_set::Iter<'a, V>,
	same_as_subject: Option<std::collections::hash_set::Iter<'a, V>>,
	same_as_predicate: Option<std::collections::hash_set::Iter<'a, V>>,
	given: Option<std::collections::hash_set::Iter<'a, V>>,
}

impl<'a, V> Iterator for AnySubjectAnyPredicateValues<'a, V> {
	type Item = &'a V;

	fn next(&mut self) -> Option<Self::Item> {
		self.any
			.next()
			.or_else(|| self.same_as_subject.as_mut().and_then(|i| i.next()))
			.or_else(|| self.same_as_predicate.as_mut().and_then(|i| i.next()))
			.or_else(|| self.given.as_mut().and_then(|i| i.next()))
	}
}

#[derive(Debug, Educe)]
#[educe(Default)]
pub struct AnySubjectGivenPredicateMap<V, T> {
	any: HashSet<V>,
	same_as_subject: HashSet<V>,
	given: HashMap<T, HashSet<V>>,
}

impl<V: Eq + Hash, T: Eq + Hash> AnySubjectGivenPredicateMap<V, T> {
	pub fn insert(&mut self, pattern: AnySubjectGivenPredicate<T>, value: V) -> bool {
		match pattern {
			AnySubjectGivenPredicate::AnyObject => self.any.insert(value),
			AnySubjectGivenPredicate::SameAsSubject => self.same_as_subject.insert(value),
			AnySubjectGivenPredicate::GivenObject(id) => {
				self.given.entry(id).or_default().insert(value)
			}
		}
	}
}

impl<V, T: Eq + Hash> AnySubjectGivenPredicateMap<V, T> {
	pub fn get(&self, triple: Triple<&T>) -> AnySubjectGivenPredicateValues<V> {
		AnySubjectGivenPredicateValues {
			any: self.any.iter(),
			same_as_subject: if triple.subject() == triple.object() {
				Some(self.same_as_subject.iter())
			} else {
				None
			},
			given: self.given.get(triple.object()).map(|o| o.iter()),
		}
	}
}

// impl<V: Eq + Hash + ReplaceId> ReplaceId for AnySubjectGivenPredicateMap<V, Id> {
// 	fn replace_id(&mut self, a: Id, b: Id) {
// 		self.any.replace_id(a, b);
// 		self.same_as_subject.replace_id(a, b);
// 		self.given.replace_id(a, b)
// 	}
// }

impl<V: Eq + Hash, T: Eq + Hash> AnySubjectGivenPredicateMap<V, T> {
	pub fn union_with(&mut self, other: Self) {
		self.any.extend(other.any);
		self.same_as_subject.extend(other.same_as_subject);
		self.given.extend(other.given)
	}
}

pub struct AnySubjectGivenPredicateValues<'a, V> {
	any: std::collections::hash_set::Iter<'a, V>,
	same_as_subject: Option<std::collections::hash_set::Iter<'a, V>>,
	given: Option<std::collections::hash_set::Iter<'a, V>>,
}

impl<'a, V> Iterator for AnySubjectGivenPredicateValues<'a, V> {
	type Item = &'a V;

	fn next(&mut self) -> Option<Self::Item> {
		self.any
			.next()
			.or_else(|| self.same_as_subject.as_mut().and_then(|i| i.next()))
			.or_else(|| self.given.as_mut().and_then(|i| i.next()))
	}
}
