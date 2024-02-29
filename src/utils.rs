pub struct InfallibleIterator<I>(pub I);

impl<I: Iterator> Iterator for InfallibleIterator<I> {
	type Item = Result<I::Item, std::convert::Infallible>;

	fn next(&mut self) -> Option<Self::Item> {
		self.0.next().map(Ok)
	}
}
