use std::borrow::{Borrow, ToOwned};
use std::fmt;
use std::ops::Deref;
use std::str::FromStr;

/// Invalid blank node identifier.
///
/// This error is raised by the [`BlankId::new`] and [`BlankIdBuf::new`] functions
/// when the input string is not a valid blank node identifier.
#[derive(Debug)]
pub struct InvalidBlankId<T>(T);

/// Blank node identifier.
///
/// A blank node identifier is a string matching
/// the `BLANK_NODE_LABEL` production in the following [EBNF](http://www.w3.org/TR/REC-xml/#sec-notation) grammar:
///
/// ```ebnf
/// [141s] BLANK_NODE_LABEL ::= '_:' (PN_CHARS_U | [0-9]) ((PN_CHARS | '.')* PN_CHARS)?
/// [157s] PN_CHARS_BASE    ::= [A-Z] | [a-z] | [#x00C0-#x00D6] | [#x00D8-#x00F6] | [#x00F8-#x02FF] | [#x0370-#x037D] | [#x037F-#x1FFF] | [#x200C-#x200D] | [#x2070-#x218F] | [#x2C00-#x2FEF] | [#x3001-#xD7FF] | [#xF900-#xFDCF] | [#xFDF0-#xFFFD] | [#x10000-#xEFFFF]
/// [158s] PN_CHARS_U       ::= PN_CHARS_BASE | '_' | ':'
/// [160s] PN_CHARS         ::= PN_CHARS_U | '-' | [0-9] | #x00B7 | [#x0300-#x036F] | [#x203F-#x2040]
/// ```
#[derive(PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BlankId(str);

impl BlankId {
	/// Parses a blank node identifier.
	#[inline(always)]
	pub fn new(s: &str) -> Result<&Self, InvalidBlankId<&str>> {
		if check(s.chars()) {
			Ok(unsafe { Self::new_unchecked(s) })
		} else {
			Err(InvalidBlankId(s))
		}
	}

	/// Creates a new blank node identifier from `s` without checking it.
	///
	/// # Safety
	///
	/// The input string `s` must be a valid blank node identifier.
	#[inline(always)]
	pub unsafe fn new_unchecked(s: &str) -> &Self {
		std::mem::transmute(s)
	}

	/// Returns a reference to the underlying string defining the blank node identifier.
	#[inline(always)]
	pub fn as_str(&self) -> &str {
		&self.0
	}

	/// Returns the suffix part (after `_:`) of the blank node identifier.
	#[inline(always)]
	pub fn suffix(&self) -> &str {
		&self.0[2..]
	}
}

impl Deref for BlankId {
	type Target = str;

	#[inline(always)]
	fn deref(&self) -> &str {
		self.as_str()
	}
}

impl ToOwned for BlankId {
	type Owned = BlankIdBuf;

	#[inline(always)]
	fn to_owned(&self) -> BlankIdBuf {
		unsafe { BlankIdBuf::new_unchecked(self.as_str().to_owned()) }
	}
}

impl fmt::Display for BlankId {
	#[inline(always)]
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		self.0.fmt(f)
	}
}

impl fmt::Debug for BlankId {
	#[inline(always)]
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		self.0.fmt(f)
	}
}

/// Owned blank node identifier.
///
/// A blank node identifier is a string matching
/// the `BLANK_NODE_LABEL` production in the following [EBNF](http://www.w3.org/TR/REC-xml/#sec-notation) grammar:
///
/// ```ebnf
/// [141s] BLANK_NODE_LABEL ::= '_:' (PN_CHARS_U | [0-9]) ((PN_CHARS | '.')* PN_CHARS)?
/// [157s] PN_CHARS_BASE    ::= [A-Z] | [a-z] | [#x00C0-#x00D6] | [#x00D8-#x00F6] | [#x00F8-#x02FF] | [#x0370-#x037D] | [#x037F-#x1FFF] | [#x200C-#x200D] | [#x2070-#x218F] | [#x2C00-#x2FEF] | [#x3001-#xD7FF] | [#xF900-#xFDCF] | [#xFDF0-#xFFFD] | [#x10000-#xEFFFF]
/// [158s] PN_CHARS_U       ::= PN_CHARS_BASE | '_' | ':'
/// [160s] PN_CHARS         ::= PN_CHARS_U | '-' | [0-9] | #x00B7 | [#x0300-#x036F] | [#x203F-#x2040]
/// ```
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BlankIdBuf(String);

impl BlankIdBuf {
	/// Parses a blank node identifier.
	#[inline(always)]
	pub fn new(s: String) -> Result<Self, InvalidBlankId<String>> {
		if check(s.chars()) {
			Ok(unsafe { Self::new_unchecked(s) })
		} else {
			Err(InvalidBlankId(s))
		}
	}

	/// Creates a new blank node identifier from `s` without checking it.
	///
	/// # Safety
	///
	/// The input string `s` must be a valid blank node identifier.
	#[inline(always)]
	pub unsafe fn new_unchecked(s: String) -> Self {
		std::mem::transmute(s)
	}

	/// Creates a blank node identifier using the given `u8` as suffix.
	#[inline(always)]
	pub fn from_u8(i: u8) -> Self {
		unsafe { Self::new_unchecked(format!("_:{}", i)) }
	}

	/// Creates a blank node identifier using the given `u16` as suffix.
	#[inline(always)]
	pub fn from_u16(i: u16) -> Self {
		unsafe { Self::new_unchecked(format!("_:{}", i)) }
	}

	/// Creates a blank node identifier using the given `u32` as suffix.
	#[inline(always)]
	pub fn from_u32(i: u32) -> Self {
		unsafe { Self::new_unchecked(format!("_:{}", i)) }
	}

	/// Creates a blank node identifier using the given `u64` as suffix.
	#[inline(always)]
	pub fn from_u64(i: u64) -> Self {
		unsafe { Self::new_unchecked(format!("_:{}", i)) }
	}

	/// Creates a blank node identifier using the given suffix.
	#[inline(always)]
	pub fn from_suffix(suffix: &str) -> Result<Self, InvalidBlankId<String>> {
		Self::new(format!("_:{}", suffix))
	}
}

impl FromStr for BlankIdBuf {
	type Err = InvalidBlankId<String>;

	fn from_str(s: &str) -> Result<Self, InvalidBlankId<String>> {
		Self::new(s.to_owned())
	}
}

impl Deref for BlankIdBuf {
	type Target = BlankId;

	#[inline(always)]
	fn deref(&self) -> &BlankId {
		unsafe { BlankId::new_unchecked(&self.0) }
	}
}

impl AsRef<BlankId> for BlankIdBuf {
	#[inline(always)]
	fn as_ref(&self) -> &BlankId {
		unsafe { BlankId::new_unchecked(&self.0) }
	}
}

impl Borrow<BlankId> for BlankIdBuf {
	#[inline(always)]
	fn borrow(&self) -> &BlankId {
		unsafe { BlankId::new_unchecked(&self.0) }
	}
}

impl Borrow<BlankId> for &BlankIdBuf {
	#[inline(always)]
	fn borrow(&self) -> &BlankId {
		unsafe { BlankId::new_unchecked(&self.0) }
	}
}

impl<'a> From<&'a BlankIdBuf> for &'a BlankId {
	#[inline(always)]
	fn from(b: &'a BlankIdBuf) -> Self {
		b.as_ref()
	}
}

impl fmt::Display for BlankIdBuf {
	#[inline(always)]
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		self.0.fmt(f)
	}
}

impl fmt::Debug for BlankIdBuf {
	#[inline(always)]
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		self.0.fmt(f)
	}
}

impl PartialEq<BlankId> for BlankIdBuf {
	fn eq(&self, other: &BlankId) -> bool {
		self.as_ref() == other
	}
}

impl<'a> PartialEq<&'a BlankId> for BlankIdBuf {
	fn eq(&self, other: &&'a BlankId) -> bool {
		self.as_ref() == *other
	}
}

impl PartialEq<BlankIdBuf> for BlankId {
	fn eq(&self, other: &BlankIdBuf) -> bool {
		self == other.as_ref()
	}
}

fn check<C: Iterator<Item = char>>(mut chars: C) -> bool {
	match chars.next() {
		Some('_') => match chars.next() {
			Some(':') => match chars.next() {
				Some(c) if c.is_ascii_digit() || is_pn_char_u(c) => {
					for c in chars {
						if !is_pn_char(c) {
							return false;
						}
					}

					true
				}
				_ => false,
			},
			_ => false,
		},
		_ => false,
	}
}

fn is_pn_char_base(c: char) -> bool {
	matches!(c, 'A'..='Z' | 'a'..='z' | '\u{00c0}'..='\u{00d6}' | '\u{00d8}'..='\u{00f6}' | '\u{00f8}'..='\u{02ff}' | '\u{0370}'..='\u{037d}' | '\u{037f}'..='\u{1fff}' | '\u{200c}'..='\u{200d}' | '\u{2070}'..='\u{218f}' | '\u{2c00}'..='\u{2fef}' | '\u{3001}'..='\u{d7ff}' | '\u{f900}'..='\u{fdcf}' | '\u{fdf0}'..='\u{fffd}' | '\u{10000}'..='\u{effff}')
}

fn is_pn_char_u(c: char) -> bool {
	is_pn_char_base(c) || matches!(c, '_' | ':')
}

fn is_pn_char(c: char) -> bool {
	is_pn_char_u(c)
		|| matches!(c, '-' | '0'..='9' | '\u{00b7}' | '\u{0300}'..='\u{036f}' | '\u{203f}'..='\u{2040}')
}
