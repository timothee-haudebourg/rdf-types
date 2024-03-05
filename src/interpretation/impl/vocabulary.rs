use educe::Educe;
use std::collections::{HashMap, HashSet};
use std::fmt::Debug;
use std::hash::Hash;
use std::iter::once;

use crate::interpretation::{ReverseBlankIdInterpretation, ReverseBlankIdInterpretationMut};
use crate::{
	interpretation::{
		ReverseIriInterpretation, ReverseIriInterpretationMut, ReverseLiteralInterpretation,
		ReverseLiteralInterpretationMut,
	},
	vocabulary::{BlankIdVocabulary, IriVocabulary, LiteralVocabulary},
	Id, Interpretation, InterpretationMut, Term, Vocabulary,
};

type VocabTerm<V> = Term<
	Id<<V as IriVocabulary>::Iri, <V as BlankIdVocabulary>::BlankId>,
	<V as LiteralVocabulary>::Literal,
>;

pub struct VocabularyInterpretationSubstitution<V: Vocabulary>(Vec<VocabTerm<V>>);

/// Mutable vocabulary interpretation.
///
/// This type is a special sort of interpretation where every term is
/// interpreted as itself, while allowing anonymous resources to be added to
/// the interpretation. In particular, it implements the `InterpretationMut`
/// trait, which is not implemented by the unit `()` interpretation.
///
/// It is possible to remove anonymous nodes by building
/// [`VocabularyInterpretationSubstitution`] with [`Self::into_substitution`] or
/// [`Self::as_substitution`], and using it to substitute resources back into
/// terms.
pub struct VocabularyInterpretation<V: Vocabulary> {
	map: HashMap<Resource<V>, HashSet<VocabTerm<V>>>,
	anonymous_count: usize,
}

impl<V: Vocabulary> Default for VocabularyInterpretation<V> {
	fn default() -> Self {
		Self::new()
	}
}

impl<V: Vocabulary> VocabularyInterpretation<V> {
	pub fn new() -> Self {
		Self {
			map: HashMap::new(),
			anonymous_count: 0,
		}
	}
}

impl<V: Vocabulary> VocabularyInterpretation<V>
where
	V::Iri: PartialEq,
	V::BlankId: PartialEq,
	V::Literal: PartialEq,
{
	/// Try to build a substitution from resources to terms.
	///
	/// In a `VocabularyInterpretation` most-resources are already interpreted
	/// as terms. This function will return an error if not **all** resources
	/// are associated to a term, or if there is a term ambiguity.
	pub fn into_substitution(
		self,
	) -> Result<VocabularyInterpretationSubstitution<V>, VocabularyInterpretationError<V>> {
		let mut opt_list = Vec::new();
		opt_list.resize_with(self.anonymous_count, || None);

		for (r, terms) in self.map {
			match r {
				Resource::Anonymous(i) => {
					let mut terms = terms.into_iter();
					match terms.next() {
						Some(term) => {
							for u in terms {
								if term != u {
									return Err(VocabularyInterpretationError::Ambiguity(term, u));
								}
							}

							opt_list[i] = Some(term)
						}
						None => return Err(VocabularyInterpretationError::MissingTerm(i)),
					}
				}
				Resource::Term(t) => {
					for u in terms {
						if t != u {
							return Err(VocabularyInterpretationError::Ambiguity(t, u));
						}
					}
				}
			}
		}

		let mut list = Vec::with_capacity(opt_list.len());
		for opt in opt_list {
			match opt {
				Some(term) => list.push(term),
				None => return Err(VocabularyInterpretationError::MissingTerm(list.len())),
			}
		}

		Ok(VocabularyInterpretationSubstitution(list))
	}

	/// Try to build a substitution from resources to terms.
	///
	/// In a `VocabularyInterpretation` most-resources are already interpreted
	/// as terms. This function will return an error if not **all** resources
	/// are associated to a term, or if there is a term ambiguity.
	pub fn as_substitution(
		&self,
	) -> Result<VocabularyInterpretationSubstitution<V>, VocabularyInterpretationError<V>>
	where
		V::Iri: Clone,
		V::BlankId: Clone,
		V::Literal: Clone,
	{
		let mut opt_list = Vec::new();
		opt_list.resize_with(self.anonymous_count, || None);

		for (r, terms) in &self.map {
			match r {
				Resource::Anonymous(i) => {
					let mut terms = terms.iter();
					match terms.next().cloned() {
						Some(term) => {
							for u in terms {
								if term != *u {
									return Err(VocabularyInterpretationError::Ambiguity(
										term,
										u.clone(),
									));
								}
							}

							opt_list[*i] = Some(term)
						}
						None => return Err(VocabularyInterpretationError::MissingTerm(*i)),
					}
				}
				Resource::Term(t) => {
					for u in terms {
						if t != u {
							return Err(VocabularyInterpretationError::Ambiguity(
								t.clone(),
								u.clone(),
							));
						}
					}
				}
			}
		}

		let mut list = Vec::with_capacity(opt_list.len());
		for opt in opt_list {
			match opt {
				Some(term) => list.push(term),
				None => return Err(VocabularyInterpretationError::MissingTerm(list.len())),
			}
		}

		Ok(VocabularyInterpretationSubstitution(list))
	}
}

pub enum VocabularyInterpretationError<V: Vocabulary> {
	MissingTerm(usize),
	Ambiguity(VocabTerm<V>, VocabTerm<V>),
}

impl<V: Vocabulary> Interpretation for VocabularyInterpretation<V> {
	type Resource = Resource<V>;
}

impl<V: Vocabulary> InterpretationMut<V> for VocabularyInterpretation<V> {
	fn new_resource(&mut self, _vocabulary: &mut V) -> Self::Resource {
		let i = self.anonymous_count;
		self.anonymous_count += 1;
		Resource::Anonymous(i)
	}
}

impl<V: Vocabulary> ReverseIriInterpretation for VocabularyInterpretation<V>
where
	V::Iri: Eq + Hash,
	V::BlankId: Eq + Hash,
	V::Literal: Eq + Hash,
{
	type Iri = V::Iri;
	type Iris<'a> = IrisOf<'a, V> where V: 'a, V::Iri: 'a, V::BlankId: 'a, V::Literal: 'a;

	fn iris_of<'a>(&'a self, id: &'a Self::Resource) -> Self::Iris<'a> {
		IrisOf {
			term: id.as_term(),
			additional_terms: self.map.get(id).map(|t| t.iter()),
		}
	}
}

impl<V: Vocabulary> ReverseIriInterpretationMut for VocabularyInterpretation<V>
where
	V::Iri: Clone + Eq + Hash,
	V::BlankId: Clone + Eq + Hash,
	V::Literal: Clone + Eq + Hash,
{
	fn assign_iri(&mut self, id: &Self::Resource, iri: Self::Iri) -> bool {
		match self.map.get_mut(id) {
			Some(l) => l.insert(Term::iri(iri)),
			None => {
				self.map.insert(id.clone(), once(Term::iri(iri)).collect());
				true
			}
		}
	}
}

impl<V: Vocabulary> ReverseBlankIdInterpretation for VocabularyInterpretation<V>
where
	V::Iri: Eq + Hash,
	V::BlankId: Eq + Hash,
	V::Literal: Eq + Hash,
{
	type BlankId = V::BlankId;
	type BlankIds<'a> = BlankIdsOf<'a, V> where V: 'a, V::Iri: 'a, V::BlankId: 'a, V::Literal: 'a;

	fn blank_ids_of<'a>(&'a self, id: &'a Self::Resource) -> Self::BlankIds<'a> {
		BlankIdsOf {
			term: id.as_term(),
			additional_terms: self.map.get(id).map(|t| t.iter()),
		}
	}
}

impl<V: Vocabulary> ReverseBlankIdInterpretationMut for VocabularyInterpretation<V>
where
	V::Iri: Clone + Eq + Hash,
	V::BlankId: Clone + Eq + Hash,
	V::Literal: Clone + Eq + Hash,
{
	fn assign_blank_id(&mut self, id: &Self::Resource, b: Self::BlankId) -> bool {
		match self.map.get_mut(id) {
			Some(l) => l.insert(Term::blank(b)),
			None => {
				self.map.insert(id.clone(), once(Term::blank(b)).collect());
				true
			}
		}
	}
}

impl<V: Vocabulary> ReverseLiteralInterpretation for VocabularyInterpretation<V>
where
	V::Iri: Clone + Eq + Hash,
	V::BlankId: Clone + Eq + Hash,
	V::Literal: Clone + Eq + Hash,
{
	type Literal = V::Literal;
	type Literals<'a> = LiteralsOf<'a, V> where V: 'a, V::Iri: 'a, V::BlankId: 'a, V::Literal: 'a;

	fn literals_of<'a>(&'a self, id: &'a Self::Resource) -> Self::Literals<'a> {
		LiteralsOf {
			term: id.as_term(),
			additional_terms: self.map.get(id).map(|t| t.iter()),
		}
	}
}

impl<V: Vocabulary> ReverseLiteralInterpretationMut for VocabularyInterpretation<V>
where
	V::Iri: Clone + Eq + Hash,
	V::BlankId: Clone + Eq + Hash,
	V::Literal: Clone + Eq + Hash,
{
	fn assign_literal(&mut self, id: &Self::Resource, literal: Self::Literal) -> bool {
		match self.map.get_mut(id) {
			Some(l) => l.insert(Term::Literal(literal)),
			None => {
				self.map
					.insert(id.clone(), once(Term::Literal(literal)).collect());
				true
			}
		}
	}
}

#[derive(Educe)]
#[educe(Debug(bound(V::Iri: Debug, V::BlankId: Debug, V::Literal: Debug)))]
pub enum Resource<V: Vocabulary = ()> {
	Anonymous(usize),
	Term(VocabTerm<V>),
}

impl<V: Vocabulary> Resource<V> {
	pub fn as_term(&self) -> Option<&VocabTerm<V>> {
		match self {
			Self::Term(t) => Some(t),
			_ => None,
		}
	}
}

impl<V: Vocabulary> From<VocabTerm<V>> for Resource<V> {
	fn from(value: VocabTerm<V>) -> Self {
		Self::Term(value)
	}
}

impl<V: Vocabulary> PartialEq for Resource<V>
where
	V::Iri: PartialEq,
	V::BlankId: PartialEq,
	V::Literal: PartialEq,
{
	fn eq(&self, other: &Self) -> bool {
		match (self, other) {
			(Self::Anonymous(a), Self::Anonymous(b)) => a == b,
			(Self::Term(a), Self::Term(b)) => a == b,
			_ => false,
		}
	}
}

impl<V: Vocabulary> Eq for Resource<V>
where
	V::Iri: Eq,
	V::BlankId: Eq,
	V::Literal: Eq,
{
}

impl<V: Vocabulary> PartialOrd for Resource<V>
where
	V::Iri: PartialOrd,
	V::BlankId: PartialOrd,
	V::Literal: PartialOrd,
{
	fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
		match (self, other) {
			(Self::Anonymous(a), Self::Anonymous(b)) => a.partial_cmp(b),
			(Self::Anonymous(_), _) => Some(std::cmp::Ordering::Greater),
			(Self::Term(a), Self::Term(b)) => a.partial_cmp(b),
			(_, Self::Anonymous(_)) => Some(std::cmp::Ordering::Less),
		}
	}
}

impl<V: Vocabulary> Ord for Resource<V>
where
	V::Iri: Ord,
	V::BlankId: Ord,
	V::Literal: Ord,
{
	fn cmp(&self, other: &Self) -> std::cmp::Ordering {
		match (self, other) {
			(Self::Anonymous(a), Self::Anonymous(b)) => a.cmp(b),
			(Self::Anonymous(_), _) => std::cmp::Ordering::Greater,
			(Self::Term(a), Self::Term(b)) => a.cmp(b),
			(_, Self::Anonymous(_)) => std::cmp::Ordering::Less,
		}
	}
}

impl<V: Vocabulary> Clone for Resource<V>
where
	V::Iri: Clone,
	V::BlankId: Clone,
	V::Literal: Clone,
{
	fn clone(&self) -> Self {
		match self {
			Self::Anonymous(a) => Self::Anonymous(*a),
			Self::Term(a) => Self::Term(a.clone()),
		}
	}
}

impl<V: Vocabulary> Hash for Resource<V>
where
	V::Iri: Hash,
	V::BlankId: Hash,
	V::Literal: Hash,
{
	fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
		match self {
			Self::Anonymous(a) => {
				0u8.hash(state);
				a.hash(state)
			}
			Self::Term(t) => {
				1u8.hash(state);
				t.hash(state)
			}
		}
	}
}

impl<V: Vocabulary> Copy for Resource<V>
where
	V::Iri: Copy,
	V::BlankId: Copy,
	V::Literal: Copy,
{
}

#[derive(Educe)]
#[educe(Clone)]
pub struct IrisOf<'a, V: Vocabulary> {
	term: Option<&'a VocabTerm<V>>,
	additional_terms: Option<std::collections::hash_set::Iter<'a, VocabTerm<V>>>,
}

impl<'a, V: Vocabulary> Iterator for IrisOf<'a, V> {
	type Item = &'a V::Iri;

	fn next(&mut self) -> Option<Self::Item> {
		match self.term.take() {
			Some(Term::Id(Id::Iri(iri))) => Some(iri),
			_ => match self.additional_terms.as_mut() {
				Some(terms) => {
					for term in terms {
						if let Term::Id(Id::Iri(iri)) = term {
							return Some(iri);
						}
					}

					None
				}
				None => None,
			},
		}
	}
}

#[derive(Educe)]
#[educe(Clone)]
pub struct BlankIdsOf<'a, V: Vocabulary> {
	term: Option<&'a VocabTerm<V>>,
	additional_terms: Option<std::collections::hash_set::Iter<'a, VocabTerm<V>>>,
}

impl<'a, V: Vocabulary> Iterator for BlankIdsOf<'a, V> {
	type Item = &'a V::BlankId;

	fn next(&mut self) -> Option<Self::Item> {
		match self.term.take() {
			Some(Term::Id(Id::Blank(b))) => Some(b),
			_ => match self.additional_terms.as_mut() {
				Some(terms) => {
					for term in terms {
						if let Term::Id(Id::Blank(b)) = term {
							return Some(b);
						}
					}

					None
				}
				None => None,
			},
		}
	}
}

#[derive(Educe)]
#[educe(Debug(bound(V::Iri: Debug, V::BlankId: Debug, V::Literal: Debug)))]
pub struct LiteralsOf<'a, V: Vocabulary> {
	term: Option<&'a VocabTerm<V>>,
	additional_terms: Option<std::collections::hash_set::Iter<'a, VocabTerm<V>>>,
}

impl<'a, V: Vocabulary> Clone for LiteralsOf<'a, V>
where
	V::Iri: Clone,
	V::BlankId: Clone,
	V::Literal: Clone,
{
	fn clone(&self) -> Self {
		Self {
			term: self.term,
			additional_terms: self.additional_terms.clone(),
		}
	}
}

impl<'a, V: Vocabulary> Iterator for LiteralsOf<'a, V> {
	type Item = &'a V::Literal;

	fn next(&mut self) -> Option<Self::Item> {
		match self.term.take() {
			Some(Term::Literal(l)) => Some(l),
			_ => match self.additional_terms.as_mut() {
				Some(terms) => {
					for term in terms {
						if let Term::Literal(l) = term {
							return Some(l);
						}
					}

					None
				}
				None => None,
			},
		}
	}
}
