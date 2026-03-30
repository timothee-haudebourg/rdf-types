use maybe_owned::MaybeOwned;
use std::future::Future;

use futures_lite::{stream, Stream};

use crate::{
	dataset::{
		fallible::{TryDatasetMut, TryFiniteDataset, TryPatternMatchingDataset},
		TryDataset,
	},
	pattern::CanonicalQuadPattern,
	Quad, Triple,
};

/// Async fallible finite dataset.
pub trait AsyncTryFiniteDataset: TryDataset {
	type AsyncTryQuads<'a>: Stream<Item = Result<Quad<MaybeOwned<'a, Self::Resource>>, Self::Error>>
	where
		Self: 'a;

	fn async_try_quads(&self) -> Self::AsyncTryQuads<'_>;
}

impl<D: TryFiniteDataset> AsyncTryFiniteDataset for D {
	type AsyncTryQuads<'a>
		= stream::Iter<D::TryQuads<'a>>
	where
		Self: 'a;

	fn async_try_quads(&self) -> Self::AsyncTryQuads<'_> {
		stream::iter(self.try_quads())
	}
}

/// Async fallible pattern-matching-capable dataset.
pub trait AsyncTryPatternMatchingDataset: TryDataset {
	type AsyncTryQuadPatternMatching<'a, 'p>: Stream<
		Item = Result<Quad<MaybeOwned<'a, Self::Resource>>, Self::Error>,
	>
	where
		Self: 'a,
		Self::Resource: 'p;

	fn async_try_quad_pattern_matching<'p>(
		&self,
		pattern: CanonicalQuadPattern<MaybeOwned<'p, Self::Resource>>,
	) -> Self::AsyncTryQuadPatternMatching<'_, 'p>;

	fn async_try_contains_triple(
		&self,
		triple: Triple<&Self::Resource>,
	) -> impl Future<Output = Result<bool, Self::Error>> {
		async move {
			use futures_lite::StreamExt;
			let mut stream = std::pin::pin!(
				self.async_try_quad_pattern_matching(triple.map(MaybeOwned::Borrowed).into())
			);
			Ok(stream.next().await.transpose()?.is_some())
		}
	}
}

impl<D: TryPatternMatchingDataset> AsyncTryPatternMatchingDataset for D {
	type AsyncTryQuadPatternMatching<'a, 'p>
		= stream::Iter<D::TryQuadPatternMatching<'a, 'p>>
	where
		Self: 'a,
		Self::Resource: 'p;

	fn async_try_quad_pattern_matching<'p>(
		&self,
		pattern: CanonicalQuadPattern<MaybeOwned<'p, Self::Resource>>,
	) -> Self::AsyncTryQuadPatternMatching<'_, 'p> {
		stream::iter(self.try_quad_pattern_matching(pattern))
	}
}

/// Async fallible mutable dataset.
pub trait AsyncTryDatasetMut: TryDataset {
	fn async_try_insert(
		&mut self,
		quad: Quad<Self::Resource>,
	) -> impl Future<Output = Result<(), Self::Error>>;
}

impl<D: TryDatasetMut> AsyncTryDatasetMut for D {
	async fn async_try_insert(&mut self, quad: Quad<Self::Resource>) -> Result<(), Self::Error> {
		self.try_insert(quad)
	}
}
