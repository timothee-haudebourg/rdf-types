use crate::{utils::InfallibleIterator, InterpretationMut};

pub trait FallibleInterpretation {
	type Resource;
	type Error;
}

impl<I: super::Interpretation> FallibleInterpretation for I {
	type Resource = I::Resource;
	type Error = std::convert::Infallible;
}

pub trait TraversableFallibleInterpretation: FallibleInterpretation {
	type Resources<'a>: Iterator<Item = Result<&'a Self::Resource, Self::Error>>
	where
		Self: 'a;

	fn try_resources(&self) -> Self::Resources<'_>;
}

impl<I: super::TraversableInterpretation> TraversableFallibleInterpretation for I {
	type Resources<'a> = InfallibleIterator<I::Resources<'a>> where Self: 'a;

	fn try_resources(&self) -> Self::Resources<'_> {
		InfallibleIterator(self.resources())
	}
}

pub trait FallibleInterpretationMut<V>: FallibleInterpretation {
	fn try_new_resource(&mut self, vocabulary: &mut V) -> Result<Self::Resource, Self::Error>;
}

impl<V, I: InterpretationMut<V>> FallibleInterpretationMut<V> for I {
	fn try_new_resource(&mut self, vocabulary: &mut V) -> Result<Self::Resource, Self::Error> {
		Ok(self.new_resource(vocabulary))
	}
}
