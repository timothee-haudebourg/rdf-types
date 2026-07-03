use std::convert::Infallible;

use super::{ConstGenDomain, Cow, Domain, FiniteDomain};

pub trait TryDomain: Domain {
	type Error;
}

/// Finite domain.
pub trait TryFiniteDomain: TryDomain {
	type TryResources<'a>: Iterator<Item = Result<Cow<'a, Self::Resource>, Self::Error>>
	where
		Self: 'a;

	fn try_len(&self) -> Result<usize, Self::Error>;

	fn try_is_empty(&self) -> Result<bool, Self::Error> {
		Ok(self.try_len()? == 0)
	}

	fn try_contains(&self, a: &Self::Resource) -> Result<bool, Self::Error>;

	fn try_resources(&self) -> Self::TryResources<'_>;
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

impl<I: FiniteDomain> TryFiniteDomain for I {
	type TryResources<'a>
		= std::iter::Map<
		I::Resources<'a>,
		fn(Cow<'a, Self::Resource>) -> Result<Cow<'a, Self::Resource>, Infallible>,
	>
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

	fn try_resources(&self) -> Self::TryResources<'_> {
		self.resources().map(Ok as fn(_) -> _)
	}
}

impl<I: ConstGenDomain> TryConstGenDomain for I {
	fn try_new_resource(&self) -> Result<Self::Resource, Self::Error> {
		Ok(ConstGenDomain::new_resource(self))
	}
}
