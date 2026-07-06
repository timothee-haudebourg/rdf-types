use core::fmt;
use std::cmp::Ordering;

use crate::Quad;

/// Diff between two RDF datasets.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct RdfDiff<R> {
	pub added: Vec<Quad<R>>,
	pub removed: Vec<Quad<R>>,
}

impl<R> Default for RdfDiff<R> {
	fn default() -> Self {
		Self {
			added: Vec::new(),
			removed: Vec::new(),
		}
	}
}

impl<R> RdfDiff<R> {
	pub fn new(
		a: impl IntoIterator<Item = Quad<R>>,
		b: impl IntoIterator<Item = Quad<R>>,
	) -> RdfDiff<R>
	where
		R: Ord,
	{
		Self::new_with(a, b, false)
	}

	pub fn new_dedup(
		a: impl IntoIterator<Item = Quad<R>>,
		b: impl IntoIterator<Item = Quad<R>>,
	) -> RdfDiff<R>
	where
		R: Ord,
	{
		Self::new_with(a, b, true)
	}

	pub fn new_with(
		a: impl IntoIterator<Item = Quad<R>>,
		b: impl IntoIterator<Item = Quad<R>>,
		dedup: bool,
	) -> RdfDiff<R>
	where
		R: Ord,
	{
		let mut a: Vec<_> = a.into_iter().collect();
		let mut b: Vec<_> = b.into_iter().collect();

		a.sort_unstable();
		b.sort_unstable();

		if dedup {
			a.dedup();
			b.dedup();
		}

		let mut result = Self::default();

		let mut a = a.into_iter().peekable();
		let mut b = b.into_iter().peekable();

		loop {
			match (a.peek(), b.peek()) {
				(Some(a_quad), Some(b_quad)) => match a_quad.cmp(b_quad) {
					Ordering::Equal => {
						a.next();
						b.next();
					}
					Ordering::Less => result.added.push(a.next().unwrap()),
					Ordering::Greater => result.removed.push(b.next().unwrap()),
				},
				(Some(_), None) => {
					result.added.push(a.next().unwrap());
				}
				(None, Some(_)) => {
					result.removed.push(b.next().unwrap());
				}
				(None, None) => break,
			}
		}

		result
	}

	pub fn is_empty(&self) -> bool {
		self.added.is_empty() && self.removed.is_empty()
	}

	pub fn colored(&self) -> ColoredRdfDiff<'_, R> {
		ColoredRdfDiff(self)
	}
}

impl<R: fmt::Display> fmt::Display for RdfDiff<R> {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		for quad in &self.added {
			writeln!(f, "+ {quad}")?;
		}

		for quad in &self.removed {
			writeln!(f, "- {quad}")?;
		}

		Ok(())
	}
}

pub struct ColoredRdfDiff<'a, R>(pub &'a RdfDiff<R>);

impl<'a, R: fmt::Display> fmt::Display for ColoredRdfDiff<'a, R> {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		for quad in &self.0.added {
			writeln!(f, "\x1b[32m+ {quad}\x1b[0m")?;
		}

		for quad in &self.0.removed {
			writeln!(f, "\x1b[31m- {quad}\x1b[0m")?;
		}

		Ok(())
	}
}
