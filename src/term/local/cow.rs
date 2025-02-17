use crate::{BlankId, CowTerm};
use std::borrow::Cow;

use super::LocalTerm;

pub enum CowLocalTerm<'a> {
	Anonymous(Cow<'a, BlankId>),

	Named(CowTerm<'a>),
}

impl From<LocalTerm> for CowLocalTerm<'_> {
	fn from(value: LocalTerm) -> Self {
		value.into_cow()
	}
}
