use std::collections::{BTreeMap, BTreeSet, btree_map::Entry};

use crate::pattern::{AsPattern, Pattern};
use crate::{Quad, dataset::FiniteDataset};

/// Checks that there is an isomorphism between the datasets `a` and `b`.
///
/// There is an isomorphism if there exists a blank node identifier bijection
/// between `a` and `b`.
/// This is equivalent to `find_bijection(a, b).is_some()`.
pub fn are_isomorphic<R, A, B>(a: &A, b: &B) -> bool
where
	R: Ord + AsPattern<Var: Ord, Ground: PartialEq>,
	A: FiniteDataset<Resource = R>,
	B: FiniteDataset<Resource = R>,
{
	find_bijection(a, b).is_some()
}

/// Maps every blank node identifier (or variable) found in a dataset to its
/// [`BlankSignature`].
type BlankSignatureMap<'d, R> = BTreeMap<&'d <R as AsPattern>::Var, BlankSignature<'d, R>>;

/// Variant of [`BlankSignatureMap`] borrowing its signatures rather than
/// owning them, as produced by [`split_by_size`].
type BlankSignatureMapRef<'s, 'd, R> =
	BTreeMap<&'d <R as AsPattern>::Var, &'s BlankSignature<'d, R>>;

/// Finds a blank node identifier bijection from `a` to `b`.
/// If such bijection exists,
/// there is an isomorphism between `a` and `b`.
pub fn find_bijection<'a, 'b, R, A, B>(a: &'a A, b: &'b B) -> Option<BTreeBijection<'a, 'b, R::Var>>
where
	R: 'a + 'b + Ord + AsPattern<Var: Ord, Ground: PartialEq>,
	A: FiniteDataset<Resource = R>,
	B: FiniteDataset<Resource = R>,
{
	if a.quads_count() != b.quads_count() {
		return None;
	}

	let a_blank_count = a.quads().fold(0, |c, q| c + blank_count(q));
	let b_blank_count = b.quads().fold(0, |c, q| c + blank_count(q));

	if a_blank_count != b_blank_count {
		return None;
	}

	// Step 1: collect signatures.
	let mut a_blanks_map = BTreeMap::new();
	let mut b_blanks_map = BTreeMap::new();
	collect_signatures(&mut a_blanks_map, a);
	collect_signatures(&mut b_blanks_map, b);

	if a_blanks_map.len() != b_blanks_map.len() {
		return None;
	}

	// Step 2: split by sizes.
	let a_groups = split_by_size(&a_blanks_map);
	let b_groups = split_by_size(&b_blanks_map);

	if a_groups.len() != b_groups.len() {
		return None;
	}

	if !a_groups.iter().all(|(len, _)| b_groups.contains_key(len)) {
		return None;
	}

	// Step 3: find candidates for each blank id.
	let mut candidates = BTreeMap::new();
	for (len, a_group) in a_groups {
		let b_group = b_groups.get(&len).unwrap();

		for (a_blank_id, a_sig) in a_group {
			let mut a_blank_id_candidates = BTreeSet::new();
			for (b_blank_id, b_sig) in b_group {
				if a_sig.matches(b_sig) {
					a_blank_id_candidates.insert(*b_blank_id);
				}
			}

			if a_blank_id_candidates.is_empty() {
				return None;
			}

			candidates.insert(a_blank_id, a_blank_id_candidates);
		}
	}

	BTreeBijection::new().find_from_candidates(candidates.iter(), &a_blanks_map, &b_blanks_map)
}

/// Checks whether `a` and `b` can denote the same resource.
///
/// Two ground values match if they are equal, and any two variables always
/// match (since either can be substituted for the other by the bijection).
fn resource_matches<R>(a: &R, b: &R) -> bool
where
	R: AsPattern<Ground: PartialEq>,
{
	match (a.as_pattern(), b.as_pattern()) {
		(Pattern::Ground(a), Pattern::Ground(b)) => a == b,
		(Pattern::Var(_), Pattern::Var(_)) => true,
		_ => false,
	}
}

/// Checks whether `a` and `b` can denote the same quad, component-wise, using
/// [`resource_matches`].
fn quad_matches<R>(a: Quad<&R>, b: Quad<&R>) -> bool
where
	R: AsPattern<Ground: PartialEq>,
{
	resource_matches(a.0, b.0)
		&& resource_matches(a.1, b.1)
		&& resource_matches(a.2, b.2)
		&& match (a.3, b.3) {
			(Some(a), Some(b)) => resource_matches(a, b),
			(None, None) => true,
			_ => false,
		}
}

/// Counts how many components of the given quad are variables.
fn blank_count<R: AsPattern>(Quad(s, p, o, g): Quad<&R>) -> usize {
	let mut r = 0;

	if s.is_var() {
		r += 1
	}

	if p.is_var() {
		r += 1
	}

	if o.is_var() {
		r += 1
	}

	if let Some(g) = g {
		if g.is_var() {
			r += 1
		}
	}

	r
}

/// Collects the [`BlankSignature`] of every variable occurring in `ds` into
/// `map`.
fn collect_signatures<'d, R, D>(map: &mut BlankSignatureMap<'d, R>, ds: &'d D)
where
	R: Ord + AsPattern<Var: Ord>,
	D: FiniteDataset<Resource = R>,
{
	for quad in ds.quads() {
		if let Pattern::Var(x) = quad.0.as_pattern() {
			map.entry(x).or_default().insert(quad);
		}

		if let Pattern::Var(x) = quad.1.as_pattern() {
			map.entry(x).or_default().insert(quad);
		}

		if let Pattern::Var(x) = quad.2.as_pattern() {
			map.entry(x).or_default().insert(quad);
		}

		if let Some(g) = quad.3 {
			if let Pattern::Var(x) = g.as_pattern() {
				map.entry(x).or_default().insert(quad);
			}
		}
	}

	for sig in map.values_mut() {
		sig.0.sort_unstable();
	}
}

/// Groups the given blank node identifiers by the size (number of quads) of
/// their signature.
fn split_by_size<'s, 'd, R>(
	blanks: &'s BlankSignatureMap<'d, R>,
) -> BTreeMap<usize, BlankSignatureMapRef<'s, 'd, R>>
where
	R: AsPattern<Var: Ord>,
{
	let mut result = BTreeMap::new();

	for (blank_id, sig) in blanks {
		match result.entry(sig.len()) {
			Entry::Vacant(entry) => {
				let mut map = BTreeMap::new();
				map.insert(*blank_id, sig);
				entry.insert(map);
			}
			Entry::Occupied(mut entry) => {
				entry.get_mut().insert(*blank_id, sig);
			}
		}
	}

	result
}

/// Blank node identifier bijection
/// between two (isomorphic) datasets.
pub struct BTreeBijection<'a, 'b, T: ?Sized> {
	/// Maps each blank node identifier of the first dataset to the
	/// corresponding blank node identifier of the second dataset.
	pub forward: BTreeMap<&'a T, &'b T>,

	/// Maps each blank node identifier of the second dataset to the
	/// corresponding blank node identifier of the first dataset.
	pub backward: BTreeMap<&'b T, &'a T>,
}

impl<T: ?Sized> Clone for BTreeBijection<'_, '_, T> {
	fn clone(&self) -> Self {
		Self {
			forward: self.forward.clone(),
			backward: self.backward.clone(),
		}
	}
}

impl<T: ?Sized> BTreeBijection<'_, '_, T> {
	fn new() -> Self {
		Self {
			forward: BTreeMap::new(),
			backward: BTreeMap::new(),
		}
	}
}

impl<'a, 'b, T: ?Sized + Ord> BTreeBijection<'a, 'b, T> {
	/// Extends the bijection with a new `a <-> b` blank node identifier pair.
	fn insert(&mut self, a: &'a T, b: &'b T) {
		self.forward.insert(a, b);
		self.backward.insert(b, a);
	}

	/// Checks whether `a` and `b` are consistent with this (partial)
	/// bijection: either they are equal, or the bijection already maps one
	/// to the other, or neither is mapped yet.
	fn resource_matches_with<R>(&self, a: &R, b: &R) -> bool
	where
		R: PartialEq + AsPattern<Var = T>,
	{
		if a == b {
			return true;
		}

		match (a.as_pattern(), b.as_pattern()) {
			(Pattern::Var(a), Pattern::Var(b)) => match self.forward.get(a) {
				Some(c) => **c == *b,
				None => match self.backward.get(b) {
					Some(c) => *a == **c,
					None => true,
				},
			},
			_ => false,
		}
	}

	/// Checks whether `a` and `b` are consistent with this (partial)
	/// bijection, component-wise, using [`Self::resource_matches_with`].
	fn quad_matches_with<R>(&self, a: Quad<&R>, b: Quad<&R>) -> bool
	where
		R: PartialEq + AsPattern<Var = T>,
	{
		self.resource_matches_with(a.0, b.0)
			&& self.resource_matches_with(a.1, b.1)
			&& self.resource_matches_with(a.2, b.2)
			&& match (a.3, b.3) {
				(Some(a), Some(b)) => self.resource_matches_with(a, b),
				(None, None) => true,
				_ => false,
			}
	}

	/// Checks whether the quads of `a` and `b` are consistent with this
	/// (partial) bijection, matching each quad of `a` with a distinct quad of
	/// `b` using [`Self::quad_matches_with`].
	fn signature_matches_with<R>(
		&self,
		a: &BlankSignature<'a, R>,
		b: &BlankSignature<'b, R>,
	) -> bool
	where
		R: PartialEq + AsPattern<Var = T>,
	{
		if a.len() == b.len() {
			let mut other: Vec<_> = b.0.iter().cloned().map(Some).collect();
			'next_quad: for quad in a.0.iter() {
				for other_quad in &mut other {
					if let Some(oq) = other_quad {
						if self.quad_matches_with(quad.as_deref(), oq.as_deref()) {
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

	/// Extends this (partial) bijection into a full bijection by picking, for
	/// each remaining `(a_blank_id, b_candidates)` pair, a candidate that is
	/// consistent with the bijection built so far, backtracking whenever a
	/// choice leads to a dead end.
	fn find_from_candidates<R>(
		self,
		mut candidates: std::collections::btree_map::Iter<&'a T, BTreeSet<&'b T>>,
		a: &BTreeMap<&'a T, BlankSignature<'a, R>>,
		b: &BTreeMap<&'b T, BlankSignature<'b, R>>,
	) -> Option<Self>
	where
		R: PartialEq + AsPattern<Var = T>,
	{
		match candidates.next() {
			Some((a_blank_id, b_candidates)) => {
				for b_candidate in b_candidates {
					if !self.backward.contains_key(b_candidate) {
						let mut new_sigma = self.clone();
						new_sigma.insert(a_blank_id, b_candidate);
						if new_sigma.signature_matches_with(
							a.get(a_blank_id).unwrap(),
							b.get(b_candidate).unwrap(),
						) {
							if let Some(final_sigma) =
								new_sigma.find_from_candidates(candidates.clone(), a, b)
							{
								return Some(final_sigma);
							}
						}
					}
				}

				None
			}
			None => Some(self),
		}
	}
}

/// Signature of a blank node identifier: the list of quads it occurs in.
///
/// Used to narrow down, for each blank node identifier of a dataset, the set
/// of candidate blank node identifiers it could be mapped to in the other
/// dataset.
struct BlankSignature<'a, R>(Vec<Quad<&'a R>>);

impl<R> Default for BlankSignature<'_, R> {
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
}

impl<'a, R> BlankSignature<'a, R>
where
	R: AsPattern<Ground: PartialEq>,
{
	/// Checks whether `self` and `other` could be the signatures of two
	/// blank node identifiers mapped to one another, ignoring any bijection
	/// built so far (any two variables are considered a possible match, see
	/// [`resource_matches`]).
	fn matches(&self, other: &BlankSignature<R>) -> bool {
		if self.len() == other.len() {
			let mut other: Vec<_> = other.0.iter().cloned().map(Some).collect();
			'next_quad: for quad in &self.0 {
				for other_quad in &mut other {
					if let Some(oq) = other_quad {
						if quad_matches(quad.as_deref(), oq.as_deref()) {
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
