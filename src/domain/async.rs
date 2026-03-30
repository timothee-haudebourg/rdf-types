use std::future::Future;

use futures_lite::{stream, Stream};

use super::{ConstGenDomain, EqDomain, FiniteDomain, MaybeOwned, VariableDomain};
use crate::domain::fallible::TryDomain;

pub trait AsyncEqDomain: TryDomain {
	fn async_is_eq(
		&self,
		a: &Self::Resource,
		b: &Self::Resource,
	) -> impl Future<Output = Result<bool, Self::Error>>;
}

pub trait AsyncVariableDomain: TryDomain {
	fn async_is_ground(
		&self,
		a: &Self::Resource,
	) -> impl Future<Output = Result<bool, Self::Error>>;

	fn async_is_variable(
		&self,
		a: &Self::Resource,
	) -> impl Future<Output = Result<bool, Self::Error>> {
		async { Ok(!self.async_is_ground(a).await?) }
	}
}

/// Finite domain.
pub trait AsyncFiniteDomain: TryDomain {
	type AsyncResources<'a>: Stream<Item = Result<MaybeOwned<'a, Self::Resource>, Self::Error>>
	where
		Self: 'a;

	fn async_len(&self) -> impl Future<Output = Result<usize, Self::Error>>;

	fn async_is_empty(&self) -> impl Future<Output = Result<bool, Self::Error>> {
		async { Ok(self.async_len().await? == 0) }
	}

	fn async_contains(&self, a: &Self::Resource)
		-> impl Future<Output = Result<bool, Self::Error>>;

	fn async_resources(&self) -> Self::AsyncResources<'_>;
}

/// Domain that can spawn fresh new resources.
pub trait AsyncGenDomain: TryDomain {
	/// Create a new resource.
	fn async_new_resource(&mut self) -> impl Future<Output = Result<Self::Resource, Self::Error>>;
}

/// Domain that can spawn fresh new resources from a shared reference.
pub trait AsyncConstGenDomain: TryDomain {
	/// Create a new resource.
	fn async_new_resource(&self) -> impl Future<Output = Result<Self::Resource, Self::Error>>;
}

impl<I: AsyncConstGenDomain> AsyncGenDomain for I {
	fn async_new_resource(&mut self) -> impl Future<Output = Result<Self::Resource, Self::Error>> {
		AsyncConstGenDomain::async_new_resource(self)
	}
}

// Blanket implementations for infallible domains.

impl<I: EqDomain> AsyncEqDomain for I {
	async fn async_is_eq(
		&self,
		a: &Self::Resource,
		b: &Self::Resource,
	) -> Result<bool, Self::Error> {
		Ok(self.is_eq(a, b))
	}
}

impl<I: VariableDomain> AsyncVariableDomain for I {
	async fn async_is_ground(&self, a: &Self::Resource) -> Result<bool, Self::Error> {
		Ok(self.is_ground(a))
	}

	async fn async_is_variable(&self, a: &Self::Resource) -> Result<bool, Self::Error> {
		Ok(self.is_variable(a))
	}
}

impl<I: FiniteDomain> AsyncFiniteDomain for I {
	type AsyncResources<'a>
		= stream::Iter<
		std::iter::Map<
			I::Resources<'a>,
			fn(MaybeOwned<'a, Self::Resource>) -> Result<MaybeOwned<'a, Self::Resource>, Self::Error>,
		>,
	>
	where
		Self: 'a;

	async fn async_len(&self) -> Result<usize, Self::Error> {
		Ok(self.len())
	}

	async fn async_is_empty(&self) -> Result<bool, Self::Error> {
		Ok(self.is_empty())
	}

	async fn async_contains(&self, a: &Self::Resource) -> Result<bool, Self::Error> {
		Ok(self.contains(a))
	}

	fn async_resources(&self) -> Self::AsyncResources<'_> {
		stream::iter(self.resources().map(Ok as fn(_) -> _))
	}
}

impl<I: ConstGenDomain> AsyncConstGenDomain for I {
	async fn async_new_resource(&self) -> Result<Self::Resource, Self::Error> {
		Ok(ConstGenDomain::new_resource(self))
	}
}
