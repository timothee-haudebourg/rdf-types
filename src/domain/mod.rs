//! RDF resource domains: abstractions over a set of resources,
//! independently of any graph or dataset.
pub use std::borrow::Cow;

mod r#async;
mod fallible;
mod r#static;

pub use r#async::*;
pub use fallible::*;
pub use r#static::*;

/// RDF resource domain.
///
/// A domain simply fixes the type used to represent resources. It is the
/// most basic building block shared by every trait of this crate (e.g.
/// [`crate::Graph`] and [`crate::Dataset`]), and does not, by itself,
/// require the domain to be finite, mutable, or otherwise usable.
pub trait Domain {
	/// Resource type.
	type Resource;
}

/// Finite domain.
///
/// A domain whose resources can be counted, checked for membership, and
/// iterated over.
pub trait FiniteDomain: Domain {
	/// Resources iterator.
	type Resources<'a>: Iterator<Item = &'a Self::Resource>
	where
		Self: 'a;

	/// Returns the number of resources in the domain.
	fn len(&self) -> usize;

	/// Checks if the domain has no resources.
	fn is_empty(&self) -> bool {
		self.len() == 0
	}

	/// Checks if the given resource belongs to the domain.
	fn contains(&self, a: &Self::Resource) -> bool;

	/// Returns an iterator over the resources of the domain.
	fn resources(&self) -> Self::Resources<'_>;
}

/// Domain that can spawn fresh new resources from a shared reference.
pub trait ConstGenDomain: Domain {
	/// Create a new resource.
	fn new_resource(&self) -> Self::Resource;
}

/// Domain that can spawn fresh new resources.
pub trait GenDomain: Domain {
	/// Create a new resource.
	fn new_resource(&mut self) -> Self::Resource;
}

impl<T: ConstGenDomain> GenDomain for T {
	fn new_resource(&mut self) -> Self::Resource {
		ConstGenDomain::new_resource(self)
	}
}
