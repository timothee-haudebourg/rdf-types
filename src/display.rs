use std::fmt;

pub trait RdfDisplay {
	fn rdf_fmt(&self, f: &mut fmt::Formatter) -> fmt::Result;

	fn rdf_display(&self) -> RdfDisplayed<&Self> {
		RdfDisplayed(self)
	}
}

impl<'a, T: RdfDisplay + ?Sized> RdfDisplay for &'a T {
	fn rdf_fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		T::rdf_fmt(*self, f)
	}
}

pub struct RdfDisplayed<T>(T);

impl<T: RdfDisplay> fmt::Display for RdfDisplayed<T> {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		self.0.rdf_fmt(f)
	}
}

#[cfg(feature = "contextual")]
pub trait RdfDisplayWithContext<C: ?Sized> {
	fn rdf_fmt_with(&self, context: &C, f: &mut fmt::Formatter) -> fmt::Result;
}

#[cfg(feature = "contextual")]
impl<'a, T: RdfDisplayWithContext<C> + ?Sized, C: ?Sized> RdfDisplayWithContext<C> for &'a T {
	fn rdf_fmt_with(&self, context: &C, f: &mut fmt::Formatter) -> fmt::Result {
		T::rdf_fmt_with(*self, context, f)
	}
}

#[cfg(feature = "contextual")]
impl<'c, T: RdfDisplayWithContext<C>, C: ?Sized> RdfDisplay for contextual::Contextual<T, &'c C> {
	fn rdf_fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		self.0.rdf_fmt_with(self.1, f)
	}
}

#[cfg(feature = "contextual")]
impl<'c, T: RdfDisplayWithContext<C>, C: ?Sized> RdfDisplay
	for contextual::Contextual<T, &'c mut C>
{
	fn rdf_fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		self.0.rdf_fmt_with(self.1, f)
	}
}
