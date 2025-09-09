use crate::BlankId;
use std::borrow::Cow;

use super::{CowGroundTerm, Term};

pub enum CowLocalTerm<'a> {
	Anonymous(Cow<'a, BlankId>),

	Named(CowGroundTerm<'a>),
}

impl From<Term> for CowLocalTerm<'_> {
	fn from(value: Term) -> Self {
		value.into_cow()
	}
}
