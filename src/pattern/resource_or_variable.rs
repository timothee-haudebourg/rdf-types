/// Resource or variable.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ResourceOrVar<T, X> {
	Resource(T),
	Var(X),
}

impl<T, X> ResourceOrVar<T, X> {
	pub fn map<U>(self, f: impl Fn(T) -> U) -> ResourceOrVar<U, X> {
		match self {
			Self::Resource(t) => ResourceOrVar::Resource(f(t)),
			Self::Var(x) => ResourceOrVar::Var(x),
		}
	}

	pub fn as_ref(&self) -> ResourceOrVar<&T, &X> {
		match self {
			Self::Resource(t) => ResourceOrVar::Resource(t),
			Self::Var(x) => ResourceOrVar::Var(x),
		}
	}

	pub fn is_id_or(&self, f: impl FnOnce(&X) -> bool) -> bool {
		match self {
			Self::Resource(_) => true,
			Self::Var(x) => f(x),
		}
	}
}

impl<T, X> From<T> for ResourceOrVar<T, X> {
	fn from(value: T) -> Self {
		Self::Resource(value)
	}
}
