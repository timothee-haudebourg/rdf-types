use std::{convert::Infallible, iter::Cloned};

use crate::util::InfallibleIterator;

use super::{ConstGenDomain, Domain, FiniteDomain};

pub trait TryDomain: Domain {
	type Error;
}

/// Finite domain.
pub trait TryFiniteDomain: TryDomain {
	type TryResources<'a>: Iterator<Item = Result<Self::Resource, Self::Error>>
	where
		Self: 'a;

	fn try_len(&self) -> Result<usize, Self::Error>;

	fn try_is_empty(&self) -> Result<bool, Self::Error> {
		Ok(self.try_len()? == 0)
	}

	fn try_contains(&self, a: &Self::Resource) -> Result<bool, Self::Error>;

	fn try_resources(&self) -> Result<Self::TryResources<'_>, Self::Error>;
}

/// Domain that can spawn fresh new resources.
pub trait TryGenDomain: TryDomain {
	/// Create a new resource.
	fn try_new_resource(&mut self) -> Result<Self::Resource, Self::Error>;
}

/// Interpretation that can spawn fresh new resources.
pub trait TryConstGenDomain: TryDomain {
	/// Create a new resource.
	fn try_new_resource(&self) -> Result<Self::Resource, Self::Error>;
}

impl<I: TryConstGenDomain> TryGenDomain for I {
	fn try_new_resource(&mut self) -> Result<Self::Resource, Self::Error> {
		TryConstGenDomain::try_new_resource(self)
	}
}

impl<I: Domain> TryDomain for I {
	type Error = Infallible;
}

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

impl<I: ConstGenDomain> TryConstGenDomain for I {
	fn try_new_resource(&self) -> Result<Self::Resource, Self::Error> {
		Ok(ConstGenDomain::new_resource(self))
	}
}
