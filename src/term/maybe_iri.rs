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
pub trait TryAsIri: MaybeIri {
	/// Returns a reference to the iri value, if any.
	fn try_as_iri(&self) -> Option<&Self::Iri>;

	fn is_iri(&self) -> bool {
		self.try_as_iri().is_some()
	}
}

impl<I, B> TryAsIri for Id<I, B> {
	fn try_as_iri(&self) -> Option<&I> {
		self.as_iri()
	}
}

impl<I: TryAsIri, L> TryAsIri for Term<I, L> {
	fn try_as_iri(&self) -> Option<&Self::Iri> {
		self.as_iri()
	}
}

/// Types that can be turned into an iri.
pub trait TryIntoIri: MaybeIri + Sized {
	fn try_into_iri(self) -> Result<Self::Iri, Self>;
}

impl<I, B> TryIntoIri for Id<I, B> {
	fn try_into_iri(self) -> Result<I, Self> {
		self.try_into_iri().map_err(Self::Blank)
	}
}

impl<I: TryIntoIri, L> TryIntoIri for Term<I, L> {
	fn try_into_iri(self) -> Result<Self::Iri, Self> {
		self.try_into_iri()
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
