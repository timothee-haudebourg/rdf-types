use crate::{BlankIdBuf, BlankIdVocabularyMut, Subject};

#[cfg(feature = "meta")]
use locspan::Meta;

/// Subject identifier generator.
pub trait Generator<T, B, N> {
	/// Generates the next fresh identifier in the given vocabulary.
	fn next(&mut self, vocabulary: &mut N) -> Subject<T, B>;

	#[cfg(feature = "meta")]
	/// Generates identifiers annotated with the given metadata.
	fn with_metadata<M>(self, metadata: M) -> WithMetadata<Self, M>
	where
		Self: Sized,
	{
		WithMetadata {
			metadata,
			generator: self,
		}
	}

	#[cfg(feature = "meta")]
	/// Generates identifiers annotated with the default value of type `M`.
	fn with_default_metadata<M: Default>(self) -> WithMetadata<Self, M>
	where
		Self: Sized,
	{
		WithMetadata {
			metadata: M::default(),
			generator: self,
		}
	}
}

impl<'a, T, B, N, G: Generator<T, B, N>> Generator<T, B, N> for &'a mut G {
	fn next(&mut self, vocabulary: &mut N) -> Subject<T, B> {
		(*self).next(vocabulary)
	}
}

#[cfg(feature = "meta")]
/// Subject identifier generator, with metadata.
pub trait MetaGenerator<I, B, N, M> {
	fn next(&mut self, vocabulary: &mut N) -> Meta<Subject<I, B>, M>;
}

#[cfg(feature = "meta")]
impl<'a, T, B, N, M, G: MetaGenerator<T, B, N, M>> MetaGenerator<T, B, N, M> for &'a mut G {
	fn next(&mut self, vocabulary: &mut N) -> Meta<Subject<T, B>, M> {
		(*self).next(vocabulary)
	}
}

#[cfg(feature = "meta")]
pub struct WithMetadata<G, M> {
	metadata: M,
	generator: G,
}

#[cfg(feature = "meta")]
impl<G, M> WithMetadata<G, M> {
	pub fn metadata(&self) -> &M {
		&self.metadata
	}

	pub fn generator(&self) -> &G {
		&self.generator
	}
}

#[cfg(feature = "meta")]
impl<I, B, N, G: Generator<I, B, N>, M> Generator<I, B, N> for WithMetadata<G, M> {
	fn next(&mut self, vocabulary: &mut N) -> Subject<I, B> {
		self.generator.next(vocabulary)
	}
}

#[cfg(feature = "meta")]
impl<I, B, N, G: Generator<I, B, N>, M: Clone> MetaGenerator<I, B, N, M> for WithMetadata<G, M> {
	fn next(&mut self, vocabulary: &mut N) -> Meta<Subject<I, B>, M> {
		Meta(self.generator.next(vocabulary), self.metadata.clone())
	}
}

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

impl<T, B, N: BlankIdVocabularyMut<BlankId = B>> Generator<T, B, N> for Blank {
	fn next(&mut self, vocabulary: &mut N) -> Subject<T, B> {
		Subject::Blank(vocabulary.insert_blank_id(&self.next_blank_id()))
	}
}

/// Generates UUID blank node identifiers based on the [`uuid`](https://crates.io/crates/uuid) crate.
///
/// This is an enum type with different UUID versions supported
/// by the `uuid` library, so you can choose which kind of UUID
/// better fits your application.
/// Version 1 is not supported.
///
/// You need to enable the `uuid-generator` feature to
/// use this type.
/// You also need to enable the features of each version you need
/// in the `uuid` crate.
pub enum Uuid {
	/// UUIDv3.
	///
	/// You must provide a vocabulary UUID and a name.
	/// See [uuid::Uuid::new_v3] for more information.
	#[cfg(feature = "uuid-generator-v3")]
	V3(uuid::Uuid, String),

	/// UUIDv4.
	///
	/// See [uuid::Uuid::new_v4] for more information.
	#[cfg(feature = "uuid-generator-v4")]
	V4,

	/// UUIDv5.
	///
	/// You must provide a vocabulary UUID and a name.
	/// See [uuid::Uuid::new_v5] for more information.
	#[cfg(feature = "uuid-generator-v5")]
	V5(uuid::Uuid, String),
}

#[cfg(any(
	feature = "uuid-generator-v3",
	feature = "uuid-generator-v4",
	feature = "uuid-generator-v5"
))]
impl Uuid {
	pub fn next_uuid(&self) -> uuid::Uuid {
		match self {
			#[cfg(feature = "uuid-generator-v3")]
			Self::V3(vocabulary, name) => uuid::Uuid::new_v3(vocabulary, name.as_bytes()),
			#[cfg(feature = "uuid-generator-v4")]
			Self::V4 => uuid::Uuid::new_v4(),
			#[cfg(feature = "uuid-generator-v5")]
			Self::V5(vocabulary, name) => uuid::Uuid::new_v5(vocabulary, name.as_bytes()),
		}
	}
}

#[cfg(any(
	feature = "uuid-generator-v3",
	feature = "uuid-generator-v4",
	feature = "uuid-generator-v5"
))]
impl<T, B, N: crate::IriVocabularyMut<Iri = T>> Generator<T, B, N> for Uuid {
	fn next(&mut self, vocabulary: &mut N) -> Subject<T, B> {
		unsafe {
			let mut buffer = Vec::with_capacity(uuid::adapter::Urn::LENGTH);
			let ptr = buffer.as_mut_ptr();
			let capacity = buffer.capacity();
			std::mem::forget(buffer);
			let uuid = self.next_uuid();
			let len = uuid
				.to_urn()
				.encode_lower(std::slice::from_raw_parts_mut(
					ptr,
					uuid::adapter::Urn::LENGTH,
				))
				.len();
			let buffer = Vec::from_raw_parts(ptr, len, capacity);
			let p = iref::parsing::ParsedIriRef::new(&buffer).unwrap();
			let iri = iref::IriBuf::from_raw_parts(buffer, p);
			Subject::Iri(vocabulary.insert(iri.as_iri()))
		}
	}
}

#[cfg(any(
	feature = "uuid-generator-v3",
	feature = "uuid-generator-v4",
	feature = "uuid-generator-v5"
))]
#[cfg(test)]
mod tests {
	use super::*;

	#[cfg(feature = "uuid-generator-v3")]
	#[test]
	fn uuidv3_iri() {
		let mut uuid_gen = Uuid::V3(
			uuid::Uuid::parse_str("936DA01F9ABD4d9d80C702AF85C822A8").unwrap(),
			"test".to_string(),
		);
		for _ in 0..100 {
			let reference: Subject = uuid_gen.next(&mut ());
			assert!(iref::IriBuf::new(reference.as_str()).is_ok())
		}
	}

	#[cfg(feature = "uuid-generator-v4")]
	#[test]
	fn uuidv4_iri() {
		let mut uuid_gen = Uuid::V4;
		for _ in 0..100 {
			let reference: Subject = uuid_gen.next(&mut ());
			assert!(iref::IriBuf::new(reference.as_str()).is_ok())
		}
	}

	#[cfg(feature = "uuid-generator-v5")]
	#[test]
	fn uuidv5_iri() {
		let mut uuid_gen = Uuid::V5(
			uuid::Uuid::parse_str("936DA01F9ABD4d9d80C702AF85C822A8").unwrap(),
			"test".to_string(),
		);
		for _ in 0..100 {
			let reference: Subject = uuid_gen.next(&mut ());
			assert!(iref::IriBuf::new(reference.as_str()).is_ok())
		}
	}
}
