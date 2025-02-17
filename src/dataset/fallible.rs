use crate::{pattern::CanonicalQuadPattern, utils::InfallibleIterator, Quad, Triple};

use super::{Dataset, DatasetMut, PatternMatchingDataset, TraversableDataset};

/// Fallible dataset.
pub trait FallibleDataset {
	/// Resource type.
	type Resource;

	/// Error type.
	type Error;
}

impl<D: Dataset> FallibleDataset for D {
	type Resource = D::Resource;
	type Error = std::convert::Infallible;
}

/// Fallible traversable dataset.
pub trait FallibleTraversableDataset: FallibleDataset {
	/// Fallible quads iterator.
	type TryQuads<'a>: Iterator<Item = Result<Quad<&'a Self::Resource>, Self::Error>>
	where
		Self: 'a;

	fn try_quads(&self) -> Self::TryQuads<'_>;
}

impl<D: TraversableDataset> FallibleTraversableDataset for D {
	type TryQuads<'a>
		= InfallibleIterator<D::Quads<'a>>
	where
		Self: 'a;

	fn try_quads(&self) -> Self::TryQuads<'_> {
		InfallibleIterator(self.quads())
	}
}

/// Pattern-matching-capable fallible dataset.
pub trait FalliblePatternMatchingDataset: FallibleDataset {
	type TryQuadPatternMatching<'a, 'p>: Iterator<
		Item = Result<Quad<&'a Self::Resource>, Self::Error>,
	>
	where
		Self: 'a,
		Self::Resource: 'p;

	fn try_quad_pattern_matching<'p>(
		&self,
		pattern: CanonicalQuadPattern<&'p Self::Resource>,
	) -> Self::TryQuadPatternMatching<'_, 'p>;

	fn try_contains_triple(&self, triple: Triple<&Self::Resource>) -> Result<bool, Self::Error> {
		Ok(self
			.try_quad_pattern_matching(triple.into())
			.next()
			.transpose()?
			.is_some())
	}
}

impl<D: PatternMatchingDataset> FalliblePatternMatchingDataset for D {
	type TryQuadPatternMatching<'a, 'p>
		= InfallibleIterator<D::QuadPatternMatching<'a, 'p>>
	where
		Self: 'a,
		Self::Resource: 'p;

	fn try_quad_pattern_matching<'p>(
		&self,
		pattern: CanonicalQuadPattern<&'p Self::Resource>,
	) -> Self::TryQuadPatternMatching<'_, 'p> {
		InfallibleIterator(self.quad_pattern_matching(pattern))
	}
}

/// Fallible mutable dataset.
pub trait FallibleDatasetMut: FallibleDataset {
	fn try_insert(&mut self, quad: Quad<Self::Resource>) -> Result<(), Self::Error>;
}

impl<D: DatasetMut> FallibleDatasetMut for D {
	fn try_insert(&mut self, quad: Quad<Self::Resource>) -> Result<(), Self::Error> {
		self.insert(quad);
		Ok(())
	}
}
