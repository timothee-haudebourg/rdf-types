use futures_lite::{Stream, StreamExt, stream};

use crate::{
	Quad,
	dataset::fallible::{TryDataset, TryDatasetMut, TryFiniteDataset, TryPatternMatchingDataset},
	pattern::CanonicalQuadPattern,
};

/// Async finite dataset.
pub trait AsyncFiniteDataset: TryDataset {
	type AsyncQuads<'a>: Stream<Item = Result<Quad<Self::Resource>, Self::Error>>
	where
		Self: 'a;

	async fn async_quads(&self) -> Result<Self::AsyncQuads<'_>, Self::Error>;
}

impl<D: TryFiniteDataset> AsyncFiniteDataset for D {
	type AsyncQuads<'a>
		= stream::Iter<D::TryQuads<'a>>
	where
		Self: 'a;

	async fn async_quads(&self) -> Result<Self::AsyncQuads<'_>, Self::Error> {
		self.try_quads().map(stream::iter)
	}
}

/// Async pattern-matching-capable dataset.
pub trait AsyncPatternMatchingDataset: TryDataset {
	type AsyncQuadPatternMatching<'a, 'p>: Stream<Item = Result<Quad<Self::Resource>, Self::Error>>
	where
		Self: 'a,
		Self::Resource: 'p;

	async fn async_quad_pattern_matching<'p>(
		&self,
		pattern: CanonicalQuadPattern<&'p Self::Resource>,
	) -> Result<Self::AsyncQuadPatternMatching<'_, 'p>, Self::Error>;

	async fn async_contains_quad(&self, quad: Quad<&Self::Resource>) -> Result<bool, Self::Error> {
		let mut stream = std::pin::pin!(self.async_quad_pattern_matching(quad.into()).await?);
		Ok(stream.next().await.transpose()?.is_some())
	}
}

impl<D: TryPatternMatchingDataset> AsyncPatternMatchingDataset for D {
	type AsyncQuadPatternMatching<'a, 'p>
		= stream::Iter<D::TryQuadPatternMatching<'a, 'p>>
	where
		Self: 'a,
		Self::Resource: 'p;

	async fn async_quad_pattern_matching<'p>(
		&self,
		pattern: CanonicalQuadPattern<&'p Self::Resource>,
	) -> Result<Self::AsyncQuadPatternMatching<'_, 'p>, Self::Error> {
		self.try_quad_pattern_matching(pattern).map(stream::iter)
	}
}

/// Async mutable dataset.
pub trait AsyncDatasetMut: TryDataset {
	async fn async_insert(&mut self, quad: Quad<Self::Resource>) -> Result<(), Self::Error>;
}

impl<D: TryDatasetMut> AsyncDatasetMut for D {
	async fn async_insert(&mut self, quad: Quad<Self::Resource>) -> Result<(), Self::Error> {
		self.try_insert(quad)
	}
}
