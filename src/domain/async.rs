//! Asynchronous counterparts of the [`Domain`](super::Domain) traits, for
//! domains backed by asynchronous storage.
use futures_lite::{Stream, stream};

use crate::domain::fallible::{TryConstGenDomain, TryDomain, TryFiniteDomain};

/// Asynchronous finite domain.
pub trait AsyncFiniteDomain: TryDomain {
	/// Asynchronous fallible resources stream.
	type AsyncResources<'a>: Stream<Item = Result<Self::Resource, Self::Error>>
	where
		Self: 'a;

	/// Returns the number of resources in the domain.
	async fn async_len(&self) -> Result<usize, Self::Error>;

	/// Checks if the domain has no resources.
	async fn async_is_empty(&self) -> Result<bool, Self::Error> {
		Ok(self.async_len().await? == 0)
	}

	/// Checks if the given resource belongs to the domain.
	async fn async_contains(&self, a: &Self::Resource) -> Result<bool, Self::Error>;

	/// Returns a stream over the resources of the domain.
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

/// Any [`AsyncConstGenDomain`] can be used as an [`AsyncGenDomain`].
impl<I: AsyncConstGenDomain> AsyncGenDomain for I {
	async fn async_new_resource(&mut self) -> Result<Self::Resource, Self::Error> {
		AsyncConstGenDomain::async_new_resource(self).await
	}
}

/// Any fallible finite domain can be used, synchronously, as an
/// asynchronous finite domain.
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

/// Any fallible const-generative domain can be used, synchronously, as an
/// asynchronous const-generative domain.
impl<I: TryConstGenDomain> AsyncConstGenDomain for I {
	async fn async_new_resource(&self) -> Result<Self::Resource, Self::Error> {
		TryConstGenDomain::try_new_resource(self)
	}
}
