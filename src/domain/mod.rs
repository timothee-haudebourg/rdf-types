pub use std::borrow::Cow;

mod r#async;
mod fallible;
mod r#static;

pub use r#async::*;
pub use fallible::*;
pub use r#static::*;

/// RDF resource domain.
pub trait Domain {
	type Resource;
}

/// Finite domain.
pub trait FiniteDomain: Domain {
	type Resources<'a>: Iterator<Item = &'a Self::Resource>
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
