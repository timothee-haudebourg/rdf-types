use std::marker::PhantomData;

use crate::domain::{EqDomain, VariableDomain};

use super::Domain;

pub trait MaybeVariable {
	fn is_ground(&self) -> bool;

	fn is_variable(&self) -> bool {
		!self.is_ground()
	}
}

pub struct SealedDomain<R>(PhantomData<R>);

impl<R> Default for SealedDomain<R> {
	fn default() -> Self {
		Self(PhantomData)
	}
}

impl<R> SealedDomain<R> {
	pub fn new() -> Self {
		Self::default()
	}
}

impl<R: ToOwned> Domain for SealedDomain<R> {
	type Resource = R;
}

impl<R: ToOwned> EqDomain for SealedDomain<R>
where
	R: Eq,
{
	fn is_eq(&self, a: &Self::Resource, b: &Self::Resource) -> bool {
		a == b
	}
}

impl<R: ToOwned> VariableDomain for SealedDomain<R>
where
	R: MaybeVariable,
{
	fn is_ground(&self, a: &Self::Resource) -> bool {
		a.is_ground()
	}

	fn is_variable(&self, a: &Self::Resource) -> bool {
		a.is_variable()
	}
}
