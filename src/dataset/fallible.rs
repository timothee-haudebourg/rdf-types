use crate::{
	Quad,
	pattern::CanonicalQuadPattern,
	util::{InfallibleIterator, QuadsIntoOwned},
};

use super::{Dataset, DatasetMut, FiniteDataset, PatternMatchingDataset};

/// Fallible dataset.
pub trait TryDataset {
	/// Resource type.
	type Resource;

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
	type TryQuads<'a>: Iterator<Item = Result<Quad<Self::Resource>, Self::Error>>
	where
		Self: 'a;

	fn try_quads(&self) -> Result<Self::TryQuads<'_>, Self::Error>;
}

impl<D: FiniteDataset> TryFiniteDataset for D
where
	D::Resource: Clone,
{
	type TryQuads<'a>
		= InfallibleIterator<QuadsIntoOwned<D::Quads<'a>>>
	where
		Self: 'a;

	fn try_quads(&self) -> Result<Self::TryQuads<'_>, Self::Error> {
		Ok(InfallibleIterator(QuadsIntoOwned(self.quads())))
	}
}

/// Pattern-matching-capable fallible dataset.
pub trait TryPatternMatchingDataset: TryDataset {
	type TryQuadPatternMatching<'a, 'p>: Iterator<Item = Result<Quad<Self::Resource>, Self::Error>>
	where
		Self: 'a,
		Self::Resource: 'p;

	fn try_quad_pattern_matching<'p>(
		&self,
		pattern: CanonicalQuadPattern<&'p Self::Resource>,
	) -> Result<Self::TryQuadPatternMatching<'_, 'p>, Self::Error>;

	fn try_contains_quad(&self, quad: Quad<&Self::Resource>) -> Result<bool, Self::Error> {
		Ok(self
			.try_quad_pattern_matching(quad.into())?
			.next()
			.transpose()?
			.is_some())
	}
}

impl<D: PatternMatchingDataset> TryPatternMatchingDataset for D
where
	D::Resource: Clone,
{
	type TryQuadPatternMatching<'a, 'p>
		= InfallibleIterator<QuadsIntoOwned<D::QuadPatternMatching<'a, 'p>>>
	where
		Self: 'a,
		Self::Resource: 'p;

	fn try_quad_pattern_matching<'p>(
		&self,
		pattern: CanonicalQuadPattern<&'p Self::Resource>,
	) -> Result<Self::TryQuadPatternMatching<'_, 'p>, Self::Error> {
		Ok(InfallibleIterator(QuadsIntoOwned(
			self.quad_pattern_matching(pattern),
		)))
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
