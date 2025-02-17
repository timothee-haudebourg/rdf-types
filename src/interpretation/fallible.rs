use super::{GenerativeInterpretation, Interpretation};
use crate::utils::InfallibleIterator;

pub trait FallibleInterpretation {
	type Resource;
	type Error;
}

impl<I: Interpretation> FallibleInterpretation for I {
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
	type Resources<'a>
		= InfallibleIterator<I::Resources<'a>>
	where
		Self: 'a;

	fn try_resources(&self) -> Self::Resources<'_> {
		InfallibleIterator(self.resources())
	}
}

pub trait FallibleGenerativeInterpretation: FallibleInterpretation {
	fn try_new_resource(&mut self) -> Result<Self::Resource, Self::Error>;
}

impl<I: GenerativeInterpretation> FallibleGenerativeInterpretation for I {
	fn try_new_resource(&mut self) -> Result<Self::Resource, Self::Error> {
		Ok(self.new_resource())
	}
}
