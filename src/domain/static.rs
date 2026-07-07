//! Trivial [`Domain`] implementation, only fixing the resource type.
use std::marker::PhantomData;

use super::Domain;

/// Trivial domain implementation that just fixes the resource type `R`,
/// without tracking any actual resource.
///
/// Useful whenever a [`Domain`] is required by a trait bound but no
/// domain-specific behavior (finiteness, generation, etc.) is needed, or as
/// a default/building block for other, more specific domains.
pub struct StaticDomain<R>(PhantomData<R>);

impl<R> Default for StaticDomain<R> {
	fn default() -> Self {
		Self(PhantomData)
	}
}

impl<R> StaticDomain<R> {
	/// Creates a new static domain.
	pub fn new() -> Self {
		Self::default()
	}
}

impl<R> Domain for StaticDomain<R> {
	type Resource = R;
}
