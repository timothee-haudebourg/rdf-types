use std::fmt;

use langtag::{LangTag, LangTagBuf};

/// Display method for RDF syntax elements.
pub trait RdfDisplay {
	/// Formats the value using the given formatter.
	fn rdf_fmt(&self, f: &mut fmt::Formatter) -> fmt::Result;

	/// Prepare the value to be formatted as an RDF syntax element.
	#[inline(always)]
	fn rdf_display(&self) -> RdfDisplayed<&Self> {
		RdfDisplayed(self)
	}
}

impl RdfDisplay for str {
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

impl RdfDisplay for String {
	fn rdf_fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		self.as_str().rdf_fmt(f)
	}
}

impl RdfDisplay for iref::IriRef {
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

impl RdfDisplay for iref::Iri {
	#[inline(always)]
	fn rdf_fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		self.as_iri_ref().rdf_fmt(f)
	}
}

impl RdfDisplay for iref::IriBuf {
	#[inline(always)]
	fn rdf_fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		self.as_iri_ref().rdf_fmt(f)
	}
}

impl RdfDisplay for iref::IriRefBuf {
	#[inline(always)]
	fn rdf_fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		self.as_iri_ref().rdf_fmt(f)
	}
}

impl RdfDisplay for LangTag {
	fn rdf_fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		use fmt::Display;
		self.as_str().fmt(f)
	}
}

impl RdfDisplay for LangTagBuf {
	fn rdf_fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		use fmt::Display;
		self.as_str().fmt(f)
	}
}

impl<T: RdfDisplay + ?Sized> RdfDisplay for &T {
	#[inline(always)]
	fn rdf_fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		T::rdf_fmt(*self, f)
	}
}

/// Value ready to be formatted as an RDF syntax element.
pub struct RdfDisplayed<T>(T);

impl<T: RdfDisplay> fmt::Display for RdfDisplayed<T> {
	#[inline(always)]
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		self.0.rdf_fmt(f)
	}
}
