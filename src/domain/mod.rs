pub mod r#async;
pub mod fallible;
pub mod sealed;

pub use std::borrow::Cow;

/// RDF resource domain.
pub trait Domain {
	type Resource: ToOwned;
}

pub trait EqDomain: Domain {
	fn is_eq(&self, a: &Self::Resource, b: &Self::Resource) -> bool;
}

pub trait VariableDomain: Domain {
	fn is_ground(&self, a: &Self::Resource) -> bool;

	fn is_variable(&self, a: &Self::Resource) -> bool {
		!self.is_ground(a)
	}
}

/// Finite domain.
pub trait FiniteDomain: Domain {
	type Resources<'a>: Iterator<Item = Cow<'a, Self::Resource>>
	where
		Self: 'a;

	fn len(&self) -> usize;

	fn is_empty(&self) -> bool {
		self.len() == 0
	}

	fn contains(&self, a: &Self::Resource) -> bool;

	fn resources(&self) -> Self::Resources<'_>;
}

/// Domain that can spawn fresh new resources.
pub trait GenDomain: Domain {
	/// Create a new resource.
	fn new_resource(&mut self) -> Self::Resource;
}

/// Interpretation that can spawn fresh new resources.
pub trait ConstGenDomain: Domain {
	/// Create a new resource.
	fn new_resource(&self) -> Self::Resource;
}

pub trait Interpretation<T>: Domain {
	fn interpret(&mut self, term: T) -> Self::Resource;
}
