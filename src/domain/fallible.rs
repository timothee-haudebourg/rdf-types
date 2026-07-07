//! Fallible counterparts of the [`Domain`] traits, for domains backed by
//! fallible storage (e.g. a database or a file).
use std::{convert::Infallible, iter::Cloned};

use crate::util::InfallibleIterator;

use super::{ConstGenDomain, Domain, FiniteDomain};

/// Fallible domain.
///
/// A [`Domain`] whose operations may fail with an associated [`Self::Error`]
/// type, e.g. because it is backed by fallible storage such as a database
/// or a file.
///
/// Every non-fallible [`Domain`] is also a `TryDomain`, with the
/// [`Infallible`] error type.
pub trait TryDomain: Domain {
	/// Error type.
	type Error;
}

/// Fallible finite domain.
pub trait TryFiniteDomain: TryDomain {
	/// Fallible resources iterator.
	type TryResources<'a>: Iterator<Item = Result<Self::Resource, Self::Error>>
	where
		Self: 'a;

	/// Returns the number of resources in the domain.
	fn try_len(&self) -> Result<usize, Self::Error>;

	/// Checks if the domain has no resources.
	fn try_is_empty(&self) -> Result<bool, Self::Error> {
		Ok(self.try_len()? == 0)
	}

	/// Checks if the given resource belongs to the domain.
	fn try_contains(&self, a: &Self::Resource) -> Result<bool, Self::Error>;

	/// Returns a fallible iterator over the resources of the domain.
	fn try_resources(&self) -> Result<Self::TryResources<'_>, Self::Error>;
}

/// Fallible domain that can spawn fresh new resources.
pub trait TryGenDomain: TryDomain {
	/// Tries to create a new resource.
	fn try_new_resource(&mut self) -> Result<Self::Resource, Self::Error>;
}

/// Fallible domain that can spawn fresh new resources from a shared
/// reference.
pub trait TryConstGenDomain: TryDomain {
	/// Tries to create a new resource.
	fn try_new_resource(&self) -> Result<Self::Resource, Self::Error>;
}

/// Any [`TryConstGenDomain`] can be used as a [`TryGenDomain`].
impl<I: TryConstGenDomain> TryGenDomain for I {
	fn try_new_resource(&mut self) -> Result<Self::Resource, Self::Error> {
		TryConstGenDomain::try_new_resource(self)
	}
}

/// Any non-fallible domain can be used as fallible, with the [`Infallible`]
/// error type.
impl<I: Domain> TryDomain for I {
	type Error = Infallible;
}

/// Any non-fallible finite domain can be used as fallible, with the
/// [`Infallible`] error type.
impl<I: FiniteDomain> TryFiniteDomain for I
where
	I::Resource: Clone,
{
	type TryResources<'a>
		= InfallibleIterator<Cloned<I::Resources<'a>>>
	where
		Self: 'a;

	fn try_len(&self) -> Result<usize, Self::Error> {
		Ok(self.len())
	}

	fn try_is_empty(&self) -> Result<bool, Self::Error> {
		Ok(self.is_empty())
	}

	fn try_contains(&self, a: &Self::Resource) -> Result<bool, Self::Error> {
		Ok(self.contains(a))
	}

	fn try_resources(&self) -> Result<Self::TryResources<'_>, Self::Error> {
		Ok(InfallibleIterator(self.resources().cloned()))
	}
}

/// Any non-fallible const-generative domain can be used as fallible, with
/// the [`Infallible`] error type.
impl<I: ConstGenDomain> TryConstGenDomain for I {
	fn try_new_resource(&self) -> Result<Self::Resource, Self::Error> {
		Ok(ConstGenDomain::new_resource(self))
	}
}
