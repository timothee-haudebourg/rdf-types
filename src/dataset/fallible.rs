use crate::{pattern::CanonicalQuadPattern, utils::InfallibleIterator, Quad, Triple};
use std::borrow::Cow;

use super::{Dataset, DatasetMut, FiniteDataset, PatternMatchingDataset};

/// Fallible dataset.
pub trait TryDataset {
	/// Resource type.
	type Resource: ToOwned;

	/// Error type.
	type Error;
}

impl<D: Dataset> TryDataset for D {
	type Resource = D::Resource;
	type Error = std::convert::Infallible;
}

/// Fallible traversable dataset.
pub trait TryFiniteDataset: TryDataset {
	/// Fallible quads iterator.
	type TryQuads<'a>: Iterator<Item = Result<Quad<Cow<'a, Self::Resource>>, Self::Error>>
	where
		Self: 'a;

	fn try_quads(&self) -> Self::TryQuads<'_>;
}

impl<D: FiniteDataset> TryFiniteDataset for D {
	type TryQuads<'a>
		= InfallibleIterator<D::Quads<'a>>
	where
		Self: 'a;

	fn try_quads(&self) -> Self::TryQuads<'_> {
		InfallibleIterator(self.quads())
	}
}

/// Pattern-matching-capable fallible dataset.
pub trait TryPatternMatchingDataset: TryDataset {
	type TryQuadPatternMatching<'a, 'p>: Iterator<
		Item = Result<Quad<Cow<'a, Self::Resource>>, Self::Error>,
	>
	where
		Self: 'a,
		Self::Resource: 'p;

	fn try_quad_pattern_matching<'p>(
		&self,
		pattern: CanonicalQuadPattern<Cow<'p, Self::Resource>>,
	) -> Self::TryQuadPatternMatching<'_, 'p>;

	fn try_contains_triple(&self, triple: Triple<&Self::Resource>) -> Result<bool, Self::Error> {
		Ok(self
			.try_quad_pattern_matching(triple.map(Cow::Borrowed).into())
			.next()
			.transpose()?
			.is_some())
	}
}

impl<D: PatternMatchingDataset> TryPatternMatchingDataset for D {
	type TryQuadPatternMatching<'a, 'p>
		= InfallibleIterator<D::QuadPatternMatching<'a, 'p>>
	where
		Self: 'a,
		Self::Resource: 'p;

	fn try_quad_pattern_matching<'p>(
		&self,
		pattern: CanonicalQuadPattern<Cow<'p, Self::Resource>>,
	) -> Self::TryQuadPatternMatching<'_, 'p> {
		InfallibleIterator(self.quad_pattern_matching(pattern))
	}
}

/// Fallible mutable dataset.
pub trait TryDatasetMut: TryDataset {
	fn try_insert(&mut self, quad: Quad<Self::Resource>) -> Result<(), Self::Error>;
}

impl<D: DatasetMut> TryDatasetMut for D {
	fn try_insert(&mut self, quad: Quad<Self::Resource>) -> Result<(), Self::Error> {
		self.insert(quad);
		Ok(())
	}
}
