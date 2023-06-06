use std::fmt;

use langtag::{LanguageTag, LanguageTagBuf};

pub trait RdfDisplay {
	fn rdf_fmt(&self, f: &mut fmt::Formatter) -> fmt::Result;

	#[inline(always)]
	fn rdf_display(&self) -> RdfDisplayed<&Self> {
		RdfDisplayed(self)
	}
}

impl RdfDisplay for String {
	fn rdf_fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		use fmt::Display;
		write!(f, "\"")?;

		for c in self.chars() {
			match c {
				'"' => write!(f, "\\\""),
				'\\' => write!(f, "\\\\"),
				'\n' => write!(f, "\\n"),
				'\r' => write!(f, "\\r"),
				c => c.fmt(f),
			}?
		}

		write!(f, "\"")
	}
}

impl<'a> RdfDisplay for iref::IriRef<'a> {
	fn rdf_fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "<")?;

		for c in self.as_str().chars() {
			match c {
				'\x00'..='\x20' | '<' | '>' | '"' | '{' | '}' | '|' | '^' | '`' | '\\' => {
					let bytes: u32 = c.into();
					write!(f, "\\u{bytes:#04x}")
				}
				_ => fmt::Display::fmt(&c, f),
			}?;
		}

		write!(f, ">")
	}
}

impl<'a> RdfDisplay for iref::Iri<'a> {
	#[inline(always)]
	fn rdf_fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		self.as_iri_ref().rdf_fmt(f)
	}
}

#[cfg(feature = "contextual")]
impl<'a, C: ?Sized> RdfDisplayWithContext<C> for iref::Iri<'a> {
	fn rdf_fmt_with(&self, _context: &C, f: &mut fmt::Formatter) -> fmt::Result {
		self.rdf_fmt(f)
	}
}

impl RdfDisplay for iref::IriBuf {
	#[inline(always)]
	fn rdf_fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		self.as_iri_ref().rdf_fmt(f)
	}
}

#[cfg(feature = "contextual")]
impl<C: ?Sized> RdfDisplayWithContext<C> for iref::IriBuf {
	fn rdf_fmt_with(&self, _context: &C, f: &mut fmt::Formatter) -> fmt::Result {
		self.rdf_fmt(f)
	}
}

impl RdfDisplay for iref::IriRefBuf {
	#[inline(always)]
	fn rdf_fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		self.as_iri_ref().rdf_fmt(f)
	}
}

#[cfg(feature = "contextual")]
impl<C: ?Sized> RdfDisplayWithContext<C> for iref::IriRefBuf {
	fn rdf_fmt_with(&self, _context: &C, f: &mut fmt::Formatter) -> fmt::Result {
		self.rdf_fmt(f)
	}
}

impl<'a> RdfDisplay for LanguageTag<'a> {
	fn rdf_fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		use fmt::Display;
		self.as_str().fmt(f)
	}
}

#[cfg(feature = "contextual")]
impl<'a, C: ?Sized> RdfDisplayWithContext<C> for LanguageTag<'a> {
	fn rdf_fmt_with(&self, _context: &C, f: &mut fmt::Formatter) -> fmt::Result {
		self.rdf_fmt(f)
	}
}

impl RdfDisplay for LanguageTagBuf {
	fn rdf_fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		use fmt::Display;
		self.as_str().fmt(f)
	}
}

#[cfg(feature = "contextual")]
impl<C: ?Sized> RdfDisplayWithContext<C> for LanguageTagBuf {
	fn rdf_fmt_with(&self, _context: &C, f: &mut fmt::Formatter) -> fmt::Result {
		self.rdf_fmt(f)
	}
}

impl<'a, T: RdfDisplay + ?Sized> RdfDisplay for &'a T {
	#[inline(always)]
	fn rdf_fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		T::rdf_fmt(*self, f)
	}
}

pub struct RdfDisplayed<T>(T);

impl<T: RdfDisplay> fmt::Display for RdfDisplayed<T> {
	#[inline(always)]
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
	#[inline(always)]
	fn rdf_fmt_with(&self, context: &C, f: &mut fmt::Formatter) -> fmt::Result {
		T::rdf_fmt_with(*self, context, f)
	}
}

#[cfg(feature = "contextual")]
impl<'c, T: RdfDisplayWithContext<C>, C: ?Sized> RdfDisplay for contextual::Contextual<T, &'c C> {
	#[inline(always)]
	fn rdf_fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		self.0.rdf_fmt_with(self.1, f)
	}
}

#[cfg(feature = "contextual")]
impl<'c, T: RdfDisplayWithContext<C>, C: ?Sized> RdfDisplay
	for contextual::Contextual<T, &'c mut C>
{
	#[inline(always)]
	fn rdf_fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		self.0.rdf_fmt_with(self.1, f)
	}
}
