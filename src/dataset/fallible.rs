//! Fallible counterparts of the [`Dataset`] traits, for datasets backed by
//! fallible storage (e.g. a database or a file).
use crate::{
	Quad,
	pattern::LinearQuadPattern,
	util::{InfallibleIterator, QuadsIntoOwned},
};

use super::{Dataset, DatasetMut, FiniteDataset, PatternMatchingDataset};

/// Fallible dataset.
///
/// A [`Dataset`] whose operations may fail with an associated
/// [`Self::Error`] type.
///
/// Every non-fallible [`Dataset`] is also a `TryDataset`, with the
/// [`Infallible`](std::convert::Infallible) error type.
pub trait TryDataset {
	/// Resource type.
	type Resource;

	/// Error type.
	type Error;
}

/// Any non-fallible dataset can be used as fallible, with the
/// [`Infallible`](std::convert::Infallible) error type.
impl<D: Dataset> TryDataset for D {
	type Resource = D::Resource;
	type Error = std::convert::Infallible;
}

/// Fallible dataset that can be traversed using a provided quad iterator.
pub trait TryFiniteDataset: TryDataset {
	/// Fallible quads iterator.
	type TryQuads<'a>: Iterator<Item = Result<Quad<Self::Resource>, Self::Error>>
	where
		Self: 'a;

	/// Returns a fallible iterator over the quads of the dataset.
	fn try_quads(&self) -> Result<Self::TryQuads<'_>, Self::Error>;
}

/// Any non-fallible finite dataset can be used as fallible, with the
/// [`Infallible`](std::convert::Infallible) error type.
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
	/// Fallible pattern-matching iterator.
	type TryQuadPatternMatching<'a, 'p>: Iterator<Item = Result<Quad<Self::Resource>, Self::Error>>
	where
		Self: 'a,
		Self::Resource: 'p;

	/// Returns a fallible iterator over all the quads of the dataset
	/// matching the given pattern.
	fn try_quad_pattern_matching<'p>(
		&self,
		pattern: LinearQuadPattern<&'p Self::Resource>,
	) -> Result<Self::TryQuadPatternMatching<'_, 'p>, Self::Error>;

	/// Checks if the dataset contains the given quad.
	fn try_contains_quad(&self, quad: Quad<&Self::Resource>) -> Result<bool, Self::Error> {
		Ok(self
			.try_quad_pattern_matching(quad.into())?
			.next()
			.transpose()?
			.is_some())
	}
}

/// Any non-fallible pattern-matching-capable dataset can be used as
/// fallible, with the [`Infallible`](std::convert::Infallible) error type.
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
		pattern: LinearQuadPattern<&'p Self::Resource>,
	) -> Result<Self::TryQuadPatternMatching<'_, 'p>, Self::Error> {
		Ok(InfallibleIterator(QuadsIntoOwned(
			self.quad_pattern_matching(pattern),
		)))
	}
}

/// Fallible mutable dataset.
pub trait TryDatasetMut: TryDataset {
	/// Tries to insert the given quad in the dataset.
	fn try_insert(&mut self, quad: Quad<Self::Resource>) -> Result<(), Self::Error>;
}

/// Any non-fallible mutable dataset can be used as fallible, with the
/// [`Infallible`](std::convert::Infallible) error type.
impl<D: DatasetMut> TryDatasetMut for D {
	fn try_insert(&mut self, quad: Quad<Self::Resource>) -> Result<(), Self::Error> {
		self.insert(quad);
		Ok(())
	}
}
