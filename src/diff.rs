//! Diff between two RDF datasets.
use core::fmt;
use std::cmp::Ordering;

use crate::Quad;

/// Diff between two RDF datasets.
///
/// Computed by comparing the sorted list of quads of each dataset, see
/// [`RdfDiff::new`].
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct RdfDiff<R> {
	/// Quads present in the second dataset but not in the first.
	pub added: Vec<Quad<R>>,

	/// Quads present in the first dataset but not in the second.
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
	/// Computes the diff between the quads of `a` and the quads of `b`.
	///
	/// Duplicate quads (whether within `a`, within `b`, or across both) are
	/// preserved: for instance, if a quad occurs twice in `a` and once in
	/// `b`, it will appear once in [`Self::removed`]. Use [`Self::new_dedup`]
	/// to ignore duplicates instead.
	pub fn new(
		a: impl IntoIterator<Item = Quad<R>>,
		b: impl IntoIterator<Item = Quad<R>>,
	) -> RdfDiff<R>
	where
		R: Ord,
	{
		Self::new_with(a, b, false)
	}

	/// Computes the diff between the quads of `a` and the quads of `b`,
	/// ignoring duplicate quads in either dataset.
	pub fn new_dedup(
		a: impl IntoIterator<Item = Quad<R>>,
		b: impl IntoIterator<Item = Quad<R>>,
	) -> RdfDiff<R>
	where
		R: Ord,
	{
		Self::new_with(a, b, true)
	}

	/// Computes the diff between the quads of `a` and the quads of `b`,
	/// optionally ignoring duplicate quads (`dedup`) in either dataset.
	///
	/// See [`Self::new`] and [`Self::new_dedup`].
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

	/// Checks if there is no difference, i.e. both datasets have the same
	/// quads.
	pub fn is_empty(&self) -> bool {
		self.added.is_empty() && self.removed.is_empty()
	}

	/// Returns a wrapper implementing [`fmt::Display`] that prints this diff
	/// using ANSI colors (green for added quads, red for removed quads).
	pub fn colored(&self) -> ColoredRdfDiff<'_, R> {
		ColoredRdfDiff(self)
	}
}

impl<R: fmt::Display> fmt::Display for RdfDiff<R> {
	/// Prints one line per quad, prefixed with `+` for added quads and `-`
	/// for removed quads.
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

/// Wrapper around a [`RdfDiff`] reference that prints it using ANSI colors,
/// created with [`RdfDiff::colored`].
pub struct ColoredRdfDiff<'a, R>(pub &'a RdfDiff<R>);

impl<'a, R: fmt::Display> fmt::Display for ColoredRdfDiff<'a, R> {
	/// Prints one line per quad, in green and prefixed with `+` for added
	/// quads, in red and prefixed with `-` for removed quads.
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
