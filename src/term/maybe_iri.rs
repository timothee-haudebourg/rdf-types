use crate::{Id, Term};

/// Types that may represent an iri.
pub trait MaybeIri {
	/// Inner iri type.
	type Iri;
}

impl<I, B> MaybeIri for Id<I, B> {
	type Iri = I;
}

impl<I: MaybeIri, L> MaybeIri for Term<I, L> {
	type Iri = I::Iri;
}

/// Types that may have an iri representation that can be
/// borrowed.
pub trait AsIri: MaybeIri {
	/// Returns a reference to the iri value, if any.
	fn as_iri(&self) -> Option<&Self::Iri>;

	fn is_iri(&self) -> bool {
		self.as_iri().is_some()
	}
}

impl<I, B> AsIri for Id<I, B> {
	fn as_iri(&self) -> Option<&I> {
		self.as_iri()
	}
}

impl<I: AsIri, L> AsIri for Term<I, L> {
	fn as_iri(&self) -> Option<&Self::Iri> {
		self.as_iri()
	}
}

/// Types that can be turned into an iri.
pub trait IntoIri: MaybeIri {
	/// Converts the value into an iri, if any.
	fn into_iri(self) -> Option<Self::Iri>;
}

impl<I, B> IntoIri for Id<I, B> {
	fn into_iri(self) -> Option<I> {
		self.into_iri()
	}
}

impl<I: IntoIri, L> IntoIri for Term<I, L> {
	fn into_iri(self) -> Option<Self::Iri> {
		self.into_iri()
	}
}

/// Types that can be constructed from an iri.
pub trait FromIri: MaybeIri {
	/// Builds a value from an iri.
	fn from_iri(b: Self::Iri) -> Self;
}

impl<I, B> FromIri for Id<I, B> {
	fn from_iri(b: I) -> Self {
		Self::Iri(b)
	}
}

impl<I: FromIri, L> FromIri for Term<I, L> {
	fn from_iri(b: Self::Iri) -> Self {
		Self::Id(I::from_iri(b))
	}
}