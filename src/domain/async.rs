use futures_lite::{Stream, stream};

use crate::domain::fallible::{TryConstGenDomain, TryDomain, TryFiniteDomain};

/// Finite domain.
pub trait AsyncFiniteDomain: TryDomain {
	type AsyncResources<'a>: Stream<Item = Result<Self::Resource, Self::Error>>
	where
		Self: 'a;

	async fn async_len(&self) -> Result<usize, Self::Error>;

	async fn async_is_empty(&self) -> Result<bool, Self::Error> {
		Ok(self.async_len().await? == 0)
	}

	async fn async_contains(&self, a: &Self::Resource) -> Result<bool, Self::Error>;

	async fn async_resources(&self) -> Result<Self::AsyncResources<'_>, Self::Error>;
}

/// Domain that can spawn fresh new resources.
pub trait AsyncGenDomain: TryDomain {
	/// Create a new resource.
	async fn async_new_resource(&mut self) -> Result<Self::Resource, Self::Error>;
}

/// Domain that can spawn fresh new resources from a shared reference.
pub trait AsyncConstGenDomain: TryDomain {
	/// Create a new resource.
	async fn async_new_resource(&self) -> Result<Self::Resource, Self::Error>;
}

impl<I: AsyncConstGenDomain> AsyncGenDomain for I {
	async fn async_new_resource(&mut self) -> Result<Self::Resource, Self::Error> {
		AsyncConstGenDomain::async_new_resource(self).await
	}
}

impl<I: TryFiniteDomain> AsyncFiniteDomain for I
where
	I::Resource: Clone,
{
	type AsyncResources<'a>
		= stream::Iter<I::TryResources<'a>>
	where
		Self: 'a;

	async fn async_len(&self) -> Result<usize, Self::Error> {
		self.try_len()
	}

	async fn async_is_empty(&self) -> Result<bool, Self::Error> {
		self.try_is_empty()
	}

	async fn async_contains(&self, a: &Self::Resource) -> Result<bool, Self::Error> {
		self.try_contains(a)
	}

	async fn async_resources(&self) -> Result<Self::AsyncResources<'_>, Self::Error> {
		self.try_resources().map(stream::iter)
	}
}

impl<I: TryConstGenDomain> AsyncConstGenDomain for I {
	async fn async_new_resource(&self) -> Result<Self::Resource, Self::Error> {
		TryConstGenDomain::try_new_resource(self)
	}
}

/// Async fallible finite domain.
pub trait AsyncTryFiniteDomain: TryDomain {
	type AsyncTryResources<'a>: Stream<Item = Result<Self::Resource, Self::Error>>
	where
		Self: 'a;

	async fn async_try_len(&self) -> Result<usize, Self::Error>;

	async fn async_try_is_empty(&self) -> Result<bool, Self::Error> {
		Ok(self.async_try_len().await? == 0)
	}

	async fn async_try_contains(&self, a: &Self::Resource) -> Result<bool, Self::Error>;

	async fn async_try_resources(&self) -> Result<Self::AsyncTryResources<'_>, Self::Error>;
}
