use std::marker::PhantomData;

use super::Domain;

pub struct StaticDomain<R>(PhantomData<R>);

impl<R> Default for StaticDomain<R> {
	fn default() -> Self {
		Self(PhantomData)
	}
}

impl<R> StaticDomain<R> {
	pub fn new() -> Self {
		Self::default()
	}
}

impl<R> Domain for StaticDomain<R> {
	type Resource = R;
}
