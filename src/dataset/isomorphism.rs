use std::collections::{btree_map::Entry, BTreeMap, BTreeSet};

use educe::Educe;

use crate::{
	dataset::TraversableDataset,
	interpretation::{ReverseIriInterpretation, ReverseLiteralInterpretation},
	Quad, Term,
};

/// Checks that there is an isomorphism between the datasets `a` and `b`.
///
/// There is an isomorphism if there exists a blank node identifier bijection
/// between `a` and `b`.
/// This is equivalent to `find_bijection_with(a, b).is_some()`.
pub fn are_isomorphic<A, B>(a: &A, b: &B) -> bool
where
	A: TraversableDataset<Resource = Term>,
	B: TraversableDataset<Resource = Term>,
{
	are_isomorphic_with(&(), a, b)
}

/// Checks that there is an isomorphism between the datasets `a` and `b`.
///
/// There is an isomorphism if there exists a blank node identifier bijection
/// between `a` and `b`.
/// This is equivalent to `find_bijection_with(a, b).is_some()`.
pub fn are_isomorphic_with<I, A, B>(interpretation: &I, a: &A, b: &B) -> bool
where
	I: ReverseIriInterpretation + ReverseLiteralInterpretation,
	I::Resource: Ord,
	I::Iri: PartialEq,
	I::Literal: PartialEq,
	A: TraversableDataset<Resource = I::Resource>,
	B: TraversableDataset<Resource = I::Resource>,
{
	find_bijection_with(interpretation, a, b).is_some()
}

/// Finds a blank node identifier bijection between from `a` to `b`.
/// If such bijection exists,
/// there is an isomorphism between `a` and `b`.
pub fn find_bijection<'a, 'b, A, B>(a: &'a A, b: &'b B) -> Option<BTreeBijection<'a, 'b>>
where
	A: TraversableDataset<Resource = Term>,
	B: TraversableDataset<Resource = Term>,
{
	find_bijection_with(&(), a, b)
}

/// Finds a blank node identifier bijection between from `a` to `b`.
/// If such bijection exists,
/// there is an isomorphism between `a` and `b`.
pub fn find_bijection_with<'a, 'b, I, A, B>(
	interpretation: &I,
	a: &'a A,
	b: &'b B,
) -> Option<BTreeBijection<'a, 'b, I::Resource>>
where
	I: ReverseIriInterpretation + ReverseLiteralInterpretation,
	I::Resource: Ord,
	I::Iri: PartialEq,
	I::Literal: PartialEq,
	A: TraversableDataset<Resource = I::Resource>,
	B: TraversableDataset<Resource = I::Resource>,
{
	if a.quads_count() != b.quads_count() {
		return None;
	}

	let a_blank_count = a.quads().fold(0, |c, q| c + blank_count(interpretation, q));
	let b_blank_count = b.quads().fold(0, |c, q| c + blank_count(interpretation, q));

	if a_blank_count != b_blank_count {
		return None;
	}

	// Step 1: collect signatures.
	let mut a_blanks_map = BTreeMap::new();
	let mut b_blanks_map = BTreeMap::new();
	collect_signatures(interpretation, &mut a_blanks_map, a);
	collect_signatures(interpretation, &mut b_blanks_map, b);

	if a_blanks_map.len() != b_blanks_map.len() {
		eprintln!("different blank node count");
		return None;
	}

	// Step 2: split by sizes.
	let a_groups = split_by_size(&a_blanks_map);
	let b_groups = split_by_size(&b_blanks_map);

	if a_groups.len() != b_groups.len() {
		eprintln!("different group count");
		return None;
	}

	if !a_groups.iter().all(|(len, _)| b_groups.contains_key(len)) {
		eprintln!("different group lengths");
		return None;
	}

	// Step 3: find candidates for each blank id.
	let mut candidates: BTreeMap<&'a I::Resource, BTreeSet<&'b I::Resource>> = BTreeMap::new();
	for (len, a_group) in a_groups {
		let b_group = b_groups.get(&len).unwrap();

		for (a_blank_id, a_sig) in a_group {
			let mut a_blank_id_candidates = BTreeSet::new();
			for (b_blank_id, b_sig) in b_group {
				if a_sig.matches(interpretation, b_sig) {
					a_blank_id_candidates.insert(*b_blank_id);
				}
			}

			if a_blank_id_candidates.is_empty() {
				eprintln!("no candidates found for blank id");
				return None;
			}

			candidates.insert(a_blank_id, a_blank_id_candidates);
		}
	}

	BTreeBijection::new().find_from_candidates(
		interpretation,
		candidates.iter(),
		&a_blanks_map,
		&b_blanks_map,
	)
}

fn resource_matches<I>(interpretation: &I, a: &I::Resource, b: &I::Resource) -> bool
where
	I: ReverseIriInterpretation + ReverseLiteralInterpretation,
	I::Iri: PartialEq,
	I::Literal: PartialEq,
{
	for a in interpretation.iris_of(a) {
		for b in interpretation.iris_of(b) {
			if a == b {
				return true;
			}
		}
	}

	for a in interpretation.literals_of(a) {
		for b in interpretation.literals_of(b) {
			if a == b {
				return true;
			}
		}
	}

	is_blank(interpretation, a) && is_blank(interpretation, b)
}

fn quad_matches<I>(interpretation: &I, a: Quad<&I::Resource>, b: Quad<&I::Resource>) -> bool
where
	I: ReverseIriInterpretation + ReverseLiteralInterpretation,
	I::Iri: PartialEq,
	I::Literal: PartialEq,
{
	resource_matches(interpretation, a.0, b.0)
		&& resource_matches(interpretation, a.1, b.1)
		&& resource_matches(interpretation, a.2, b.2)
		&& match (a.3, b.3) {
			(Some(a), Some(b)) => resource_matches(interpretation, a, b),
			(None, None) => true,
			_ => false,
		}
}

fn is_blank<I>(interpretation: &I, r: &I::Resource) -> bool
where
	I: ReverseIriInterpretation + ReverseLiteralInterpretation,
{
	interpretation.iris_of(r).next().is_none() && interpretation.literals_of(r).next().is_none()
}

fn blank_count<I>(interpretation: &I, Quad(s, p, o, g): Quad<&I::Resource>) -> usize
where
	I: ReverseIriInterpretation + ReverseLiteralInterpretation,
{
	let mut r = 0;

	if is_blank(interpretation, s) {
		r += 1
	}

	if is_blank(interpretation, p) {
		r += 1
	}

	if is_blank(interpretation, o) {
		r += 1
	}

	if let Some(g) = g {
		if is_blank(interpretation, g) {
			r += 1
		}
	}

	r
}

fn collect_signatures<'d, I, D>(
	interpretation: &I,
	map: &mut BTreeMap<&'d I::Resource, BlankSignature<'d, I::Resource>>,
	ds: &'d D,
) where
	I: ReverseIriInterpretation + ReverseLiteralInterpretation,
	I::Resource: Ord,
	I::Iri: PartialEq,
	I::Literal: PartialEq,
	D: TraversableDataset<Resource = I::Resource>,
{
	for quad in ds.quads() {
		if is_blank(interpretation, quad.0) {
			map.entry(quad.0).or_default().insert(quad);
		}

		if is_blank(interpretation, quad.1) {
			map.entry(quad.1).or_default().insert(quad);
		}

		if is_blank(interpretation, quad.2) {
			map.entry(quad.2).or_default().insert(quad);
		}

		if let Some(g) = quad.3 {
			if is_blank(interpretation, g) {
				map.entry(g).or_default().insert(quad);
			}
		}
	}

	for sig in map.values_mut() {
		sig.0.sort_unstable();
	}
}

fn split_by_size<'s, 'd, R>(
	blanks: &'s BTreeMap<&'d R, BlankSignature<'d, R>>,
) -> BTreeMap<usize, BTreeMap<&'d R, &'s BlankSignature<'d, R>>>
where
	R: Ord,
{
	let mut result = BTreeMap::new();

	for (&blank_id, sig) in blanks {
		match result.entry(sig.len()) {
			Entry::Vacant(entry) => {
				let mut map = BTreeMap::new();
				map.insert(blank_id, sig);
				entry.insert(map);
			}
			Entry::Occupied(mut entry) => {
				entry.get_mut().insert(blank_id, sig);
			}
		}
	}

	result
}

/// Blank node identifier bijection
/// between two (isomorphic) datasets.
#[derive(Educe)]
#[educe(Clone)]
pub struct BTreeBijection<'a, 'b, R = Term> {
	pub forward: BTreeMap<&'a R, &'b R>,
	pub backward: BTreeMap<&'b R, &'a R>,
}

impl<'a, 'b, R> BTreeBijection<'a, 'b, R> {
	fn new() -> Self {
		Self {
			forward: BTreeMap::new(),
			backward: BTreeMap::new(),
		}
	}
}

impl<'a, 'b, R: Ord> BTreeBijection<'a, 'b, R> {
	fn insert(&mut self, a: &'a R, b: &'b R) {
		self.forward.insert(a, b);
		self.backward.insert(b, a);
	}

	fn resource_matches_with<I>(&self, interpretation: &I, a: &'a R, b: &'b R) -> bool
	where
		I: ReverseIriInterpretation<Resource = R> + ReverseLiteralInterpretation,
		I::Iri: PartialEq,
		I::Literal: PartialEq,
	{
		for a in interpretation.iris_of(a) {
			for b in interpretation.iris_of(b) {
				if a == b {
					return true;
				}
			}
		}

		for a in interpretation.literals_of(a) {
			for b in interpretation.literals_of(b) {
				if a == b {
					return true;
				}
			}
		}

		match self.forward.get(a) {
			Some(&c) => c == b,
			None => match self.backward.get(b) {
				Some(&c) => a == c,
				None => true,
			},
		}
	}

	fn quad_matches_with<I>(&self, interpretation: &I, a: Quad<&'a R>, b: Quad<&'b R>) -> bool
	where
		I: ReverseIriInterpretation<Resource = R> + ReverseLiteralInterpretation,
		I::Iri: PartialEq,
		I::Literal: PartialEq,
	{
		self.resource_matches_with(interpretation, a.0, b.0)
			&& self.resource_matches_with(interpretation, a.1, b.1)
			&& self.resource_matches_with(interpretation, a.2, b.2)
			&& match (a.3, b.3) {
				(Some(a), Some(b)) => self.resource_matches_with(interpretation, a, b),
				(None, None) => true,
				_ => false,
			}
	}

	fn signature_matches_with<I>(
		&self,
		interpretation: &I,
		a: &BlankSignature<'a, R>,
		b: &BlankSignature<'b, R>,
	) -> bool
	where
		I: ReverseIriInterpretation<Resource = R> + ReverseLiteralInterpretation,
		I::Iri: PartialEq,
		I::Literal: PartialEq,
	{
		if a.len() == b.len() {
			let mut other: Vec<_> = b.0.iter().map(|q| Some(*q)).collect();
			'next_quad: for quad in a.0.iter() {
				for other_quad in &mut other {
					if let Some(oq) = other_quad {
						if self.quad_matches_with(interpretation, *quad, *oq) {
							// eprintln!("matching {} with {}", quad, oq);
							other_quad.take();
							continue 'next_quad;
						}
					}
				}

				// eprintln!("could not match {} with anything", quad);
				return false;
			}

			true
		} else {
			false
		}
	}

	fn find_from_candidates<I>(
		self,
		interpretation: &I,
		mut candidates: std::collections::btree_map::Iter<&'a R, BTreeSet<&'b R>>,
		a: &BTreeMap<&'a R, BlankSignature<'a, R>>,
		b: &BTreeMap<&'b R, BlankSignature<'b, R>>,
	) -> Option<Self>
	where
		I: ReverseIriInterpretation<Resource = R> + ReverseLiteralInterpretation,
		I::Iri: PartialEq,
		I::Literal: PartialEq,
	{
		match candidates.next() {
			Some((a_blank_id, b_candidates)) => {
				for b_candidate in b_candidates {
					if !self.backward.contains_key(b_candidate) {
						// eprintln!("analyzing candidate {} for {}", b_candidate, a_blank_id);

						let mut new_sigma = self.clone();
						new_sigma.insert(a_blank_id, b_candidate);
						if new_sigma.signature_matches_with(
							interpretation,
							a.get(a_blank_id).unwrap(),
							b.get(b_candidate).unwrap(),
						) {
							// eprintln!("this is a valid candidate. continuing.");
							if let Some(final_sigma) = new_sigma.find_from_candidates(
								interpretation,
								candidates.clone(),
								a,
								b,
							) {
								return Some(final_sigma);
							}
							// eprintln!("it didn't work out in the end. next candidate for {}.", a_blank_id);
						}
					}
				}

				// eprintln!("no valid candidate for {}", a_blank_id);
				None
			}
			None => Some(self),
		}
	}
}

/// Signature of a blank node identifier.
#[allow(clippy::type_complexity)]
struct BlankSignature<'a, R>(Vec<Quad<&'a R>>);

impl<'a, R> Default for BlankSignature<'a, R> {
	fn default() -> Self {
		Self(Vec::new())
	}
}

impl<'a, R> BlankSignature<'a, R> {
	fn insert(&mut self, quad: Quad<&'a R>) {
		self.0.push(quad)
	}

	fn len(&self) -> usize {
		self.0.len()
	}

	fn matches<I>(&self, interpretation: &I, other: &BlankSignature<R>) -> bool
	where
		I: ReverseIriInterpretation<Resource = R> + ReverseLiteralInterpretation,
		I::Iri: PartialEq,
		I::Literal: PartialEq,
	{
		if self.len() == other.len() {
			let mut other: Vec<_> = other.0.iter().map(|q| Some(*q)).collect();
			'next_quad: for quad in &self.0 {
				for other_quad in &mut other {
					if let Some(oq) = other_quad {
						if quad_matches(interpretation, *quad, *oq) {
							other_quad.take();
							continue 'next_quad;
						}
					}
				}

				return false;
			}

			true
		} else {
			false
		}
	}
}
