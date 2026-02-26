use crate::{BlankIdBuf, Generator, Id};

/// Generates numbered blank node identifiers,
/// with an optional prefix.
///
/// This generator can create `usize::MAX` unique blank node identifiers.
/// If [`Self::next_id`] will wrap around if more identifiers are created.
#[derive(Default)]
pub struct BlankIdGenerator {
	/// Prefix string.
	prefix: String,

	/// Number of already generated identifiers.
	count: usize,
}

impl BlankIdGenerator {
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

	/// Generates the next blank id.
	///
	/// Wraps around if more than `usize::MAX` identifiers are generated.
	pub fn next_blank_id(&mut self) -> BlankIdBuf {
		let id = unsafe { BlankIdBuf::new_unchecked(format!("_:{}{}", self.prefix, self.count)) };
		self.count = self.count.wrapping_add(1);
		id
	}
}

impl Generator for BlankIdGenerator {
	fn next_id(&mut self) -> Id {
		Id::BlankId(self.next_blank_id())
	}
}
