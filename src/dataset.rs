use crate::{pattern::CanonicalTriplePattern, utils::InfallibleIterator, Quad};

/// RDF dataset.
pub trait Dataset {
	type Resource;
}

/// Pattern-matching-capable dataset.
pub trait PatternMatchingDataset: Dataset {
	type PatternMatching<'a>: Iterator<Item = Quad<&'a Self::Resource>>
	where
		Self: 'a;

	fn pattern_matching(
		&self,
		pattern: CanonicalTriplePattern<&Self::Resource>,
	) -> Self::PatternMatching<'_>;
}

/// Fallible dataset.
pub trait FallibleDataset {
	type Resource;
	type Error;
}

impl<D: Dataset> FallibleDataset for D {
	type Resource = D::Resource;
	type Error = std::convert::Infallible;
}

/// Pattern-matching-capable fallible dataset.
pub trait FalliblePatternMatchingDataset: FallibleDataset {
	type TryPatternMatching<'a>: Iterator<Item = Result<Quad<&'a Self::Resource>, Self::Error>>
	where
		Self: 'a;

	fn try_pattern_matching(
		&self,
		pattern: CanonicalTriplePattern<&Self::Resource>,
	) -> Self::TryPatternMatching<'_>;
}

impl<D: PatternMatchingDataset> FalliblePatternMatchingDataset for D {
	type TryPatternMatching<'a> = InfallibleIterator<D::PatternMatching<'a>> where Self: 'a;

	fn try_pattern_matching(
		&self,
		pattern: CanonicalTriplePattern<&Self::Resource>,
	) -> Self::TryPatternMatching<'_> {
		InfallibleIterator(self.pattern_matching(pattern))
	}
}
