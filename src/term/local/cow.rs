use crate::{BlankId, CowTerm};
use std::borrow::Cow;

use super::LocalTerm;

pub enum CowLocalTerm<'a> {
	Anonymous(Cow<'a, BlankId>),

	Named(CowTerm<'a>),
}

impl<'a> From<LocalTerm> for CowLocalTerm<'a> {
	fn from(value: LocalTerm) -> Self {
		value.into_cow()
	}
}
