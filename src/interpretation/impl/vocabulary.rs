use educe::Educe;
use std::fmt::Debug;
use std::hash::Hash;
use std::marker::PhantomData;

use crate::interpretation::{
	BlankIdInterpretation, IriInterpretation, LiteralInterpretation, ReverseBlankIdInterpretation,
};
use crate::{
	interpretation::{ReverseIriInterpretation, ReverseLiteralInterpretation},
	vocabulary::{BlankIdVocabulary, IriVocabulary, LiteralVocabulary},
	Id, Interpretation, Term, Vocabulary,
};

type VocabTerm<V> = Term<
	Id<<V as IriVocabulary>::Iri, <V as BlankIdVocabulary>::BlankId>,
	<V as LiteralVocabulary>::Literal,
>;

/// Vocabulary interpretation.
///
/// This type is a special sort of interpretation where every term is
/// interpreted as itself.
pub struct VocabularyInterpretation<V: Vocabulary>(PhantomData<V>);

impl<V: Vocabulary> Default for VocabularyInterpretation<V> {
	fn default() -> Self {
		Self(PhantomData)
	}
}

impl<V: Vocabulary> VocabularyInterpretation<V> {
	pub fn new() -> Self {
		Self::default()
	}
}

impl<V: Vocabulary> Interpretation for VocabularyInterpretation<V> {
	type Resource = VocabTerm<V>;
}

impl<V: Vocabulary> IriInterpretation<V::Iri> for VocabularyInterpretation<V>
where
	V::Iri: Clone,
{
	fn iri_interpretation(&self, iri: &V::Iri) -> Option<Self::Resource> {
		Some(Term::iri(iri.clone()))
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
		IrisOf(id.as_iri())
	}
}

impl<V: Vocabulary> BlankIdInterpretation<V::BlankId> for VocabularyInterpretation<V>
where
	V::BlankId: Clone,
{
	fn blank_id_interpretation(&self, blank_id: &V::BlankId) -> Option<Self::Resource> {
		Some(Term::blank(blank_id.clone()))
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
		BlankIdsOf(id.as_blank())
	}
}

impl<V: Vocabulary> LiteralInterpretation<V::Literal> for VocabularyInterpretation<V>
where
	V::Literal: Clone,
{
	fn literal_interpretation(&self, literal: &V::Literal) -> Option<Self::Resource> {
		Some(Term::Literal(literal.clone()))
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
		LiteralsOf(id.as_literal())
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
#[educe(Debug(bound(V::Iri: Debug)))]
pub struct IrisOf<'a, V: IriVocabulary>(Option<&'a V::Iri>);

impl<'a, V: Vocabulary> Iterator for IrisOf<'a, V> {
	type Item = &'a V::Iri;

	fn next(&mut self) -> Option<Self::Item> {
		self.0.take()
	}
}

impl<'a, V: Vocabulary> Clone for IrisOf<'a, V> {
	fn clone(&self) -> Self {
		*self
	}
}

impl<'a, V: Vocabulary> Copy for IrisOf<'a, V> {}

#[derive(Educe)]
#[educe(Debug(bound(V::BlankId: Debug)))]
pub struct BlankIdsOf<'a, V: Vocabulary>(Option<&'a V::BlankId>);

impl<'a, V: Vocabulary> Iterator for BlankIdsOf<'a, V> {
	type Item = &'a V::BlankId;

	fn next(&mut self) -> Option<Self::Item> {
		self.0.take()
	}
}

impl<'a, V: Vocabulary> Clone for BlankIdsOf<'a, V> {
	fn clone(&self) -> Self {
		*self
	}
}

impl<'a, V: Vocabulary> Copy for BlankIdsOf<'a, V> {}

#[derive(Educe)]
#[educe(Debug(bound(V::Literal: Debug)))]
pub struct LiteralsOf<'a, V: Vocabulary>(Option<&'a V::Literal>);

impl<'a, V: Vocabulary> Clone for LiteralsOf<'a, V> {
	fn clone(&self) -> Self {
		*self
	}
}

impl<'a, V: Vocabulary> Copy for LiteralsOf<'a, V> {}

impl<'a, V: Vocabulary> Iterator for LiteralsOf<'a, V> {
	type Item = &'a V::Literal;

	fn next(&mut self) -> Option<Self::Item> {
		self.0.take()
	}
}
