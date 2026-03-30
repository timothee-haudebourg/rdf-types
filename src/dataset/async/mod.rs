use maybe_owned::MaybeOwned;
use std::future::Future;

use futures_lite::{stream, Stream};

use super::{DatasetMut, FiniteDataset, PatternMatchingDataset};
use crate::{
	dataset::fallible::TryDataset, pattern::CanonicalQuadPattern, utils::InfallibleIterator, Quad,
	Triple,
};

pub mod fallible;

/// Async finite dataset.
pub trait AsyncFiniteDataset: TryDataset {
	type AsyncQuads<'a>: Stream<Item = Result<Quad<MaybeOwned<'a, Self::Resource>>, Self::Error>>
	where
		Self: 'a;

	fn async_quads(&self) -> Self::AsyncQuads<'_>;
}

impl<D: FiniteDataset> AsyncFiniteDataset for D {
	type AsyncQuads<'a>
		= stream::Iter<InfallibleIterator<D::Quads<'a>>>
	where
		Self: 'a;

	fn async_quads(&self) -> Self::AsyncQuads<'_> {
		stream::iter(InfallibleIterator(self.quads()))
	}
}

/// Async pattern-matching-capable dataset.
pub trait AsyncPatternMatchingDataset: TryDataset {
	type AsyncQuadPatternMatching<'a, 'p>: Stream<
		Item = Result<Quad<MaybeOwned<'a, Self::Resource>>, Self::Error>,
	>
	where
		Self: 'a,
		Self::Resource: 'p;

	fn async_quad_pattern_matching<'p>(
		&self,
		pattern: CanonicalQuadPattern<MaybeOwned<'p, Self::Resource>>,
	) -> Self::AsyncQuadPatternMatching<'_, 'p>;

	fn async_contains_triple(
		&self,
		triple: Triple<&Self::Resource>,
	) -> impl Future<Output = Result<bool, Self::Error>> {
		async move {
			use futures_lite::StreamExt;
			let mut stream = std::pin::pin!(
				self.async_quad_pattern_matching(triple.map(MaybeOwned::Borrowed).into())
			);
			Ok(stream.next().await.transpose()?.is_some())
		}
	}
}

impl<D: PatternMatchingDataset> AsyncPatternMatchingDataset for D {
	type AsyncQuadPatternMatching<'a, 'p>
		= stream::Iter<InfallibleIterator<D::QuadPatternMatching<'a, 'p>>>
	where
		Self: 'a,
		Self::Resource: 'p;

	fn async_quad_pattern_matching<'p>(
		&self,
		pattern: CanonicalQuadPattern<MaybeOwned<'p, Self::Resource>>,
	) -> Self::AsyncQuadPatternMatching<'_, 'p> {
		stream::iter(InfallibleIterator(self.quad_pattern_matching(pattern)))
	}
}

/// Async mutable dataset.
pub trait AsyncDatasetMut: TryDataset {
	fn async_insert(
		&mut self,
		quad: Quad<Self::Resource>,
	) -> impl Future<Output = Result<(), Self::Error>>;
}

impl<D: DatasetMut> AsyncDatasetMut for D {
	async fn async_insert(&mut self, quad: Quad<Self::Resource>) -> Result<(), Self::Error> {
		self.insert(quad);
		Ok(())
	}
}
