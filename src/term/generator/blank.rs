use crate::{BlankIdBuf, LocalTerm};

use super::LocalGenerator;

/// Generates numbered blank node identifiers,
/// with an optional prefix.
///
/// This generator can create `usize::MAX` unique blank node identifiers.
/// If [`Generator::next`] is called `usize::MAX + 1` times, it will panic.
#[derive(Default)]
pub struct Blank {
	/// Prefix string.
	prefix: String,

	/// Number of already generated identifiers.
	count: usize,
}

impl Blank {
	/// Creates a new numbered generator with no prefix.
	pub fn new() -> Self {
		Self::new_full(String::new(), 0)
	}

	/// Creates a new numbered generator with no prefix,
	/// starting with the given `offset` number.
	///
	/// The returned generator can create `usize::MAX - offset` unique blank node identifiers
	/// before panicking.
	pub fn new_with_offset(offset: usize) -> Self {
		Self::new_full(String::new(), offset)
	}

	/// Creates a new numbered generator with the given prefix.
	pub fn new_with_prefix(prefix: String) -> Self {
		Self::new_full(prefix, 0)
	}

	/// Creates a new numbered generator with the given prefix,
	/// starting with the given `offset` number.
	///
	/// The returned generator can create `usize::MAX - offset` unique blank node identifiers
	/// before panicking.
	pub fn new_full(prefix: String, offset: usize) -> Self {
		Self {
			prefix,
			count: offset,
		}
	}

	/// Returns the prefix of this generator.
	pub fn prefix(&self) -> &str {
		&self.prefix
	}

	/// Returns the number of already generated identifiers.
	pub fn count(&self) -> usize {
		self.count
	}

	pub fn next_blank_id(&mut self) -> BlankIdBuf {
		let id = unsafe { BlankIdBuf::new_unchecked(format!("_:{}{}", self.prefix, self.count)) };
		self.count += 1;
		id
	}
}

impl LocalGenerator for Blank {
	fn next_local_term(&mut self) -> LocalTerm {
		LocalTerm::Anonymous(self.next_blank_id())
	}
}
