use crate::{Id, Term};

/// Types that may represent a blank node identifier.
pub trait MaybeBlankId {
	/// Inner blank node identifier type.
	type BlankId;
}

impl<I, B> MaybeBlankId for Id<I, B> {
	type BlankId = B;
}

impl<I: MaybeBlankId, L> MaybeBlankId for Term<I, L> {
	type BlankId = I::BlankId;
}

/// Types that may have a blank node identifier representation that can be
/// borrowed.
pub trait TryAsBlankId: MaybeBlankId {
	/// Returns a reference to the blank node identifier value, if any.
	fn try_as_blank(&self) -> Option<&Self::BlankId>;

	fn is_blank(&self) -> bool {
		self.try_as_blank().is_some()
	}
}

impl<I, B> TryAsBlankId for Id<I, B> {
	fn try_as_blank(&self) -> Option<&Self::BlankId> {
		self.as_blank()
	}
}

impl<I: TryAsBlankId, L> TryAsBlankId for Term<I, L> {
	fn try_as_blank(&self) -> Option<&Self::BlankId> {
		self.as_blank()
	}
}

/// Types that can be turned into a blank node identifier.
pub trait TryIntoBlankId: MaybeBlankId + Sized {
	fn try_into_blank(self) -> Result<Self::BlankId, Self>;
}

impl<I, B> TryIntoBlankId for Id<I, B> {
	fn try_into_blank(self) -> Result<Self::BlankId, Self> {
		self.try_into_blank().map_err(Self::Iri)
	}
}

impl<I: TryIntoBlankId, L> TryIntoBlankId for Term<I, L> {
	fn try_into_blank(self) -> Result<Self::BlankId, Self> {
		self.try_into_blank()
	}
}

/// Types that can be constructed from a blank node identifier.
pub trait FromBlankId: MaybeBlankId {
	/// Builds a value from a blank node identifier.
	fn from_blank(b: Self::BlankId) -> Self;
}

impl<I, B> FromBlankId for Id<I, B> {
	fn from_blank(b: Self::BlankId) -> Self {
		Self::Blank(b)
	}
}

impl<I: FromBlankId, L> FromBlankId for Term<I, L> {
	fn from_blank(b: Self::BlankId) -> Self {
		Self::Id(I::from_blank(b))
	}
}
