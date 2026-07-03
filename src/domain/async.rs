use std::future::Future;

use futures_lite::{stream, Stream};

use super::{ConstGenDomain, Cow, FiniteDomain};
use crate::domain::fallible::{TryConstGenDomain, TryDomain, TryFiniteDomain};
use crate::utils::{BorrowedResources, BorrowedResultResources, InfallibleIterator};

/// Finite domain.
pub trait AsyncFiniteDomain: TryDomain
where
	Self::Resource: ToOwned,
{
	type AsyncResources<'a>: Stream<Item = Result<Cow<'a, Self::Resource>, Self::Error>>
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

impl<I: FiniteDomain> AsyncFiniteDomain for I
where
	I::Resource: ToOwned,
{
	type AsyncResources<'a>
		= stream::Iter<InfallibleIterator<BorrowedResources<I::Resources<'a>>>>
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
		stream::iter(InfallibleIterator(BorrowedResources(self.resources())))
	}
}

impl<I: ConstGenDomain> AsyncConstGenDomain for I {
	async fn async_new_resource(&self) -> Result<Self::Resource, Self::Error> {
		Ok(ConstGenDomain::new_resource(self))
	}
}

/// Async fallible finite domain.
pub trait AsyncTryFiniteDomain: TryDomain
where
	Self::Resource: ToOwned,
{
	type AsyncTryResources<'a>: Stream<Item = Result<Cow<'a, Self::Resource>, Self::Error>>
	where
		Self: 'a;

	fn async_try_len(&self) -> impl Future<Output = Result<usize, Self::Error>>;

	fn async_try_is_empty(&self) -> impl Future<Output = Result<bool, Self::Error>> {
		async { Ok(self.async_try_len().await? == 0) }
	}

	fn async_try_contains(
		&self,
		a: &Self::Resource,
	) -> impl Future<Output = Result<bool, Self::Error>>;

	fn async_try_resources(&self) -> Self::AsyncTryResources<'_>;
}

/// Async fallible generative domain.
pub trait AsyncTryGenDomain: TryDomain {
	fn async_try_new_resource(
		&mut self,
	) -> impl Future<Output = Result<Self::Resource, Self::Error>>;
}

/// Async fallible const-generative domain.
pub trait AsyncTryConstGenDomain: TryDomain {
	fn async_try_new_resource(&self) -> impl Future<Output = Result<Self::Resource, Self::Error>>;
}

impl<I: AsyncTryConstGenDomain> AsyncTryGenDomain for I {
	fn async_try_new_resource(
		&mut self,
	) -> impl Future<Output = Result<Self::Resource, Self::Error>> {
		AsyncTryConstGenDomain::async_try_new_resource(self)
	}
}

impl<I: TryFiniteDomain> AsyncTryFiniteDomain for I
where
	I::Resource: ToOwned,
{
	type AsyncTryResources<'a>
		= stream::Iter<BorrowedResultResources<I::TryResources<'a>>>
	where
		Self: 'a;

	async fn async_try_len(&self) -> Result<usize, Self::Error> {
		self.try_len()
	}

	async fn async_try_is_empty(&self) -> Result<bool, Self::Error> {
		self.try_is_empty()
	}

	async fn async_try_contains(&self, a: &Self::Resource) -> Result<bool, Self::Error> {
		self.try_contains(a)
	}

	fn async_try_resources(&self) -> Self::AsyncTryResources<'_> {
		stream::iter(BorrowedResultResources(self.try_resources()))
	}
}

impl<I: TryConstGenDomain> AsyncTryConstGenDomain for I {
	async fn async_try_new_resource(&self) -> Result<Self::Resource, Self::Error> {
		TryConstGenDomain::try_new_resource(self)
	}
}
